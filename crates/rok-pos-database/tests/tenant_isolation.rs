//! Phase 0 spike 2: `in_business` hides another business's rows from the role
//! the app connects as, through row level security and even through raw SQL.

use rok_db::{Db, raw};
use rok_pos_database::{BusinessSession, in_business};
use sqlx::Row;
use uuid::Uuid;

/// The role the app connects as: it owns no tables and has no `BYPASSRLS`.
const APP_ROLE: &str = "rok_pos_app";
const APP_PASSWORD: &str = "rok_pos_app";

/// The same server and the same test database as `DATABASE_URL`, as the app
/// role: `#[rok_db::test]` gives the test a database of its own, so the name has
/// to be read back rather than guessed.
async fn app_db(db: &Db) -> Db {
    let database_name: String = raw("select current_database()")
        .scalar(db)
        .await
        .expect("the test database names itself");

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL names the server");
    let (scheme, rest) = url.split_once("://").expect("DATABASE_URL has a scheme");
    let (_, authority) = rest.split_once('@').expect("DATABASE_URL has a host");
    let (server, path) = authority.split_once('/').unwrap_or((authority, ""));
    let query = path
        .split_once('?')
        .map_or_else(String::new, |(_, query)| format!("?{query}"));
    let app_url = format!("{scheme}://{APP_ROLE}:{APP_PASSWORD}@{server}/{database_name}{query}");

    let app = Db::connect(&app_url)
        .await
        .expect("the app role can connect");

    let role: String = raw("select current_user")
        .scalar(&app)
        .await
        .expect("the app role can say who it is");
    assert_eq!(
        role, APP_ROLE,
        "the app connects as the app role, not the owner"
    );

    let privileges: (bool, bool) =
        sqlx::query_as("select rolsuper, rolbypassrls from pg_roles where rolname = $1")
            .bind(APP_ROLE)
            .fetch_one(&app)
            .await
            .expect("the role has a row in pg_roles");
    assert!(
        !privileges.0 && !privileges.1,
        "the app role is neither a superuser nor a bypasser, so the policies apply"
    );

    app
}

fn session_for(organization_id: Uuid) -> BusinessSession {
    BusinessSession::new(organization_id, Uuid::now_v7(), Uuid::now_v7(), "Grace N.")
}

/// One batch, written by the owner role before the app role looks.
async fn seed_batch(db: &Db, organization_id: Uuid, batch_number: &str, amount: i64) {
    raw(
        "insert into spike.batches (id, organization_id, batch_number, expiry_on, amount) \
         values (?, ?, ?, ?, ?)",
    )
    .bind(Uuid::now_v7())
    .bind(organization_id)
    .bind(batch_number)
    .bind(chrono::NaiveDate::from_ymd_opt(2027, 8, 31).unwrap())
    .bind(amount)
    .execute(db)
    .await
    .expect("the owner role can seed a batch");
}

#[rok_db::test(sql = "tests/app_role.sql", sql = "tests/tenant_schema.sql")]
async fn raw_sql_sees_only_the_sessions_own_business(db: Db) {
    let afya = Uuid::now_v7();
    let uzima = Uuid::now_v7();
    seed_batch(&db, afya, "AMS-2404", 58_800).await;
    seed_batch(&db, uzima, "AMS-9999", 999_999).await;

    let app = app_db(&db).await;
    let afya_batches: Vec<String> = in_business(&app, &session_for(afya), |transaction| {
        Box::pin(async move {
            let rows = sqlx::query("select batch_number from spike.batches")
                .fetch_all(&mut **transaction)
                .await?;
            Ok(rows
                .iter()
                .map(|row| row.get::<String, _>("batch_number"))
                .collect())
        })
    })
    .await
    .expect("the app role reads its own business");

    assert_eq!(
        afya_batches,
        vec!["AMS-2404".to_string()],
        "another business's row is invisible, even through raw SQL"
    );
}

#[rok_db::test(sql = "tests/app_role.sql", sql = "tests/tenant_schema.sql")]
async fn each_business_sees_only_its_own_rows(db: Db) {
    let afya = Uuid::now_v7();
    let uzima = Uuid::now_v7();
    seed_batch(&db, afya, "AMX-2409", 120_000).await;
    seed_batch(&db, uzima, "TRM-2411", 340_000).await;

    let app = app_db(&db).await;
    let total = |organization_id: Uuid| {
        let app = &app;
        async move {
            in_business(app, &session_for(organization_id), |transaction| {
                Box::pin(async move {
                    raw("select coalesce(sum(amount), 0) from spike.batches")
                        .scalar(&mut **transaction)
                        .await
                })
            })
            .await
            .expect("the app role reads its own business")
        }
    };

    let afya_total: rust_decimal::Decimal = total(afya).await;
    let uzima_total: rust_decimal::Decimal = total(uzima).await;

    assert_eq!(afya_total, rust_decimal::Decimal::new(120_000, 0));
    assert_eq!(uzima_total, rust_decimal::Decimal::new(340_000, 0));
}

#[rok_db::test(sql = "tests/app_role.sql", sql = "tests/tenant_schema.sql")]
async fn a_write_for_another_business_is_refused(db: Db) {
    let afya = Uuid::now_v7();
    let uzima = Uuid::now_v7();
    let app = app_db(&db).await;

    let written = in_business(&app, &session_for(afya), |transaction| {
        Box::pin(async move {
            raw(
                "insert into spike.batches (id, organization_id, batch_number, expiry_on, amount) \
                 values (?, ?, ?, ?, ?)",
            )
            .bind(Uuid::now_v7())
            .bind(uzima)
            .bind("AMS-0001")
            .bind(chrono::NaiveDate::from_ymd_opt(2027, 8, 31).unwrap())
            .bind(1)
            .execute(&mut **transaction)
            .await
        })
    })
    .await;

    assert!(
        written.is_err(),
        "a row for another business is refused, not written"
    );

    let stored: i64 = raw("select count(*) from spike.batches")
        .scalar(&db)
        .await
        .expect("the owner role can count");
    assert_eq!(stored, 0, "nothing was written into another business");
}

#[rok_db::test(sql = "tests/app_role.sql", sql = "tests/tenant_schema.sql")]
async fn a_session_without_the_setting_sees_nothing(db: Db) {
    let afya = Uuid::now_v7();
    seed_batch(&db, afya, "AMX-2409", 120_000).await;

    let app = app_db(&db).await;
    let rows = sqlx::query("select batch_number from spike.batches")
        .fetch_all(&app)
        .await
        .expect("the query runs");

    assert!(
        rows.is_empty(),
        "without `in_business` there is no business to show, so there is nothing"
    );
}
