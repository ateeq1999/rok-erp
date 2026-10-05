//! The arithmetic over the licences board.
//!
//! Every count the screen shows is derived from the data itself, so a stat
//! cannot claim a number the cards do not have.

use super::entities::{CivilDate, Document, Licence, Licences, Requirement};
use super::enums::Standing;

/// How many licences the pharmacy holds.
#[must_use]
pub fn held(licences: &[Licence]) -> u32 {
    counted(licences.iter().map(|_| true))
}

/// How many licences the pharmacy has to act on: held but about to lapse, or
/// not held at all.
#[must_use]
pub fn renewing(licences: &[Licence]) -> u32 {
    counted(licences.iter().map(Licence::needs_renewal))
}

/// How many of the inspector's requirements the pharmacy satisfies.
#[must_use]
pub fn ready(requirements: &[Requirement]) -> u32 {
    counted(requirements.iter().map(|requirement| requirement.ready))
}

/// How many of the inspector's requirements the pharmacy is missing.
#[must_use]
pub fn not_ready(requirements: &[Requirement]) -> u32 {
    counted(requirements.iter().map(|requirement| !requirement.ready))
}

/// How many requirements there are, which the screen shows as "5 of 6".
#[must_use]
pub fn total(requirements: &[Requirement]) -> u32 {
    counted(requirements.iter().map(|_| true))
}

/// How many documents the folder is missing.
#[must_use]
pub fn missing_documents(documents: &[Document]) -> u32 {
    counted(documents.iter().map(|document| !document.on_file))
}

/// How many days the board's lead renewal has left from `today`.
///
/// `None` when nothing needs renewing, which is the good case: the screen has
/// no renewal to lead with.
#[must_use]
pub fn days_until_renewal(licences: &Licences, today: CivilDate) -> Option<i32> {
    licences
        .renewal()
        .map(|licence| licence.days_until_expiry(today))
}

/// The standing the board's chip shows for a readiness item's twin: whether a
/// licence standing counts as ready for inspection.
#[must_use]
pub fn standing_is_valid(standing: Standing) -> bool {
    matches!(standing, Standing::Valid)
}

/// A count a screen uses. A pharmacy's folder cannot hold more rows than a
/// `u32` counts, but the conversion says so rather than assuming it.
fn counted(kept: impl Iterator<Item = bool>) -> u32 {
    u32::try_from(kept.filter(|row| *row).count()).unwrap_or(u32::MAX)
}

/// The board's date, as a test and a story source agree on it.
pub const BOARD_TODAY: CivilDate = CivilDate::new(2026, 10, 4);
