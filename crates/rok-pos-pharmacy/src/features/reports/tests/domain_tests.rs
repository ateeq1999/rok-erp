//! The reports' rules, against the board's own figures.

use crate::features::reports::data::models::ReportRecord;
use crate::features::reports::data::story;
use crate::features::reports::domain::calculations::{
    kinds, missing_kinds, offered, row_sum, total,
};
use crate::features::reports::domain::entities::{Kind, Reports};

/// The board's reports, as the entities they become.
fn board() -> Reports {
    Reports::from(story::board())
}

#[test]
fn every_report_is_a_sum_of_its_own_rows() {
    let reports = board();
    assert_eq!(reports.broken().len(), 0, "a heading that is not a sum");
    for report in &reports.reports {
        assert_eq!(
            total(report),
            row_sum(report),
            "{} does not add up to {} {}",
            report.title,
            total(report),
            report.unit
        );
    }
}

#[test]
fn the_claims_figure_is_the_one_the_claims_board_shows() {
    let reports = board();
    let claims = reports.report(Kind::Claims).expect("the claims report");
    assert_eq!(total(claims), 92_500);
    let chased = claims.flagged();
    assert_eq!(chased.len(), 1);
    assert!(
        chased[0].label.contains("90 days"),
        "the chased bucket is the one over 90 days"
    );
}

#[test]
fn stock_that_is_still_sellable_is_not_counted_as_a_loss() {
    let reports = board();
    let expiry = reports.report(Kind::Expiry).expect("the expiry report");
    let within_30 = expiry
        .rows
        .iter()
        .find(|row| row.label.contains("30 days"))
        .expect("the thirty day bucket");
    assert!(
        within_30.detail.contains("still sellable"),
        "a live batch is shown apart from the written-off ones"
    );
    assert!(expiry.flagged().len() >= 2);
}

#[test]
fn controlled_sales_are_reported_against_the_register() {
    let reports = board();
    let controlled = reports
        .report(Kind::Controlled)
        .expect("the controlled report");
    assert!(
        controlled
            .rows
            .iter()
            .all(|row| row.detail.contains("dispensed")),
        "every controlled row names what was dispensed"
    );
    assert!(controlled.adds_up());
}

#[test]
fn the_board_carries_all_five_reports_in_the_boards_order() {
    let reports = board();
    assert_eq!(offered(&reports), 5);
    assert_eq!(missing_kinds(&reports), 0);
    assert_eq!(
        kinds(&reports),
        vec![
            Kind::Sales,
            Kind::Margin,
            Kind::Expiry,
            Kind::Claims,
            Kind::Controlled,
        ]
    );
    assert_eq!(
        reports.sales().expect("the board leads with sales").total,
        1_842_000
    );
}

#[test]
fn a_report_the_board_does_not_know_still_draws_but_leaves_its_stat_short() {
    let mut record = story::board();
    record.reports.push(ReportRecord {
        kind: "supplier rebates".to_string(),
        ..record.reports[0].clone()
    });
    let reports = Reports::from(record);
    assert_eq!(offered(&reports), 6);
    assert_eq!(missing_kinds(&reports), 0, "the five board kinds are there");
    assert!(
        reports
            .reports
            .last()
            .is_some_and(|report| report.kind == Kind::Unknown),
        "an unclassifiable report is kept, not dropped"
    );
}

#[test]
fn a_board_short_of_its_kinds_says_which_are_missing() {
    let mut record = story::board();
    record
        .reports
        .retain(|report| report.kind != "claims aging");
    let reports = Reports::from(record);
    assert_eq!(missing_kinds(&reports), 1);
    assert!(reports.report(Kind::Claims).is_none());
}

#[test]
fn a_report_record_round_trips_through_the_entities() {
    let reports = board();
    assert_eq!(reports.reports.len(), 5);
    assert_eq!(&*reports.period, "This month");
    assert_eq!(&*reports.branch, "Both branches");
    assert_eq!(reports.reports[0].rows.len(), 7);
    assert_eq!(&*reports.reports[0].unit, "shillings");
}
