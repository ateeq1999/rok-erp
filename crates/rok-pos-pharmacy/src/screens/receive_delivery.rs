//! Receiving a delivery: the order, the batches that arrived, their expiry and
//! the cold chain readings.
//!
//! Board: `design/pharmacy/Pharmacy_receive_delivery_batch_expiry_cold_chain.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The deliveries waiting to be booked in.
#[must_use]
pub fn list() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Receive delivery",
            "Deliveries booked in but not yet received",
        ))
        .child(Placeholder::new(Board::ReceiveDelivery))
}

/// One delivery, `order_id` a code like `UZ-7781`.
#[must_use]
pub fn view(order_id: &str) -> impl IntoElement {
    div()
        .child(PageHeader::new(
            format!("Receive delivery {order_id}"),
            "Batches, expiry and cold chain readings",
        ))
        .child(Placeholder::new(Board::ReceiveDelivery))
}
