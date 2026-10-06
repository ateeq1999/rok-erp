//! What the reports board draws, and what it needs to say it.
//!
//! Nothing here knows it will be drawn, or where it came from. Words and
//! figures stay apart: a row carries its text as [`Text`] and its value as a
//! number, because the board's rule - the heading is a sum of its own rows -
//! only holds while the parts stay countable.
//!
//! The words the board prints, as the story's source spells them - medicine
//! names, schedule names, the notes under each card - are data, not copy: they
//! stay with the story until their phase reads them from the database. The
//! board's own furniture, which is the same in both languages, comes through
//! `rust_i18n` in the presentation.

use std::sync::Arc;

/// A word the board draws every frame.
pub type Text = Arc<str>;

/// Make a [`Text`] out of borrowed words.
#[must_use]
pub fn text(value: &str) -> Text {
    Text::from(value)
}

/// One line of a report: what it is, what it counts, and what it came to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// What the row is.
    pub label: Text,
    /// How the row is broken down: the schedule, the branch, the batch.
    pub detail: Text,
    /// What the row came to, in the report's own unit.
    pub value: i64,
    /// The rate the row carries: a margin percentage, a share of the sales.
    pub share: Option<Text>,
    /// Whether the row is the one the owner has to act on.
    pub flagged: bool,
}

/// One report: the question it answers and the rows that answer it.
///
/// The board's rule is [`Report::adds_up`]: the rows add up to the figure in
/// the heading, which is what makes a report worth looking at - the headline
/// is a sum, not a number someone typed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    /// Which of the board's reports this one is.
    pub kind: Kind,
    /// What the card is called.
    pub title: Text,
    /// The question it answers.
    pub question: Text,
    /// What the rows add up to.
    pub total: i64,
    /// What the figures are counted in.
    pub unit: Text,
    /// The rows themselves.
    pub rows: Vec<Row>,
    /// The note under the table, in the board's own words.
    pub note: Text,
}

impl Report {
    /// Whether the report's rows add up to the figure in its heading.
    #[must_use]
    pub fn adds_up(&self) -> bool {
        self.rows.iter().map(|row| row.value).sum::<i64>() == self.total
    }

    /// The rows worth acting on, which the board tints.
    #[must_use]
    pub fn flagged(&self) -> Vec<&Row> {
        self.rows.iter().filter(|row| row.flagged).collect()
    }
}

/// Every report the reports screen offers, on the filters it opens with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reports {
    /// The period the filters are on.
    pub period: Text,
    /// The branch the filters are on.
    pub branch: Text,
    /// The reports themselves, in the order the board lists them.
    pub reports: Vec<Report>,
}

impl Reports {
    /// The report the board leads with: the sales report.
    ///
    /// The board always draws a sales report first, so the report the story's
    /// source hands over first is the one it leads with; a source that hands
    /// over none has nothing to lead with, and the screen shows that as its
    /// empty state rather than panicking.
    #[must_use]
    pub fn sales(&self) -> Option<&Report> {
        self.reports.first()
    }

    /// The report of one of the board's kinds, such as the claims aging
    /// report whose queried total the stat row repeats so the two screens
    /// cannot disagree.
    #[must_use]
    pub fn report(&self, kind: Kind) -> Option<&Report> {
        self.reports.iter().find(|report| report.kind == kind)
    }

    /// The reports whose rows do not add up, which must be none of them.
    #[must_use]
    pub fn broken(&self) -> Vec<&Report> {
        self.reports
            .iter()
            .filter(|report| !report.adds_up())
            .collect()
    }
}

/// One of the reports the board offers, in the words its key carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Sales by medicine and schedule.
    Sales,
    /// Margin: what each schedule leaves after cost.
    Margin,
    /// Expiry losses: what has been, and is about to be, written off.
    Expiry,
    /// Claims aging: how long the insurers have been sitting on the queries.
    Claims,
    /// Controlled movements: what was sold against the register.
    Controlled,
    /// A report the source handed over in words the board does not know. It
    /// still draws - the rows are there - but it answers none of the board's
    /// kinds, so a stat that needs it shows that it is missing.
    Unknown,
}

impl Kind {
    /// The board's five kinds, in the order the board draws them.
    pub const ALL: [Kind; 5] = [
        Kind::Sales,
        Kind::Margin,
        Kind::Expiry,
        Kind::Claims,
        Kind::Controlled,
    ];
}
