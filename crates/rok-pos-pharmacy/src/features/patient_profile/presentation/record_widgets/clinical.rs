//! The patient's allergies and their long-term conditions.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::patient_profile::domain::entities::Patient;
use crate::features::shared::board;

/// The allergies and the conditions.
pub(crate) fn card(patient: &Patient) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Allergies",
            patient.allergies_label.to_string(),
        ))
        .child(board::chip(patient.allergies.to_string(), Tone::Success))
        .child(board::footnote(patient.allergy_source.to_string()))
        .child(board::card_title("From prescriptions on file"))
        .child(board::list(
            patient
                .conditions
                .iter()
                .map(|condition| board::option(condition.name.to_string()).into_any_element())
                .collect::<Vec<_>>(),
        ))
}
