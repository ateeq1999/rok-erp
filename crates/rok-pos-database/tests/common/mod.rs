//! Shared test support: the unprivileged role the app connects as, a database with
//! the pharmacy schema installed, and the rows a test needs before it can write.

use std::path::PathBuf;

use rok_db::{Db, raw};
use rok_pos_database::{BusinessSession, InstallRequest, install};
use uuid::Uuid;

/// The role the app connects as: it owns no tables and has no `BYPASSRLS`.
pub const APP_ROLE: &str = "rok_pos_app";
pub const APP_PASSWORD: &str = "rok_pos_app";

/// The module folder in this repository.
#[must_use]
pub fn modules_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../database/modules")
        .canonicalize()
        .expect("the copied module folder is in the repository")
}

/// The same server and the same test database as `DATABASE_URL`, as the app role.
/// `#[rok_db::test]` gives the test a database of its own, so the name has to be
/// read back rather than guessed.
pub async fn app_db(db: &Db) -> Db {
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

/// A session in `organization_id`, named after whoever is signed in.
#[must_use]
pub fn session_for(organization_id: Uuid) -> BusinessSession {
    BusinessSession::new(organization_id, Uuid::now_v7(), Uuid::now_v7(), "Grace N.")
}

/// Install pharmacy and everything it depends on, without job rows.
pub async fn install_pharmacy_stack(db: &Db) {
    install(
        db,
        &InstallRequest::all(modules_root())
            .only(&["pharmacy"])
            .without_jobs(),
    )
    .await
    .expect("the pharmacy stack installs");
}

/// Give the app role what it needs to read and write the pharmacy tables. A
/// deployment would do this once per environment with the role it ships; here the
/// test does it, so the roles the installer expects are still the test's to set up.
pub async fn grant_pharmacy_to_app_role(db: &Db) {
    for schema in [
        "core",
        "catalog",
        "inventory",
        "customers",
        "point_of_sale",
        "purchasing",
        "pharmacy",
    ] {
        db.execute(
            format!(
                "grant usage on schema {schema} to {APP_ROLE}; \
             grant select, insert, update, delete on all tables in schema {schema} to {APP_ROLE};"
            )
            .as_str(),
        )
        .await
        .expect("the app role can read and write the schema");
    }
}

/// What a test seeded for one business: the organization, its branch and its user.
// The fields are named after the columns they fill, which all end in `_id`.
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Copy)]
pub struct Business {
    pub organization_id: Uuid,
    pub branch_id: Uuid,
    pub user_id: Uuid,
}

/// Insert an organization, a branch and a user, and return their ids.
pub async fn seed_business(db: &Db, name: &str) -> Business {
    let organization_id: Uuid =
        raw("insert into core.organizations (name) values (?) returning id")
            .bind(name)
            .scalar(db)
            .await
            .expect("the owner role can create an organization");

    let branch_id: Uuid = raw(
        "insert into core.branches (organization_id, name, code) values (?, ?, ?) returning id",
    )
    .bind(organization_id)
    .bind(format!("{name} branch"))
    .bind(format!("{}-001", name.to_lowercase()))
    .scalar(db)
    .await
    .expect("the owner role can create a branch");

    let user_id: Uuid = raw(
        "insert into core.users (organization_id, full_name, is_owner) values (?, ?, true) returning id",
    )
    .bind(organization_id)
    .bind(format!("{name} owner"))
    .scalar(db)
    .await
    .expect("the owner role can create a user");

    Business {
        organization_id,
        branch_id,
        user_id,
    }
}

/// A stocked product, the anchor most pharmacy rows point at.
pub async fn seed_product(db: &Db, organization_id: Uuid, name: &str) -> Uuid {
    raw(
        "insert into catalog.products (organization_id, name, product_type, tracks_stock) \
         values (?, ?, 'stocked', true) returning id",
    )
    .bind(organization_id)
    .bind(name)
    .scalar(db)
    .await
    .expect("the owner role can create a product")
}

/// A customer, the anchor for the clinical tables.
pub async fn seed_customer(db: &Db, organization_id: Uuid, name: &str) -> Uuid {
    raw("insert into customers.customers (organization_id, full_name) values (?, ?) returning id")
        .bind(organization_id)
        .bind(name)
        .scalar(db)
        .await
        .expect("the owner role can create a customer")
}

/// A prescription and one dispensed item on it.
pub async fn seed_prescription(
    db: &Db,
    business: &Business,
    customer_id: Uuid,
    product_id: Uuid,
) -> (Uuid, Uuid) {
    let prescriber_id: Uuid =
        raw("insert into pharmacy.prescribers (organization_id, name) values (?, ?) returning id")
            .bind(business.organization_id)
            .bind("Dr A. Mwakalinga")
            .scalar(db)
            .await
            .expect("the owner role can create a prescriber");

    let prescription_id: Uuid = raw("insert into pharmacy.prescriptions \
             (organization_id, branch_id, customer_id, prescriber_id, written_on) \
         values (?, ?, ?, ?, current_date) returning id")
    .bind(business.organization_id)
    .bind(business.branch_id)
    .bind(customer_id)
    .bind(prescriber_id)
    .scalar(db)
    .await
    .expect("the owner role can create a prescription");

    let prescription_item_id: Uuid = raw("insert into pharmacy.prescription_items \
             (organization_id, prescription_id, product_id, quantity_prescribed) \
         values (?, ?, ?, 30) returning id")
    .bind(business.organization_id)
    .bind(prescription_id)
    .bind(product_id)
    .scalar(db)
    .await
    .expect("the owner role can create a prescription item");

    (prescription_id, prescription_item_id)
}

/// An insurance provider, the anchor for a claim batch.
pub async fn seed_insurance_provider(db: &Db, organization_id: Uuid, name: &str) -> Uuid {
    raw(
        "insert into pharmacy.insurance_providers (organization_id, name) values (?, ?) returning id",
    )
    .bind(organization_id)
    .bind(name)
    .scalar(db)
    .await
    .expect("the owner role can create an insurance provider")
}

/// A received delivery line, the anchor for a receipt quality check.
pub async fn seed_goods_receipt_line(db: &Db, organization_id: Uuid, product_id: Uuid) -> Uuid {
    let supplier_id: Uuid =
        raw("insert into purchasing.suppliers (organization_id, name) values (?, ?) returning id")
            .bind(organization_id)
            .bind("Medipharm Distributors")
            .scalar(db)
            .await
            .expect("the owner role can create a supplier");

    let stock_location_id: Uuid = raw(
        "insert into inventory.stock_locations (organization_id, name, code) \
         values (?, ?, 'MAIN') returning id",
    )
    .bind(organization_id)
    .bind("Main store")
    .scalar(db)
    .await
    .expect("the owner role can create a stock location");

    let goods_receipt_id: Uuid = raw("insert into purchasing.goods_receipts \
             (organization_id, goods_receipt_number, supplier_id, stock_location_id) \
         values (?, ?, ?, ?) returning id")
    .bind(organization_id)
    .bind(format!("GR-{}", &Uuid::now_v7().simple().to_string()[..8]))
    .bind(supplier_id)
    .bind(stock_location_id)
    .scalar(db)
    .await
    .expect("the owner role can create a goods receipt");

    raw("insert into purchasing.goods_receipt_lines \
             (organization_id, goods_receipt_id, product_id, quantity_received) \
         values (?, ?, ?, 100) returning id")
    .bind(organization_id)
    .bind(goods_receipt_id)
    .bind(product_id)
    .scalar(db)
    .await
    .expect("the owner role can create a goods receipt line")
}
