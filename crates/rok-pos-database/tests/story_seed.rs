//! Phase 1 step 5: the Afya story seed loads and its figures are the
//! boards' figures - the expiry tiles, the register balance, UZ-7781,
//! RC-0047, the insurance batches and the RX-2210 split.
//!
//! The story's "today" is 2026-10-04, so the expiry windows are measured
//! from that date rather than from the clock.

use rok_db::{Db, raw};
use rok_pos_database::{ModuleInstaller, default_modules_directory, load_afya_story};

/// The story's today.
const TODAY: &str = "2026-10-04";

fn installer() -> ModuleInstaller {
    ModuleInstaller::load(&default_modules_directory()).expect("the manifests load")
}

/// One `(count, value)` pair from the batch windows the tiles show.
async fn batch_window(db: &Db, from: &str, to: Option<&str>) -> (i64, i64) {
    let until = match to {
        Some(to) => format!("and b.expires_on <= date '{to}'"),
        None => String::new(),
    };
    let sql = format!(
        "select count(distinct b.id), \
                coalesce(sum(m.quantity_change * m.unit_cost), 0)::bigint \
         from inventory.stock_batches b \
         join inventory.stock_movements m \
           on m.stock_batch_id = b.id and m.movement_type = 'opening_balance' \
         where b.expires_on > date '{from}' {until}"
    );
    let row: (i64, i64) = raw(sql).fetch_one(db).await.expect("the window is counted");
    row
}

/// The seed loads, twice, and every board figure comes back out.
#[rok_db::test]
async fn the_story_seed_loads_and_matches_the_boards(db: Db) {
    let installer = installer();
    installer.install(&db).await.expect("the modules install");
    load_afya_story(&db).await.expect("the seed loads");
    // Loading it again replaces the story rather than doubling it.
    load_afya_story(&db).await.expect("the seed loads twice");

    // The 12 medicines with the schedules the catalogue board shows.
    let medicines: i64 = raw("select count(*) from pharmacy.medicine_details")
        .scalar(&db)
        .await
        .expect("the catalogue counts");
    assert_eq!(medicines, 12, "twelve board medicines");
    let schedules: (i64, i64, i64, i64) = raw(
        "select count(*) filter (where medicine_schedule = 'general_sale'), \
                count(*) filter (where medicine_schedule = 'pharmacy_medicine'), \
                count(*) filter (where medicine_schedule = 'prescription_only'), \
                count(*) filter (where medicine_schedule = 'controlled') \
         from pharmacy.medicine_details",
    )
    .fetch_one(&db)
    .await
    .expect("the schedules count");
    assert_eq!(schedules, (3, 2, 6, 1), "the board's schedule spread");

    // Mwenge's shelf matches the catalogue's stock column.
    let amoxicillin: i64 = raw(
        "select quantity_on_hand::bigint from inventory.stock_levels \
         where stock_location_id = '00000000-0000-7000-8000-000000000021' \
           and product_id = '00000000-0000-7000-8000-000000000101'",
    )
    .scalar(&db)
    .await
    .expect("Mwenge holds amoxicillin");
    assert_eq!(amoxicillin, 1231, "1,231 caps on the board");

    // The expiry tiles: 2 expired, 5 in 30 days worth 186,400, 14 in 90
    // days worth 612,900, all at cost.
    let (expired, _) = batch_window(&db, "1900-01-01", Some(TODAY)).await;
    assert_eq!(expired, 2, "expired, still on shelf");
    let (soon, soon_value) = batch_window(&db, TODAY, Some("2026-11-03")).await;
    assert_eq!((soon, soon_value), (5, 186_400), "expiring in 30 days");
    let (ninety, ninety_value) = batch_window(&db, TODAY, Some("2027-01-02")).await;
    assert_eq!((ninety, ninety_value), (14, 612_900), "expiring in 90 days");

    // Quarantined: the two expired batches and AMS-2404 under the recall.
    let quarantined: i64 = raw(
        "select count(distinct m.stock_batch_id) from inventory.stock_movements m \
         where m.stock_location_id = '00000000-0000-7000-8000-000000000023'",
    )
    .scalar(&db)
    .await
    .expect("the quarantine box counts");
    assert_eq!(quarantined, 3, "three quarantined batches");

    // Ten prescriptions, four dispensed today.
    let queue: (i64, i64) = raw(
        "select count(*), count(*) filter (where status = 'dispensed') \
         from pharmacy.prescriptions",
    )
    .fetch_one(&db)
    .await
    .expect("the queue counts");
    assert_eq!(queue, (10, 4), "RX-2210 to RX-2219, four dispensed");

    // The RX-2210 split: 13,700 total, 9,590 insurer, 4,110 patient.
    let sale: i64 = raw("select total_amount::bigint from point_of_sale.sales \
         where id = '00000000-0000-7000-8000-000000000a01'")
    .scalar(&db)
    .await
    .expect("the sale reads");
    assert_eq!(sale, 13_700, "the board's total");
    let paid: i64 = raw(
        "select coalesce(sum(amount), 0)::bigint from point_of_sale.sale_payments \
         where sale_id = '00000000-0000-7000-8000-000000000a01'",
    )
    .scalar(&db)
    .await
    .expect("the payment reads");
    let claim: i64 = raw(
        "select claim_amount::bigint from pharmacy.insurance_claims \
         where id = '00000000-0000-7000-8000-000000000b30'",
    )
    .scalar(&db)
    .await
    .expect("the claim reads");
    assert_eq!((claim, paid), (9_590, 4_110), "insurer and patient shares");
    assert_eq!(claim + paid, sale, "the split adds up to the total");

    // The controlled register: received 100, dispensed 20 and 10, balance 80.
    let register: Vec<(String, i64, i64)> = raw(
        "select entry_type, quantity::bigint, balance_after::bigint \
         from pharmacy.controlled_substance_entries \
         where product_id = '00000000-0000-7000-8000-000000000109' \
         order by recorded_at",
    )
    .fetch_all(&db)
    .await
    .expect("the register reads");
    assert_eq!(
        register,
        vec![
            ("received".to_string(), 100, 100),
            ("dispensed".to_string(), 20, 80),
            ("dispensed".to_string(), 10, 70),
            ("returned".to_string(), 10, 80),
        ],
        "the register's entries and running balance"
    );
    let balance: i64 = raw(
        "select balance_after::bigint from pharmacy.controlled_substance_entries \
         where product_id = '00000000-0000-7000-8000-000000000109' \
         order by recorded_at desc limit 1",
    )
    .scalar(&db)
    .await
    .expect("the balance reads");
    assert_eq!(balance, 80, "balance 80, which is what the shelf holds");
    let on_shelf: i64 = raw(
        "select quantity_on_hand::bigint from inventory.stock_levels \
         where stock_location_id = '00000000-0000-7000-8000-000000000021' \
           and product_id = '00000000-0000-7000-8000-000000000109'",
    )
    .scalar(&db)
    .await
    .expect("the shelf reads");
    assert_eq!(balance, on_shelf, "the count matches the register");

    // Order UZ-7781: six lines, 518,700, sent.
    let order: (i64, i64, String) = raw("select count(*), po.total_amount::bigint, po.status \
         from purchasing.purchase_orders po \
         join purchasing.purchase_order_lines l on l.purchase_order_id = po.id \
         where po.purchase_order_number = 'UZ-7781' \
         group by po.total_amount, po.status")
    .fetch_one(&db)
    .await
    .expect("the order reads");
    assert_eq!(order, (6, 518_700, "sent".to_string()), "the board's order");

    // Recall RC-0047: quarantined, 14 bottles in the box, 3 patients reached.
    let recall: (String, i64) = raw("select status, \
                (select quantity::bigint from pharmacy.recall_actions \
                  where recall_notice_id = r.id and action_type = 'quarantined') \
         from pharmacy.recall_notices r where recall_reference = 'RC-0047'")
    .fetch_one(&db)
    .await
    .expect("the recall reads");
    assert_eq!(recall, ("quarantined".to_string(), 14));
    let reached: i64 = raw("select count(*) from pharmacy.recall_actions \
         where action_type = 'patient_contacted'")
    .scalar(&db)
    .await
    .expect("the contacts count");
    assert_eq!(reached, 3, "three reached by text, two pending");

    // The insurance batches: September 214 claims and 4,862,300 with 11
    // queried, October open with RX-2210's 9,590 already in it.
    let september: (i64, i64, String) = raw(
        "select count(*), coalesce(sum(c.claim_amount), 0)::bigint, b.status \
         from pharmacy.insurance_claims c \
         join pharmacy.insurance_claim_batches b on b.id = c.claim_batch_id \
         where b.period_start_on = date '2026-09-01' \
         group by b.status",
    )
    .fetch_one(&db)
    .await
    .expect("the September batch counts");
    assert_eq!(
        september,
        (214, 4_862_300, "queried".to_string()),
        "the board's September batch"
    );
    let queried: i64 = raw("select count(*) from pharmacy.insurance_claims \
         where status = 'rejected'")
    .scalar(&db)
    .await
    .expect("the queries count");
    assert_eq!(queried, 11, "eleven queried claims with their reasons");
    let october: i64 = raw("select coalesce(sum(c.claim_amount), 0)::bigint \
         from pharmacy.insurance_claims c \
         join pharmacy.insurance_claim_batches b on b.id = c.claim_batch_id \
         where b.period_start_on = date '2026-10-01'")
    .scalar(&db)
    .await
    .expect("the October batch sums");
    assert_eq!(october, 9_590, "RX-2210's share went into October");

    // The interaction rule RX-2214 fires, named with its source.
    let rules: i64 = raw("select count(*) from pharmacy.interaction_rules \
         where lower(first_generic_name) = 'metronidazole' \
           and lower(second_generic_name) = 'warfarin' \
           and severity = 'serious' \
           and source_reference <> '' \
           and reviewed_by_user_id is not null")
    .scalar(&db)
    .await
    .expect("the rules count");
    assert_eq!(rules, 1, "the warfarin and metronidazole rule, reviewed");

    // The claim rows and the batch totals must still match after a second
    // load, which is what makes the seed safe to run twice.
    load_afya_story(&db)
        .await
        .expect("the seed loads a third time");
    let again: (i64, i64) = raw("select count(*), coalesce(sum(claim_amount), 0)::bigint \
         from pharmacy.insurance_claims")
    .fetch_one(&db)
    .await
    .expect("the claims still count");
    assert_eq!(again, (215, 4_871_890), "214 September claims plus RX-2210");
}
