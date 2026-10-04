//! A patient's record: their conditions, their medicines, their dispensing
//! history.
//!
//! Board: `design/pharmacy/Pharmacy_patient_record.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The list a search for a patient starts from.
#[must_use]
pub fn list() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Patients",
            "Search by name, phone or file number",
        ))
        .child(Placeholder::new(Board::PatientRecord))
}

/// The record of `patient_id`.
#[must_use]
pub fn record(patient_id: &str) -> impl IntoElement {
    div()
        .child(PageHeader::new(
            format!("Patient {patient_id}"),
            "Conditions, allergies, current medicines and dispensing history",
        ))
        .child(Placeholder::new(Board::PatientRecord))
}
