//! Ordering medicines from licensed suppliers.
//!
//! Board: `design/pharmacy/Pharmacy_order_from_medicine_suppliers.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// What to order, from whom, and what is already on its way.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Order medicines",
            "Licensed suppliers - delivering to Mwenge",
        ))
        .child(Placeholder::new(Board::OrderFromMedicineSuppliers))
}
