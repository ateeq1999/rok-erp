//! The pharmacy dashboard: today's sales, waiting prescriptions, expiring value.
//!
//! Board: `design/pharmacy/Pharmacy_dashboard.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The tiles, the queues and the expiring batches, as the board draws them.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Dashboard",
            "Today: sales, prescriptions waiting, expiring stock and queried claims",
        ))
        .child(Placeholder::new(Board::Dashboard))
}
