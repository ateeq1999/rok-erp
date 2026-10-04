//! Licences and inspection readiness: what expires, what the inspector asks
//! for.
//!
//! Board: `design/pharmacy/Pharmacy_licences_amp_inspection.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The licences, their expiry, and the inspection checklist.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Licences & inspection",
            "Mwenge and Tegeta branches",
        ))
        .child(Placeholder::new(Board::LicencesInspection))
}
