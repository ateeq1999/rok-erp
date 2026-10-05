//! The record's rules, against the board's own patient.

use crate::features::patient_profile::data::story;
use crate::features::patient_profile::domain::calculations::{
    due_this_week, insured, listed, refills_due,
};
use crate::features::patient_profile::domain::entities::Patient;
use crate::features::patient_profile::domain::enums::{Cover, Urgency};

/// The board's patient, as the entity it becomes.
fn patient() -> Patient {
    Patient::from(story::mzee_salim())
}

/// The board's list, as the entities it becomes.
fn directory() -> Vec<crate::features::patient_profile::domain::entities::Listing> {
    story::directory()
        .into_iter()
        .map(crate::features::patient_profile::domain::entities::Listing::from)
        .collect()
}

#[test]
fn the_record_opens_on_the_patient_with_a_refill_due() {
    let patient = patient();
    assert_eq!(&*patient.name, "Mzee Salim R.");
    assert_eq!(
        patient.next_due().map(|medicine| medicine.name.to_string()),
        Some("Metformin 500mg tablets".to_string())
    );
    assert_eq!(
        &*patient.medicines[0].due, "Set by clinic",
        "the clinic sets the warfarin interval, so it is not a pharmacy refill"
    );
}

#[test]
fn only_some_of_the_medicines_are_pharmacy_refills() {
    let patient = patient();
    let due: Vec<String> = patient
        .medicines
        .iter()
        .filter(|medicine| medicine.is_a_pharmacy_refill())
        .map(|medicine| medicine.name.to_string())
        .collect();
    assert_eq!(
        due,
        [
            "Metformin 500mg tablets".to_string(),
            "Amlodipine 5mg tablets".to_string()
        ]
    );
    assert_eq!(refills_due(&patient), 2);
}

#[test]
fn the_history_is_the_boards_six_fills() {
    let patient = patient();
    assert_eq!(patient.fill_count(), 6);
    assert_eq!(&*patient.fills[0].reference, "RX-2214");
    assert_eq!(&*patient.fills[0].status, "Approved, at till");
}

#[test]
fn the_notes_are_the_boards_two_and_carry_a_pharmacist() {
    let patient = patient();
    assert_eq!(patient.notes.len(), 2);
    assert!(
        patient
            .notes
            .iter()
            .all(|note| note.meta.contains("Grace N."))
    );
}

#[test]
fn the_patient_has_consented_to_reminders() {
    let patient = patient();
    assert!(patient.wants_reminders());
    assert!(patient.consent.contains("14 Mar 2024"));
}

#[test]
fn the_list_counts_are_derived_from_the_rows() {
    let directory = directory();
    assert_eq!(listed(&directory), 7);
    assert_eq!(due_this_week(&directory), 4);
    assert_eq!(insured(&directory), 5);
}

#[test]
fn a_row_asks_for_attention_only_when_the_week_is_short() {
    let directory = directory();
    let attention: Vec<String> = directory
        .iter()
        .filter(|listing| listing.needs_attention())
        .map(|listing| listing.name.to_string())
        .collect();
    assert_eq!(
        attention,
        [
            "Mzee Salim R.".to_string(),
            "Zainabu A.".to_string(),
            "Neema K.".to_string(),
            "Fatuma H.".to_string(),
        ]
    );
}

#[test]
fn a_listing_without_a_refill_is_not_urgent() {
    let directory = directory();
    let quiet = directory
        .iter()
        .find(|listing| listing.next_due.is_none())
        .expect("the board's list has one patient with nothing due");
    assert_eq!(quiet.urgency, Urgency::Nothing);
    assert!(!quiet.needs_attention());
}

#[test]
fn cash_and_insurance_are_the_two_covers() {
    assert!(Cover::Insurance.is_insured());
    assert!(!Cover::Cash.is_insured());
    assert_eq!(Cover::Cash.label(), "Cash");
    assert_eq!(Cover::Insurance.label(), "Insurance");
}
