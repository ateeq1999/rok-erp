//! The records a source hands over for the reports board, and the entities
//! they become.
//!
//! A source names its reports, periods and rows as the board spells them; the
//! parse into [`Kind`] and the entities is the data layer's job, so a source's
//! spelling never reaches the domain. A word the board does not know is a
//! [`Kind::Unknown`], which still draws: an unknown report is shown, not
//! dropped.

use crate::features::reports::domain::entities::{Kind, Report, Reports, Row, text};

/// One line of a report, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowRecord {
    /// What the row is.
    pub label: String,
    /// How the row is broken down: the schedule, the branch, the batch.
    pub detail: String,
    /// What the row came to, in the report's own unit.
    pub value: i64,
    /// The rate the row carries: a margin percentage, a share of the sales.
    pub share: Option<String>,
    /// Whether the row is the one the owner has to act on.
    pub flagged: bool,
}

/// One report, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportRecord {
    /// Which of the board's reports it is, as the source names it.
    pub kind: String,
    /// What the card is called.
    pub title: String,
    /// The question it answers.
    pub question: String,
    /// What the rows add up to.
    pub total: i64,
    /// What the figures are counted in.
    pub unit: String,
    /// The rows themselves.
    pub rows: Vec<RowRecord>,
    /// The note under the table, in the board's own words.
    pub note: String,
}

/// The whole board, as a source hands it over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportsRecord {
    /// The period the filters are on.
    pub period: String,
    /// The branch the filters are on.
    pub branch: String,
    /// The reports themselves.
    pub reports: Vec<ReportRecord>,
}

/// What a source calls one of the board's reports.
///
/// A word the board does not know is [`Kind::Unknown`], which still draws
/// rather than being dropped on the way in.
fn kind(spelled: &str) -> Kind {
    match spelled {
        "sales by medicine" => Kind::Sales,
        "margin" => Kind::Margin,
        "expiry losses" => Kind::Expiry,
        "claims aging" => Kind::Claims,
        "controlled movements" => Kind::Controlled,
        _ => Kind::Unknown,
    }
}

impl From<RowRecord> for Row {
    fn from(record: RowRecord) -> Self {
        Self {
            label: text(&record.label),
            detail: text(&record.detail),
            value: record.value,
            share: record.share.as_deref().map(text),
            flagged: record.flagged,
        }
    }
}

impl From<ReportRecord> for Report {
    fn from(record: ReportRecord) -> Self {
        Self {
            kind: kind(&record.kind),
            title: text(&record.title),
            question: text(&record.question),
            total: record.total,
            unit: text(&record.unit),
            rows: record.rows.into_iter().map(Row::from).collect(),
            note: text(&record.note),
        }
    }
}

impl From<ReportsRecord> for Reports {
    fn from(record: ReportsRecord) -> Self {
        Self {
            period: text(&record.period),
            branch: text(&record.branch),
            reports: record.reports.into_iter().map(Report::from).collect(),
        }
    }
}
