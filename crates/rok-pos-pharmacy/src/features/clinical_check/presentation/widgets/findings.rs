//! The findings: the medicines read off the paper, the checks, and the call.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// One medicine as the screen resolves it.
pub(crate) struct MedicineView {
    /// What it is called.
    pub name: String,
    /// How many were prescribed.
    pub quantity: String,
    /// The batch it will be drawn from.
    pub batch: String,
    /// The directions, with the supply and the expiry after them.
    pub detail: String,
}

/// The medicines read off the paper, with the batch each will draw from.
pub(crate) fn medicines(medicines: &[MedicineView]) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Medicines read from the prescription",
            format!("{} lines", medicines.len()),
        ))
        .child(board::head(vec![
            board::cell("Medicine"),
            board::cell_fixed("Qty"),
            board::cell_fixed("Batch"),
        ]))
        .children(medicines.iter().map(|medicine| {
            board::line(vec![
                board::cell_stack(medicine.name.clone(), medicine.detail.clone()),
                board::cell_fixed(medicine.quantity.clone()),
                board::cell_fixed(medicine.batch.clone()),
            ])
        }))
        .child(board::footnote(
            "The batch is chosen now so the label and the shelf agree.",
        ))
}

/// One check as the screen resolves it.
pub(crate) struct CheckView {
    /// What was checked.
    pub label: String,
    /// What was found.
    pub detail: String,
    /// What the chip says.
    pub result: String,
    /// How the finding is coloured.
    pub tone: Tone,
    /// Whether the finding holds the prescription.
    pub alert: bool,
}

/// The checks, with the alert toned the way the board tints its one red row.
pub(crate) fn checks(checks: &[CheckView], cleared: u32, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Pharmacist checks",
            format!("{cleared} of {} clear", checks.len()),
        ))
        .children(checks.iter().map(|check| {
            let cells = vec![
                board::cell_fixed(check.label.clone()),
                board::cell_truncating(check.detail.clone()),
                board::chip(check.result.clone(), check.tone).into_any_element(),
            ];
            if check.alert {
                board::toned_row(check.tone, mode, cells)
            } else {
                board::line(cells)
            }
        }))
        .child(board::footnote(
            "An alert is not a stop on its own: it is a stop until the call is recorded.",
        ))
}

/// One call outcome as the screen resolves it.
pub(crate) struct OutcomeView {
    /// What the call came to.
    pub label: String,
    /// Whether it was recorded.
    pub chosen: bool,
}

/// The call's outcome and the note that goes on the patient's record.
pub(crate) fn call(outcomes: &[OutcomeView], clinic: &str, note: &str, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Prescriber call outcome",
            format!("Called {clinic} at 15:10"),
        ))
        .children(outcomes.iter().map(|outcome| {
            let cells = vec![
                board::chip(
                    if outcome.chosen {
                        "Recorded"
                    } else {
                        "Not chosen"
                    },
                    if outcome.chosen {
                        Tone::Brand
                    } else {
                        Tone::Neutral
                    },
                )
                .into_any_element(),
                board::cell(outcome.label.clone()),
            ];
            if outcome.chosen {
                board::toned_row(Tone::Brand, mode, cells)
            } else {
                board::line(cells)
            }
        }))
        .child(board::section_label("NOTE FOR THE PATIENT RECORD"))
        .child(board::footnote(note.to_string()))
}
