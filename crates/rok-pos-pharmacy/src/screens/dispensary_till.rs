//! The dispensary till: sell over the counter, take a split payment, print a
//! label.
//!
//! Board: `VerticalPharmacy`, which this export of the design leaves out; the
//! plan names it in Phase 6.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The basket, the payment split and the label printer's state.
#[must_use]
pub fn view() -> impl IntoElement {
    div()
        .child(PageHeader::new("Till", "Mwenge branch - cash drawer open"))
        .child(Placeholder::new(Board::VerticalDispensary))
}
