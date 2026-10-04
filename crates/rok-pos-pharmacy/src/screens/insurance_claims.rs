//! Insurance claims: the split between insurer and patient, and the queries to
//! answer.
//!
//! Board: `design/pharmacy/Pharmacy_insurance_claims.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The claims, their status, and what each insurer still owes.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Insurance claims",
            "Mwenge branch - all insurers",
        ))
        .child(Placeholder::new(Board::InsuranceClaims))
}
