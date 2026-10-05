//! The pharmacist's check of one prescription: what was read off the paper, the
//! checks against the rule tables, the call to the prescriber, the label to
//! print, and the approval that moves it to the till.
//!
//! Board: `Pharmacy_clinical_check_label.html`. The board's own prescription
//! lives in [`data::story`].

pub mod application;
pub mod data;
pub mod domain;
pub(crate) mod presentation;

#[cfg(test)]
mod tests;

pub use presentation::check_page::ClinicalCheckPage;
