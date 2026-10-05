//! The dashboard's rules, against the board's own figures.

use rok_pos_domain::Money;

use crate::features::dashboard::data::models::DashboardRecord;
use crate::features::dashboard::data::story;
use crate::features::dashboard::domain::calculations::{
    bar_heights, best_sellers, branch_measures, done_tasks, insurance, open_tasks, payment_share,
    percentage, sales_total, tiles,
};
use crate::features::dashboard::domain::entities::Dashboard;
use crate::features::dashboard::domain::enums::TaskKind;
use crate::features::dashboard::domain::measures::MeasureValue;

/// The board's own day, as the domain entity it becomes.
fn board() -> Dashboard {
    Dashboard::from(story::mwenge_day())
}

#[test]
fn the_day_totals_the_ways_it_was_paid() {
    assert_eq!(sales_total(&board()), Money::from_shillings(1_846_200));
    assert_eq!(insurance(&board()), Money::from_shillings(701_600));
}

#[test]
fn the_payment_split_is_the_boards() {
    let dashboard = board();
    let shares: Vec<i64> = dashboard
        .payments
        .iter()
        .map(|payment| payment_share(&dashboard, payment.amount))
        .collect();
    assert_eq!(shares, [38, 34, 28]);
}

#[test]
fn the_insurance_share_is_what_the_tile_says() {
    let dashboard = board();
    let tiles = tiles(&dashboard);
    assert_eq!(
        tiles[0].measure,
        MeasureValue::Money(Money::from_shillings(1_846_200))
    );
    assert_eq!(tiles[0].note.to_string(), "Insurance share 38%");
    assert_eq!(
        tiles[0].note_figure,
        Some(MeasureValue::Money(Money::from_shillings(701_600)))
    );
}

#[test]
fn a_day_with_no_payments_totals_nothing_and_shares_nothing() {
    let mut record = story::mwenge_day();
    record.payments.clear();
    let dashboard = Dashboard::from(record);
    assert_eq!(sales_total(&dashboard), Money::zero());
    assert_eq!(payment_share(&dashboard, Money::from_shillings(100)), 0);
    assert_eq!(insurance(&dashboard), Money::zero());
}

#[test]
fn percentages_round_half_up_and_survive_nothing() {
    assert_eq!(percentage(1, 8), 13);
    assert_eq!(percentage(1, 3), 33);
    assert_eq!(percentage(5, 0), 0);
}

#[test]
fn the_busiest_hour_has_the_tallest_bar() {
    let heights = bar_heights(&board(), 100.);
    assert_eq!(heights.len(), 8);
    assert!(
        (heights[2] - 100.).abs() < f32::EPSILON,
        "10:00 was the busiest hour"
    );
    assert!(
        (heights[0] - 31.).abs() < f32::EPSILON,
        "08:00 sold 96,500 of 312,400"
    );
}

#[test]
fn a_day_with_no_hours_draws_no_bars_and_a_flat_day_draws_flat_bars() {
    let mut empty = story::mwenge_day();
    empty.hours.clear();
    assert_eq!(
        bar_heights(&Dashboard::from(empty), 100.),
        Vec::<f32>::new()
    );

    let mut flat = story::mwenge_day();
    for hour in &mut flat.hours {
        hour.sales = 0;
    }
    assert_eq!(bar_heights(&Dashboard::from(flat), 100.), vec![0.; 8]);
}

#[test]
fn the_branch_table_compares_both_branches() {
    let measures = branch_measures(&board());
    assert_eq!(
        measures[0].here,
        MeasureValue::Money(Money::from_shillings(1_846_200))
    );
    assert_eq!(
        measures[0].there,
        MeasureValue::Money(Money::from_shillings(1_212_800))
    );
    assert_eq!(measures[2].here, MeasureValue::Percent(38));
    assert_eq!(measures[2].there, MeasureValue::Percent(44));
    assert_eq!(measures.len(), 5);
}

#[test]
fn the_tasks_split_into_the_open_ones_and_the_finished_ones() {
    let dashboard = board();
    assert_eq!(dashboard.tasks.len(), 5);
    assert_eq!(open_tasks(&dashboard).len(), 4);
    assert_eq!(done_tasks(&dashboard), 1);
    assert_eq!(dashboard.tasks[4].kind, TaskKind::Done);
    assert!(dashboard.tasks[4].is_done());
    assert_eq!(
        &*dashboard.tasks[0].title,
        "Recall RC-0047 \u{b7} Amoxicillin 250mg/5ml, batch AMS-2404"
    );
}

#[test]
fn the_best_sellers_are_ordered_by_money_not_by_the_story() {
    let dashboard = board();
    let sold: Vec<(&str, i64)> = best_sellers(&dashboard)
        .into_iter()
        .map(|medicine| (&*medicine.name as &str, medicine.sales.round_to_shillings()))
        .collect();
    assert_eq!(
        sold,
        vec![
            ("Metformin 500mg tablets", 50_400),
            ("Amoxicillin 500mg capsules", 46_200),
            ("Amlodipine 5mg tablets", 45_000),
            ("Paracetamol 500mg tablets", 32_000),
            ("Oral rehydration salts", 31_000),
        ]
    );
}

#[test]
fn a_source_record_becomes_the_days_entity() {
    let record = story::mwenge_day();
    let dashboard = Dashboard::from(record.clone());
    assert_eq!(&*dashboard.branch, story::MWENGE);
    assert_eq!(dashboard.hours.len(), record.hours.len());
    assert_eq!(dashboard.hours[2].sales, Money::from_shillings(312_400));
    assert_eq!(&*dashboard.other_branch.name, story::TEGETA);
    assert_eq!(dashboard.claims_queried, record.claims_queried);
}

#[test]
fn a_source_word_the_domain_does_not_know_is_a_finished_task() {
    let mut record = story::mwenge_day();
    record.tasks[0].kind = "Something else".to_string();
    record.tasks[0].severity = "Something else".to_string();
    record.tasks[0].screen = "somewhere else".to_string();
    let dashboard = Dashboard::from(record);
    assert_eq!(dashboard.tasks[0].kind, TaskKind::Done);
    assert!(dashboard.tasks[0].is_done());
}

#[test]
fn an_empty_record_is_a_day_with_nothing_on_it() {
    let empty = DashboardRecord {
        branch: "Tegeta".to_string(),
        hours: Vec::new(),
        payments: Vec::new(),
        prescriptions_dispensed: 0,
        prescriptions_waiting: 0,
        oldest_waiting_minutes: 0,
        expiring_within_30_days: 0,
        expiring_batches: 0,
        expired_batches_blocked: 0,
        claims_queried: 0,
        claims_queried_in: String::new(),
        average_basket: 0,
        tasks: Vec::new(),
        top_medicines: Vec::new(),
        other_branch: crate::features::dashboard::data::models::OtherBranchRecord {
            name: "Mwenge".to_string(),
            sales: 0,
            prescriptions_dispensed: 0,
            insurance_share_percent: 0,
            average_basket: 0,
            prescriptions_waiting: 0,
        },
    };
    let dashboard = Dashboard::from(empty);
    assert_eq!(sales_total(&dashboard), Money::zero());
    assert_eq!(open_tasks(&dashboard).len(), 0);
    assert_eq!(best_sellers(&dashboard).len(), 0);
    assert_eq!(tiles(&dashboard).len(), 4);
}
