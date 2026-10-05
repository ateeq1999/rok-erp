//! The arithmetic over a patient's record and the list it is found from.
//!
//! Every count the screens show is derived from the data itself, so a stat
//! cannot claim a number the cards do not have.

use super::entities::{Listing, Patient};
use super::enums::Cover;

/// How many medicines on the record the pharmacy refills on a date it knows.
#[must_use]
pub fn refills_due(patient: &Patient) -> u32 {
    counted(
        patient
            .medicines
            .iter()
            .map(super::entities::Medicine::is_a_pharmacy_refill),
    )
}

/// How many patients the list shows.
#[must_use]
pub fn listed(listings: &[Listing]) -> u32 {
    u32::try_from(listings.len()).unwrap_or(u32::MAX)
}

/// How many of the list's patients have a refill due within the week.
#[must_use]
pub fn due_this_week(listings: &[Listing]) -> u32 {
    counted(listings.iter().map(Listing::needs_attention))
}

/// How many of the list's patients an insurer pays for.
#[must_use]
pub fn insured(listings: &[Listing]) -> u32 {
    counted(
        listings
            .iter()
            .map(|listing| listing.cover == Cover::Insurance),
    )
}

/// A count a screen uses. A pharmacy's books cannot hold more rows than a
/// `u32` counts, but the conversion says so rather than assuming it.
fn counted(kept: impl Iterator<Item = bool>) -> u32 {
    u32::try_from(kept.filter(|row| *row).count()).unwrap_or(u32::MAX)
}
