//! "Add prescription": how a prescription can be brought in.

use gpui::prelude::*;
use rok_ui::prelude::*;

use crate::features::prescriptions::presentation::styles::QUEUE;
use crate::features::shared::board;

/// The three ways a prescription arrives.
pub(crate) fn card() -> Div {
    board::card(1.)
        .child(board::card_head("Add prescription", "3 ways in"))
        .child(board::meta("Add by:"))
        .child(
            div().sx(&QUEUE.ways).children(
                [
                    "Scan paper prescription",
                    "Attach WhatsApp photo",
                    "Fetch e-prescription by code",
                ]
                .into_iter()
                .map(|way| div().sx(&QUEUE.way).child(way.to_string())),
            ),
        )
}
