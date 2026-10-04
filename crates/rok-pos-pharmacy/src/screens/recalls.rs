//! Batch recalls: what a supplier or the authority has recalled, and which
//! batches of it the pharmacy holds.
//!
//! Board: `design/pharmacy/Pharmacy_batch_recall.html`.

use rok_pos_shell::PageHeader;
use rok_ui::prelude::*;

use crate::screens::placeholder::{Board, Placeholder};

/// The open recalls, and the ones already closed.
#[must_use]
pub fn list() -> impl IntoElement {
    div()
        .child(PageHeader::new(
            "Recalls",
            "Open recalls and the batches they cover",
        ))
        .child(Placeholder::new(Board::BatchRecall))
}

/// One recall, `recall_id` a code like `RC-0047`.
#[must_use]
pub fn view(recall_id: &str) -> impl IntoElement {
    div()
        .child(PageHeader::new(
            format!("Recall {recall_id}"),
            "Batches held, customers to tell, returns to book",
        ))
        .child(Placeholder::new(Board::BatchRecall))
}
