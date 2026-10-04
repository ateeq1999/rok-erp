//! The medicine catalogue: what the pharmacy may dispense, and what it does.
//!
//! Board: `design/pharmacy/Pharmacy_medicine_catalogue.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The catalogue, and the detail of one medicine.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Medicines",
            "412 medicines in the catalogue",
        ))
        .child(Placeholder::new(Board::MedicineCatalogue))
}
