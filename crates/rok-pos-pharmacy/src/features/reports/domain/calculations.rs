//! What the reports board works out from its rows.
//!
//! Every figure the screen shows is derived from the data itself, so a stat
//! cannot claim a number the cards do not have, and a heading cannot claim a
//! sum its rows do not make.

use super::entities::{Kind, Report, Reports};

/// The total a report's heading carries, from the report itself.
///
/// The board prints the figure in the heading and the sum of the rows as one
/// number; the rule is that they are the same number.
#[must_use]
pub const fn total(report: &Report) -> i64 {
    report.total
}

/// The rows' own sum, which [`total`] must agree with.
#[must_use]
pub fn row_sum(report: &Report) -> i64 {
    report.rows.iter().map(|row| row.value).sum()
}

/// The report's title, which the screen draws in the card's head.
#[must_use]
pub fn title(report: &Report) -> &str {
    &report.title
}

/// How many of the board's reports the source handed over.
#[must_use]
pub fn offered(reports: &Reports) -> u32 {
    count(reports.reports.len())
}

/// The kinds the source handed over, in the order the board draws them.
#[must_use]
pub fn kinds(reports: &Reports) -> Vec<Kind> {
    reports.reports.iter().map(|report| report.kind).collect()
}

/// How many of the board's five reports are on screen.
#[must_use]
pub fn missing_kinds(reports: &Reports) -> u32 {
    let on_screen = kinds(reports);
    let missing = Kind::ALL
        .iter()
        .filter(|kind| !on_screen.contains(kind))
        .count();
    count(missing)
}

/// A count a screen uses. A board cannot hold more reports than a `u32`
/// counts, but the conversion says so rather than assuming it.
fn count(items: usize) -> u32 {
    u32::try_from(items).unwrap_or(u32::MAX)
}
