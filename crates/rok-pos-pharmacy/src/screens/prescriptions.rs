//! The prescription queue: signed, waiting for check, ready to dispense.
//!
//! Board: `design/pharmacy/Pharmacy_prescription_queue.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The queue, the selected prescription and its history.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Prescriptions",
            "Afya Pharmacy - Mwenge branch",
        ))
        .child(Placeholder::new(Board::PrescriptionQueue))
}
