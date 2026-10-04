//! Stock by batch and expiry: first expiry, first out, and the cold chain.
//!
//! Board: `design/pharmacy/Pharmacy_batches_expiry_FEFO_cold_chain.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// Every batch, its expiry, and where it is.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Batches & expiry",
            "First expiry, first out - Mwenge and Tegeta branches",
        ))
        .child(Placeholder::new(Board::BatchesExpiryFefoColdChain))
}
