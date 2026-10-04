//! Phase 1 step 4: the models for the fifteen tables migration 0002 adds
//! read what the story seed wrote, and write a row of their own.
//!
//! A model that compiles is not a model that matches its table: every column
//! name, every `check` value and every `numeric` scale has to survive a round
//! trip through PostgreSQL. The models carry a `#[rok(tenant)]` column, so
//! every query here runs inside `with_tenant`, which is how the screens read
//! too: outside a business there is nothing to see.

use chrono::{NaiveDate, Utc};
use rok_db::prelude::*;
use rok_db::tenant::with_tenant;
use rok_pos_database::ModuleInstaller;
use rok_pos_database::default_modules_directory;
use rok_pos_database::load_afya_story;
use rok_pos_database::models::{
    CheckKey, CheckResult, ClaimBatchStatus, InsuranceClaimBatch, MedicineDetails,
    MedicineSchedule, PrescriptionCheck, RecallAction, RecallActionType, StorageCondition,
    Temperature, TemperatureLog,
};
use rust_decimal::Decimal;
use uuid::Uuid;

/// The story's business, and the two people who work in it.
const AFYA: &str = "00000000-0000-7000-8000-000000000001";
/// Mwenge.
const MWENGE: &str = "00000000-0000-7000-8000-000000000002";
/// Grace N., the pharmacist in charge.
const GRACE: &str = "00000000-0000-7000-8000-000000000011";

/// Install the eight modules and load the story, once per test database.
async fn story(db: &Db) {
    let installer =
        ModuleInstaller::load(&default_modules_directory()).expect("the manifests load");
    installer.install(db).await.expect("the modules install");
    load_afya_story(db).await.expect("the seed loads");
}

fn afya() -> Uuid {
    Uuid::parse_str(AFYA).expect("the story's organisation")
}

/// The catalogue board comes back through the model, enums and all: twelve
/// medicines, the schedule spread, and one row's fields spelled the way the
/// board spells them.
#[rok_db::test]
async fn medicine_details_read_the_story(db: Db) {
    story(&db).await;

    let medicines = with_tenant(afya(), async {
        MedicineDetails::order_by(MedicineDetails::GENERIC_NAME.asc())
            .all(&db)
            .await
            .expect("the models read")
    })
    .await;
    assert_eq!(medicines.len(), 12, "twelve board medicines");

    let controlled = with_tenant(afya(), async {
        MedicineDetails::filter(MedicineDetails::MEDICINE_SCHEDULE.eq(MedicineSchedule::Controlled))
            .all(&db)
            .await
            .expect("the schedule filter reads")
    })
    .await;
    assert_eq!(controlled.len(), 1, "one controlled medicine: tramadol");
    assert_eq!(
        controlled[0].product_id.to_string(),
        "00000000-0000-7000-8000-000000000109",
        "the tramadol row the register counts"
    );

    let insulin = with_tenant(afya(), async {
        MedicineDetails::filter(MedicineDetails::PRODUCT_ID.eq(
            Uuid::parse_str("00000000-0000-7000-8000-000000000105").expect("the insulin product"),
        ))
        .one(&db)
        .await
        .expect("insulin is on the board")
    })
    .await;
    assert_eq!(
        insulin.medicine_schedule,
        MedicineSchedule::PrescriptionOnly
    );
    assert_eq!(
        insulin.storage_condition,
        StorageCondition::RefrigeratedTwoToEight,
        "the fridge rule is on the row, not in someone's head"
    );
    assert_eq!(insulin.minimum_shelf_life_months_on_delivery, 12);
    assert!(
        !insulin.label_warnings.is_empty(),
        "the label has something to say"
    );
}

/// The insurance batch comes back with its money intact: September's
/// 4,862,300 over 214 claims, in `numeric(18,2)`, and a new batch written by
/// the model reads back the same.
#[rok_db::test]
async fn insurance_claim_batch_keeps_its_money(db: Db) {
    story(&db).await;

    let stored = with_tenant(afya(), async {
        let september = InsuranceClaimBatch::filter(
            InsuranceClaimBatch::PERIOD_START_ON
                .eq(NaiveDate::from_ymd_opt(2026, 9, 1).expect("September exists")),
        )
        .one(&db)
        .await
        .expect("the September batch reads");
        assert_eq!(september.status, ClaimBatchStatus::Queried);
        assert_eq!(
            september.claimed_amount,
            rok_pos_domain::Money::from_shillings(4_862_300),
            "the board's figure survives the round trip"
        );

        // Writing one back is what the submit and payment screens will do.
        let mut october = september;
        october.id = Uuid::now_v7();
        october.period_start_on = NaiveDate::from_ymd_opt(2026, 11, 1).expect("November exists");
        october.period_end_on = NaiveDate::from_ymd_opt(2026, 11, 30).expect("November exists");
        october.status = ClaimBatchStatus::Open;
        october.submitted_at = None;
        october.claimed_amount = rok_pos_domain::Money::from_shillings(9_590);
        october.paid_amount = rok_pos_domain::Money::zero();
        october.deducted_amount = rok_pos_domain::Money::zero();
        october.insert(&db).await.expect("the batch writes")
    })
    .await;
    assert_eq!(
        stored.claimed_amount,
        rok_pos_domain::Money::from_shillings(9_590)
    );

    let reread = with_tenant(afya(), async {
        InsuranceClaimBatch::find_or_fail(&db, stored.id)
            .await
            .expect("it reads back")
    })
    .await;
    assert_eq!(reread, stored, "a write and a read agree");
}

/// The recall board's rows read back through the model: RC-0047's quarantine
/// action carries 14 bottles in a `numeric(18,3)` column, at full precision.
#[rok_db::test]
async fn recall_reads_back_with_quantities(db: Db) {
    story(&db).await;

    let actions = with_tenant(afya(), async {
        RecallAction::filter(RecallAction::ACTION_TYPE.eq(RecallActionType::Quarantined))
            .all(&db)
            .await
            .expect("the quarantine action reads")
    })
    .await;
    assert_eq!(actions.len(), 1, "one quarantine action");
    let quantity = actions[0].quantity.expect("fourteen bottles were counted");
    assert_eq!(
        quantity.decimal(),
        Decimal::new(14, 0),
        "14 bottles in the box, at full precision"
    );

    let all = with_tenant(afya(), async {
        RecallAction::all(&db)
            .await
            .expect("the recall's actions read")
    })
    .await;
    assert_eq!(all.len(), 4, "quarantine, contact, return, credit");
}

/// A temperature log writes and reads at `numeric(5,2)`: 4.50 degrees in the
/// fridge, in range, and still 4.50 after the round trip. Soft delete takes
/// it out of queries without dropping the row.
#[rok_db::test]
async fn temperature_log_round_trips(db: Db) {
    story(&db).await;

    let stored = with_tenant(afya(), async {
        let log = TemperatureLog {
            id: Uuid::now_v7(),
            organization_id: afya(),
            branch_id: Uuid::parse_str(MWENGE).expect("Mwenge"),
            storage_unit_name: "Fridge 1".into(),
            recorded_at: Utc::now(),
            temperature_celsius: Temperature::new(Decimal::new(450, 2)),
            is_in_range: true,
            recorded_by_user_id: Uuid::parse_str(GRACE).expect("Grace N."),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            row_version: 1,
        };
        log.insert(&db).await.expect("the reading writes")
    })
    .await;

    let reread = with_tenant(afya(), async {
        TemperatureLog::find_or_fail(&db, stored.id)
            .await
            .expect("it reads back")
    })
    .await;
    assert_eq!(
        reread.temperature_celsius.decimal(),
        Decimal::new(450, 2),
        "4.50 degrees, not 4.5 rounded to who knows what"
    );
    assert!(reread.is_in_range);

    // Soft delete: the row is gone from queries but not from the table.
    with_tenant(afya(), async {
        reread.delete(&db).await.expect("the reading is struck out");
    })
    .await;
    let gone = with_tenant(afya(), async {
        TemperatureLog::find(&db, stored.id)
            .await
            .expect("the lookup itself still runs")
    })
    .await;
    assert!(gone.is_none(), "a soft-deleted row is not returned");
}

/// The six checks the clinical board records for RX-2214 read back through
/// the model, with the warfarin and metronidazole interaction as the warning
/// it is.
#[rok_db::test]
async fn prescription_checks_read_the_clinical_board(db: Db) {
    story(&db).await;

    let checks = with_tenant(afya(), async {
        PrescriptionCheck::order_by(PrescriptionCheck::CHECK_KEY.asc())
            .all(&db)
            .await
            .expect("the checks read")
    })
    .await;
    assert_eq!(checks.len(), 6, "the board works through six checks");

    let interaction = checks
        .iter()
        .find(|check| check.check_key == CheckKey::Interaction)
        .expect("the interaction check is recorded");
    assert_eq!(
        interaction.result,
        CheckResult::Warning,
        "warfarin and metronidazole is an alert, not a pass"
    );
    assert!(
        checks
            .iter()
            .any(|check| check.check_key == CheckKey::Identity)
            && checks
                .iter()
                .any(|check| check.check_key == CheckKey::DoseRange),
        "the board's checks are all here"
    );
}
