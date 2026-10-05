//! The prescription queue: every prescription received today, what state it is
//! in, and the one the pharmacist has open.
//!
//! Board: `Pharmacy_prescription_queue.html`. The board's own queue lives in
//! [`data::story`]; Phase 4 replaces it with the day's prescriptions from the
//! database.

pub mod application;
pub mod data;
pub mod domain;
pub(crate) mod presentation;

#[cfg(test)]
mod tests;

pub use presentation::prescriptions_page::PrescriptionsPage;
