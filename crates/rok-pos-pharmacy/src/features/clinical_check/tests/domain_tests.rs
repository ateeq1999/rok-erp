//! The check's rules, against the board's own prescription.

use crate::features::clinical_check::data::story;
use crate::features::clinical_check::domain::calculations::{
    alert, blocking, can_approve, checks_total, cleared, counselling_points, labels, outcome,
    preview,
};
use crate::features::clinical_check::domain::entities::ClinicalCheck;
use crate::features::clinical_check::domain::enums::{Finding, Severity};

/// The board's prescription, as the entity it becomes.
fn check() -> ClinicalCheck {
    ClinicalCheck::from(story::rx_2214())
}

/// The board's prescription with nothing found on it and nothing called.
fn quiet() -> ClinicalCheck {
    let mut record = story::rx_2214();
    record.checks.clear();
    record.outcomes.clear();
    ClinicalCheck::from(record)
}

#[test]
fn an_alert_holds_the_prescription_until_the_call_is_recorded() {
    let check = check();
    assert_eq!(checks_total(&check), 6);
    assert_eq!(cleared(&check), 5);
    assert_eq!(
        cleared(&check) + u32::try_from(blocking(&check).len()).unwrap(),
        checks_total(&check),
        "every check is either clear or blocking"
    );
    assert_eq!(blocking(&check).len(), 1);
    let held = blocking(&check)[0];
    assert_eq!(&*held.label, "Interactions");
    assert!(held.finding.blocks());
}

#[test]
fn the_check_opens_on_the_interaction_and_not_on_the_first_row() {
    let check = check();
    let found = alert(&check).expect("an alert on this prescription");
    assert_eq!(&*found.label, "Interactions");
    assert!(found.detail.contains("warfarin"));
    assert_ne!(
        &*found.label, &*check.checks[0].label,
        "the first check is the identity, which clears"
    );
    assert!(!Finding::Clear.blocks());
    assert_eq!(Finding::Clear.severity(), Severity::Clear);
    assert_eq!(Finding::Alert.severity(), Severity::Alert);
}

#[test]
fn an_alert_with_nothing_behind_it_is_still_waiting_on_the_phone() {
    let mut record = story::rx_2214();
    record
        .outcomes
        .iter_mut()
        .for_each(|one| one.chosen = false);
    let check = ClinicalCheck::from(record);
    assert_eq!(outcome(&check), None);
    assert!(alert(&check).is_some());
    assert!(!can_approve(&check));
}

#[test]
fn a_recorded_call_releases_the_hold() {
    let check = check();
    let recorded = outcome(&check).expect("the board records a call");
    assert!(recorded.chosen);
    assert_eq!(&*recorded.label, "Kept as written, extra INR check");
    assert!(can_approve(&check));
}

#[test]
fn nothing_on_the_prescription_needs_no_call_to_be_approved() {
    let check = quiet();
    assert_eq!(checks_total(&check), 0);
    assert_eq!(cleared(&check), 0);
    assert_eq!(blocking(&check).len(), 0);
    assert_eq!(alert(&check), None);
    assert_eq!(outcome(&check), None);
    assert!(
        can_approve(&check),
        "a prescription with nothing to resolve is not waiting on a call"
    );
}

#[test]
fn the_label_preview_is_the_first_medicine_on_the_paper() {
    let check = check();
    let first = preview(&check).expect("two medicines are on the paper");
    assert_eq!(&*first.name, "Metronidazole 400mg tablets");
    assert_eq!(&*first.batch, "MTZ-2503");
    assert_eq!(&*first.quantity, "21");
    assert_eq!(
        &*first.label_words,
        "Take 1 tablet 3 times a day for 7 days. Avoid alcohol. Complete the course."
    );
    assert_eq!(labels(&check), 2);
    assert_eq!(counselling_points(&check), 5);
}

#[test]
fn a_paper_with_no_medicines_previews_nothing_rather_than_a_guess() {
    let mut record = story::rx_2214();
    record.medicines.clear();
    let check = ClinicalCheck::from(record);
    assert_eq!(preview(&check), None);
    assert_eq!(labels(&check), 0);
}
