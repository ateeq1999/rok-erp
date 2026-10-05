//! Licences and inspection readiness: what the pharmacy is allowed to do, when
//! each permission runs out, and whether an inspector would find the paperwork.
//!
//! Board: `Pharmacy_licences_amp_inspection.html`. The board's own figures
//! live in [`data::story`]; the phase that stores licence documents reads them
//! from the database.

pub mod application;
pub mod data;
pub mod domain;
pub(crate) mod presentation;

#[cfg(test)]
mod tests;

pub use presentation::licence_page::LicencesPage;
