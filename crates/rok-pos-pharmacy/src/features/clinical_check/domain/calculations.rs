//! The rules over one prescription's check.
//!
//! An alert is not a stop on its own: it is a stop until the call that answers
//! it has been recorded.

use super::entities::{Check, ClinicalCheck, Medicine, Outcome};
use super::enums::Finding;

/// The findings that hold the prescription until they are answered.
#[must_use]
pub fn blocking(check: &ClinicalCheck) -> Vec<&Check> {
    check
        .checks
        .iter()
        .filter(|check| check.finding.blocks())
        .collect()
}

/// How many of the checks found nothing to act on.
#[must_use]
pub fn cleared(check: &ClinicalCheck) -> u32 {
    let found = blocking(check).len();
    u32::try_from(check.checks.len() - found).unwrap_or(u32::MAX)
}

/// How many checks there are in all.
#[must_use]
pub fn checks_total(check: &ClinicalCheck) -> u32 {
    u32::try_from(check.checks.len()).unwrap_or(u32::MAX)
}

/// The alert the check opens on, if there is one.
#[must_use]
pub fn alert(check: &ClinicalCheck) -> Option<&Check> {
    check
        .checks
        .iter()
        .find(|one| one.finding == Finding::Alert)
}

/// The outcome the call came to.
#[must_use]
pub fn outcome(check: &ClinicalCheck) -> Option<&Outcome> {
    check.outcomes.iter().find(|outcome| outcome.chosen)
}

/// Whether the check is clear enough to approve for dispensing.
///
/// An alert is answered by a recorded call, so an alert with nothing behind it
/// is a prescription still waiting on the phone.
#[must_use]
pub fn can_approve(check: &ClinicalCheck) -> bool {
    match alert(check) {
        Some(_) => outcome(check).is_some_and(|outcome| outcome.chosen),
        None => true,
    }
}

/// The medicine the label preview is showing: the first one on the paper.
#[must_use]
pub fn preview(check: &ClinicalCheck) -> Option<&Medicine> {
    check.medicines.first()
}

/// How many labels the printer will be fed for, one per medicine.
#[must_use]
pub fn labels(check: &ClinicalCheck) -> usize {
    check.medicines.len()
}

/// How many counselling points the patient is given.
#[must_use]
pub fn counselling_points(check: &ClinicalCheck) -> usize {
    check.counselling.len()
}
