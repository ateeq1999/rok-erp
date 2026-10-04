//! Phase 1: row level security on every table migration 0002 adds, checked
//! as `rok_pos_app`, the role the app connects as.
//!
//! The owner role creates the tables and the policy; only the policy, not the
//! owner, can be trusted to hide another business's rows. So the test hands
//! the tables to the app role, connects as it, and asks each of the fifteen
//! two questions: is row level security on, and does it hide what it should?

use rok_db::{Db, raw};
use rok_pos_database::{
    BusinessSession, ModuleInstaller, default_modules_directory, in_business, load_afya_story,
};
use uuid::Uuid;

/// The role the app connects as: it owns no tables and has no `BYPASSRLS`.
const APP_ROLE: &str = "rok_pos_app";
const APP_PASSWORD: &str = "rok_pos_app";

/// The story's business and a business that is not in the story.
const AFYA: &str = "00000000-0000-7000-8000-000000000001";
const ELSEWHERE: &str = "00000000-0000-7000-8000-0000000000ff";

/// Every table migration 0002 adds, in the order the migration creates them.
const NEW_TABLES: [&str; 15] = [
    "medicine_details",
    "medicine_substitutes",
    "patient_clinical_profiles",
    "clinical_notes",
    "interaction_rules",
    "prescription_checks",
    "prescriber_contacts",
    "dispensing_labels",
    "refill_schedules",
    "temperature_logs",
    "recall_notices",
    "recall_actions",
    "licence_documents",
    "receipt_quality_checks",
    "insurance_claim_batches",
];

/// Tables the story seed fills, so reading nothing from them in the story's
/// business would itself be a bug.
const SEEDED_TABLES: [&str; 7] = [
    "medicine_details",
    "patient_clinical_profiles",
    "interaction_rules",
    "prescription_checks",
    "recall_notices",
    "recall_actions",
    "insurance_claim_batches",
];

/// The same server and the same test database as `DATABASE_URL`, as the app
/// role: `#[rok_db::test]` gives the test a database of its own, so the name
/// has to be read back rather than guessed.
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

    app
}

/// Install the modules, load the story, then hand every new table to the app
/// role the way the production grant will.
async fn story_with_grants(db: &Db) {
    let installer =
        ModuleInstaller::load(&default_modules_directory()).expect("the manifests load");
    installer.install(db).await.expect("the modules install");
    load_afya_story(db).await.expect("the seed loads");

    raw("grant usage on schema pharmacy to rok_pos_app")
        .execute(db)
        .await
        .expect("the app role may name the schema");
    for table in NEW_TABLES {
        raw(format!(
            "grant select, insert, update, delete on pharmacy.{table} to rok_pos_app"
        ))
        .execute(db)
        .await
        .expect("the app role may work the table");
    }
}

/// Every one of the fifteen tables has row level security enabled and the
/// `organization_isolation` policy on it, which is what makes the app role's
/// reads safe in the first place.
#[rok_db::test]
async fn every_new_table_is_isolated(db: Db) {
    story_with_grants(&db).await;
    let app = app_db(&db).await;

    for table in NEW_TABLES {
        let (rls, policies): (bool, i64) = sqlx::query_as(
            "select c.relrowsecurity as rls, \
                    (select count(*) from pg_policies \
                      where schemaname = 'pharmacy' and tablename = $1 \
                        and policyname = 'organization_isolation') \
             from pg_class c \
             join pg_namespace n on n.oid = c.relnamespace \
             where n.nspname = 'pharmacy' and c.relname = $1",
        )
        .bind(table)
        .fetch_one(&app)
        .await
        .expect("the table has a row in pg_class");
        assert!(rls, "pharmacy.{table} has row level security enabled");
        assert_eq!(
            policies, 1,
            "pharmacy.{table} carries the organization_isolation policy"
        );
    }
}

/// The app role reads the story's rows in the story's business and nothing at
/// all in any other, table by table.
#[rok_db::test]
async fn every_new_table_hides_the_other_business(db: Db) {
    story_with_grants(&db).await;
    let app = app_db(&db).await;

    let afya = Uuid::parse_str(AFYA).expect("the story's business");
    let elsewhere = Uuid::parse_str(ELSEWHERE).expect("another business");
    let afya_session = BusinessSession::new(afya, Uuid::now_v7(), Uuid::now_v7(), "Grace N.");
    let elsewhere_session =
        BusinessSession::new(elsewhere, Uuid::now_v7(), Uuid::now_v7(), "Someone else");

    for table in NEW_TABLES {
        let count = |session: BusinessSession| {
            let app = &app;
            async move {
                let table = table;
                in_business(app, &session, |transaction| {
                    Box::pin(async move {
                        raw(format!("select count(*) from pharmacy.{table}"))
                            .scalar(&mut **transaction)
                            .await
                    })
                })
                .await
                .expect("the app role reads the table")
            }
        };

        let in_afya: i64 = count(afya_session.clone()).await;
        let in_elsewhere: i64 = count(elsewhere_session.clone()).await;

        if SEEDED_TABLES.contains(&table) {
            assert!(
                in_afya > 0,
                "pharmacy.{table} shows the story's business its own rows"
            );
        }
        assert_eq!(
            in_elsewhere, 0,
            "pharmacy.{table} shows another business nothing, even though the \
             story's rows are right there"
        );
    }
}

/// A write for another business is refused by the policy, not by the caller:
/// the row never reaches the table.
#[rok_db::test]
async fn a_write_for_another_business_is_refused_on_the_new_tables(db: Db) {
    story_with_grants(&db).await;
    let app = app_db(&db).await;

    let elsewhere = Uuid::parse_str(ELSEWHERE).expect("another business");

    // The session works as the story's business, but the row claims to belong
    // to another one: the policy's `with check` has to refuse it.
    let afya = Uuid::parse_str(AFYA).expect("the story's business");
    let session = BusinessSession::new(afya, Uuid::now_v7(), Uuid::now_v7(), "Grace N.");
    let branch = Uuid::parse_str("00000000-0000-7000-8000-000000000002").expect("Mwenge");
    let grace = Uuid::parse_str("00000000-0000-7000-8000-000000000011").expect("Grace N.");

    let written = in_business(&app, &session, |transaction| {
        Box::pin(async move {
            raw("insert into pharmacy.temperature_logs \
                   (id, organization_id, branch_id, storage_unit_name, recorded_at, \
                    temperature_celsius, is_in_range, recorded_by_user_id) \
                 values (?, ?, ?, 'Fridge 1', now(), 4.50, true, ?)")
            .bind(Uuid::now_v7())
            .bind(elsewhere)
            .bind(branch)
            .bind(grace)
            .execute(&mut **transaction)
            .await
        })
    })
    .await;

    assert!(
        written.is_err(),
        "a temperature log for another business is refused, not written"
    );

    let stored: i64 = raw("select count(*) from pharmacy.temperature_logs")
        .scalar(&db)
        .await
        .expect("the owner role can count");
    assert_eq!(stored, 0, "nothing was written into another business");
}
