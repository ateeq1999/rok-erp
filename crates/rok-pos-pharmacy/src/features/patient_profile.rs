//! The patient's record: who they are, what they take, what has been dispensed
//! and what the pharmacists have written down, with the list a record is found
//! from.
//!
//! Boards: `Pharmacy_patient_record.html` and the patient list it opens from.
//! The board's own figures live in [`data::story`]; Phase 3 reads the patient
//! from the database.

pub mod application;
pub mod data;
pub mod domain;
pub(crate) mod presentation;

#[cfg(test)]
mod tests;

pub use presentation::list_page::PatientListPage;
pub use presentation::record_page::PatientProfilePage;
