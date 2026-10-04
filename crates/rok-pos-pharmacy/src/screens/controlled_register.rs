//! The controlled medicines register: a running balance per substance, per
//! patient, with the count variance at the end of the day.
//!
//! Board: `VerticalPharmacyRegister`, which this export of the design leaves
//! out; the plan names it in Phase 8.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The register, and the balance a count has to agree with.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Controlled register",
            "Running balance per substance, per patient",
        ))
        .child(Placeholder::new(Board::VerticalControlledRegister))
}
