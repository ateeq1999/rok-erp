//! Phase 1: row level security on every table the pharmacy migration 0002 adds.
//!
//! The schema is not written by hand here. The installer builds it from
//! `database/modules`, so this test also proves the tables the app will query are
//! the ones the installer made, and that `core.prepare_table` really put a
//! tenant policy on each of them.

mod common;

use common::{
    Business, app_db, grant_pharmacy_to_app_role, install_pharmacy_stack, seed_business,
    seed_customer, seed_goods_receipt_line, seed_insurance_provider, seed_prescription,
    seed_product, session_for,
};
use rok_db::{Db, raw};
use rok_pos_database::in_business;
use sqlx::Row;
use uuid::Uuid;

/// Every table migration 0002 adds.
const NEW_TABLES: [&str; 15] = [
    "pharmacy.medicine_details",
    "pharmacy.medicine_substitutes",
    "pharmacy.patient_clinical_profiles",
    "pharmacy.clinical_notes",
    "pharmacy.interaction_rules",
    "pharmacy.prescription_checks",
    "pharmacy.prescriber_contacts",
    "pharmacy.dispensing_labels",
    "pharmacy.refill_schedules",
    "pharmacy.temperature_logs",
    "pharmacy.recall_notices",
    "pharmacy.recall_actions",
    "pharmacy.licence_documents",
    "pharmacy.receipt_quality_checks",
    "pharmacy.insurance_claim_batches",
];

/// Seed one statement through the owner role.
macro_rules! seed {
    ($db:expr, $sql:literal $(, $bind:expr)* $(,)?) => {
        raw($sql)
            $(.bind($bind))*
            .execute($db)
            .await
            .expect("the owner role can seed the row")
    };
}

/// One row in every new table, for one business.
// One insert per table, in the migration's order: splitting it would only
// scatter the list a reader checks against the migration.
#[allow(clippy::too_many_lines)]
async fn seed_every_new_table(db: &Db, business: &Business, suffix: &str) {
    let organization_id = business.organization_id;
    let product = seed_product(db, organization_id, &format!("Amoxicillin {suffix}")).await;
    let other = seed_product(db, organization_id, &format!("Paracetamol {suffix}")).await;
    let customer = seed_customer(db, organization_id, &format!("Patient {suffix}")).await;
    let (prescription_id, prescription_item_id) =
        seed_prescription(db, business, customer, product).await;
    let provider = seed_insurance_provider(db, organization_id, &format!("NHIF {suffix}")).await;
    let receipt_line = seed_goods_receipt_line(db, organization_id, product).await;

    seed!(
        db,
        "insert into pharmacy.medicine_details \
             (organization_id, product_id, generic_name, strength_text, dosage_form) \
         values (?, ?, 'Amoxicillin', '500 mg', 'capsule')",
        organization_id,
        product
    );
    seed!(
        db,
        "insert into pharmacy.medicine_substitutes \
             (organization_id, product_id, substitute_product_id, substitution_kind) \
         values (?, ?, ?, 'same_generic')",
        organization_id,
        product,
        other
    );
    seed!(
        db,
        "insert into pharmacy.patient_clinical_profiles \
             (organization_id, customer_id, allergies, reminder_consent, \
              reminder_consent_given_at) \
         values (?, ?, array['penicillin'], true, now())",
        organization_id,
        customer
    );
    seed!(
        db,
        "insert into pharmacy.clinical_notes \
             (organization_id, customer_id, note_text, written_by_user_id) \
         values (?, ?, 'Penicillin rash in 2019', ?)",
        organization_id,
        customer,
        business.user_id
    );
    seed!(
        db,
        "insert into pharmacy.interaction_rules \
             (organization_id, first_generic_name, second_generic_name, severity, \
              message_text, source_reference) \
         values (?, 'Amoxicillin', 'Warfarin', 'caution', 'Monitor the INR', \
                 'WHO Essential Medicines List')",
        organization_id
    );
    seed!(
        db,
        "insert into pharmacy.prescription_checks \
             (organization_id, prescription_id, check_key, result, checked_by_user_id) \
         values (?, ?, 'allergy', 'warning', ?)",
        organization_id,
        prescription_id,
        business.user_id
    );
    seed!(
        db,
        "insert into pharmacy.prescriber_contacts \
             (organization_id, prescription_id, contact_method, outcome, note_text, \
              recorded_by_user_id) \
         values (?, ?, 'phone', 'prescription_changed', 'Dose halved', ?)",
        organization_id,
        prescription_id,
        business.user_id
    );
    seed!(
        db,
        "insert into pharmacy.dispensing_labels \
             (organization_id, prescription_item_id, label_text, printed_by_user_id) \
         values (?, ?, 'Amoxicillin 500 mg, three times a day', ?)",
        organization_id,
        prescription_item_id,
        business.user_id
    );
    seed!(
        db,
        "insert into pharmacy.refill_schedules \
             (organization_id, customer_id, product_id, days_of_supply, reminder_status, \
              next_due_on) \
         values (?, ?, ?, 30, 'scheduled', current_date + 30)",
        organization_id,
        customer,
        product
    );
    seed!(
        db,
        "insert into pharmacy.temperature_logs \
             (organization_id, branch_id, storage_unit_name, temperature_celsius, is_in_range, \
              recorded_by_user_id) \
         values (?, ?, 'Fridge 1', 4.5, true, ?)",
        organization_id,
        business.branch_id,
        business.user_id
    );
    let recall_notice_id: Uuid = raw("insert into pharmacy.recall_notices \
             (organization_id, supplier_name, product_id, batch_number, reason_text, status, \
              recall_reference) \
         values (?, 'Medipharm Distributors', ?, 'B-2291', 'Failed stability test', 'quarantined', \
                 'RC-2024-0007') \
         returning id")
    .bind(organization_id)
    .bind(product)
    .scalar(db)
    .await
    .expect("the owner role can record a recall");
    seed!(
        db,
        "insert into pharmacy.recall_actions \
             (organization_id, recall_notice_id, action_type, customer_id, quantity, \
              done_by_user_id) \
         values (?, ?, 'quarantined', ?, 20, ?)",
        organization_id,
        recall_notice_id,
        customer,
        business.user_id
    );
    seed!(
        db,
        "insert into pharmacy.licence_documents \
             (organization_id, branch_id, licence_type, holder_name, licence_number, expires_on) \
         values (?, ?, 'premises', 'Afya Pharmacy Ltd', ?, current_date + 365)",
        organization_id,
        business.branch_id,
        format!("P-2024-{suffix}")
    );
    seed!(
        db,
        "insert into pharmacy.receipt_quality_checks \
             (organization_id, goods_receipt_line_id, check_key, result, \
              minimum_temperature_celsius, maximum_temperature_celsius) \
         values (?, ?, 'cold_chain', 'passed', 2.0, 8.0)",
        organization_id,
        receipt_line
    );
    seed!(
        db,
        "insert into pharmacy.insurance_claim_batches \
             (organization_id, insurance_provider_id, period_start_on, period_end_on, status, \
              claimed_amount) \
         values (?, ?, current_date - 30, current_date - 1, 'submitted', 450000.00)",
        organization_id,
        provider
    );
}

/// Count what the app role can see in one table, inside one business.
async fn visible_rows(app: &Db, organization_id: Uuid, table: &str) -> i64 {
    let sql = format!("select count(*) from {table}");
    in_business(app, &session_for(organization_id), move |transaction| {
        let sql = sql.clone();
        Box::pin(async move { raw(&sql).scalar(&mut **transaction).await })
    })
    .await
    .expect("the app role reads its own business")
}

/// Rows the app role changed outside its own business, which has to be none.
async fn rows_changed_outside(app: &Db, organization_id: Uuid, other: Uuid, table: &str) -> u64 {
    let sql = format!("update {table} set updated_at = now() where organization_id = ?");
    in_business(app, &session_for(organization_id), move |transaction| {
        let sql = sql.clone();
        Box::pin(async move { raw(&sql).bind(other).execute(&mut **transaction).await })
    })
    .await
    .expect("the app role runs the update")
}

#[rok_db::test]
async fn every_new_table_shows_a_business_only_its_own_rows(db: Db) {
    install_pharmacy_stack(&db).await;
    grant_pharmacy_to_app_role(&db).await;

    let afya = seed_business(&db, "Afya Pharmacy").await;
    let uzima = seed_business(&db, "Uzima Pharmacy").await;
    seed_every_new_table(&db, &afya, "A").await;
    seed_every_new_table(&db, &uzima, "B").await;

    let app = app_db(&db).await;
    for table in NEW_TABLES {
        let visible = visible_rows(&app, afya.organization_id, table).await;
        assert_eq!(
            visible, 1,
            "{table} shows one business's row and hides the other's"
        );
    }
}

#[rok_db::test]
async fn a_business_cannot_change_another_businesss_row_in_a_new_table(db: Db) {
    install_pharmacy_stack(&db).await;
    grant_pharmacy_to_app_role(&db).await;

    let afya = seed_business(&db, "Afya Pharmacy").await;
    let uzima = seed_business(&db, "Uzima Pharmacy").await;
    seed_every_new_table(&db, &afya, "A").await;
    seed_every_new_table(&db, &uzima, "B").await;

    let app = app_db(&db).await;
    for table in NEW_TABLES {
        let changed =
            rows_changed_outside(&app, afya.organization_id, uzima.organization_id, table).await;
        assert_eq!(
            changed, 0,
            "{table} refuses an update aimed at another business"
        );
    }
}

#[rok_db::test]
async fn a_claim_keeps_its_numbers_when_its_batch_is_deleted(db: Db) {
    install_pharmacy_stack(&db).await;

    let afya = seed_business(&db, "Afya Pharmacy").await;
    seed_every_new_table(&db, &afya, "A").await;

    let column = sqlx::query(
        "select is_nullable, \
                (select confdeltype::text from pg_constraint \
                  where conrelid = 'pharmacy.insurance_claims'::regclass \
                    and conname = 'insurance_claims_claim_batch_id_fkey') as delete_rule, \
                (select confupdtype::text from pg_constraint \
                  where conrelid = 'pharmacy.insurance_claims'::regclass \
                    and conname = 'insurance_claims_claim_batch_id_fkey') as update_rule \
         from information_schema.columns \
         where table_schema = 'pharmacy' and table_name = 'insurance_claims' \
           and column_name = 'claim_batch_id'",
    )
    .fetch_one(&db)
    .await
    .expect("the claim batch column is in the catalog");

    assert_eq!(
        column.get::<String, _>("is_nullable"),
        "YES",
        "a claim can be written before its batch is known"
    );
    assert_eq!(
        column.get::<String, _>("delete_rule"),
        "n",
        "deleting a batch leaves the claim alone instead of taking it with it"
    );
    assert_eq!(
        column.get::<String, _>("update_rule"),
        "a",
        "moving a claim to another batch is checked, not followed, so the target has to exist"
    );
}
