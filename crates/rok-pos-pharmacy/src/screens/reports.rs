//! Reports with the pharmacy's filters: sales by medicine and schedule, margin,
//! expiry losses, claims aging and controlled movements.
//!
//! Board: `OfficeReports`, built in Phase 13.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The reports list.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Reports",
            "Sales, margin, expiry losses, claims aging and controlled movements",
        ))
        .child(Placeholder::new(Board::Reports))
}
