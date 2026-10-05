//! The label the printer will set out, and the approval that releases it.

use gpui::prelude::*;
use rok_ui::prelude::*;

use crate::features::clinical_check::domain::enums::Destination;
use crate::features::clinical_check::presentation::check_screen::route;
use crate::features::clinical_check::presentation::styles::CHECK;
use crate::screens::board;

/// One medicine on the label preview, as the screen resolves it.
pub(crate) struct LabelView {
    /// The patient the label is for.
    pub patient: String,
    /// The medicine line, as the printer sets it.
    pub headline: String,
    /// The directions in the words the patient is given.
    pub words: String,
    /// The batch the label draws from.
    pub batch: String,
    /// How many labels the printer will be fed for.
    pub labels: usize,
}

/// The label preview, one medicine at a time.
pub(crate) fn label(view: &LabelView, issued: &str, initials: &str) -> Div {
    board::card(1.)
        .child(board::section_label(format!(
            "DOSAGE LABEL PREVIEW \u{b7} 1 OF {}",
            view.labels.max(1)
        )))
        .child(
            div()
                .sx(&CHECK.label)
                .child(board::key_value("Patient", view.patient.clone()))
                .child(board::card_title(view.headline.clone()))
                .child(board::meta(view.words.clone()))
                .child(board::key_value(
                    "Date",
                    format!("{issued} \u{b7} Pharmacist {initials}"),
                )),
        )
        .child(board::footnote(format!(
            "One label per medicine, first expiry first out of {}.",
            view.batch
        )))
}

/// The approval: the pharmacist's PIN, and what happens after it.
pub(crate) fn approval(pharmacist: &str, can_approve: bool) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Pharmacist approval",
            pharmacist.to_string(),
        ))
        .child(board::option(format!("{pharmacist} \u{b7} PIN")))
        .child(board::actions(vec![
            Button::new("check-hold").label("Hold").into_any_element(),
            Button::new("check-record")
                .label("Record outcome")
                .into_any_element(),
        ]))
        .child(
            Button::new("check-approve")
                .label(if can_approve {
                    "Approve for dispensing"
                } else {
                    "Approval needs a recorded call"
                })
                .into_any_element(),
        )
        .child(board::footnote(
            "After approval the prescription moves to the till.",
        ))
        .child(board::link(
            "check-till",
            route(Destination::Till),
            "Open dispensary till",
        ))
}
