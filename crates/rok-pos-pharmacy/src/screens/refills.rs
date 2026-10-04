//! Refills due this week, and the reminders to send.
//!
//! Board: `design/pharmacy/Pharmacy_refills_due_reminders.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// Who is due, and who has already been told.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Refills due",
            "Chronic patients - next 7 days",
        ))
        .child(Placeholder::new(Board::RefillsDueReminders))
}
