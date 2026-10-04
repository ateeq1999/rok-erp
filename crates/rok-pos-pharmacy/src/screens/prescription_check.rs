//! The pharmacist's check of one prescription: interactions, allergies, dose
//! and the label to print.
//!
//! Board: `design/pharmacy/Pharmacy_clinical_check_label.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The check for `prescription_id`, a code like `RX-2214`.
#[must_use]
pub fn view(prescription_id: &str) -> impl IntoElement {
    div()
        .child(PageHeader::new(
            format!("Check {prescription_id} before dispensing"),
            "Interactions, allergies and dose against the rule tables",
        ))
        .child(Placeholder::new(Board::ClinicalCheckLabel))
}
