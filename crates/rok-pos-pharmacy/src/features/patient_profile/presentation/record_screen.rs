//! What the patient's record draws, and for which state.
//!
//! The screen is one match over [`RecordState`]. Each arm resolves the figures
//! through the domain's rules and formats them here; the widgets draw what they
//! are handed.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use super::record_widgets::{clinical, current, details, history, notes};
use super::styles::PATIENT;
use crate::features::patient_profile::application::record_state::RecordState;
use crate::features::patient_profile::domain::calculations;
use crate::features::patient_profile::domain::entities::Patient;
use crate::features::shared::board;

/// Every figure the record's widgets draw, already resolved.
pub(crate) struct Board {
    /// The record.
    pub patient: Patient,
    /// The medicines the pharmacy refills on a date it knows.
    pub refills_due: u32,
    /// How many fills the history shows.
    pub fills: u32,
    /// The next medicine due, for the stat tile.
    pub next_due: Option<String>,
}

/// Every figure the record shows, worked out through the domain's rules.
pub(crate) fn board(patient: &Patient) -> Board {
    Board {
        patient: patient.clone(),
        refills_due: calculations::refills_due(patient),
        fills: patient.fill_count(),
        next_due: patient.next_due().map(|medicine| medicine.name.to_string()),
    }
}

/// The record while it is on its way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&PATIENT.notice)
            .child(div().sx(&PATIENT.notice_title).child("Opening the record"))
            .child(
                div()
                    .sx(&PATIENT.notice_body)
                    .child("The patient's medicines, history and notes."),
            ),
    )
}

/// The record when it could not be read.
fn failed(message: &str) -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&PATIENT.notice)
            .child(
                div()
                    .sx(&PATIENT.notice_title)
                    .child("The record could not be opened"),
            )
            .child(div().sx(&PATIENT.notice_body).child(message.to_string())),
    )
}

/// The record for a patient on screen.
fn content(board: &Board, cx: &mut Cx) -> Div {
    let mode = board::mode(cx);
    let patient = &board.patient;
    div()
        .sx(board::root())
        .child(
            div()
                .sx(board::row())
                .child(details::card(patient))
                .child(clinical::card(patient)),
        )
        .child(
            div()
                .sx(board::row())
                .child(current::card(patient))
                .child(history::card(patient))
                .child(notes::card(patient)),
        )
        .child(board::stat_row(vec![
            board::stat(
                "Refill reminders",
                if patient.wants_reminders() {
                    "On"
                } else {
                    "Off"
                }
                .to_string(),
                patient.consent.to_string(),
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Medicines to refill",
                board.refills_due.to_string(),
                "the ones the pharmacy counts the days for",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Next refill due",
                board.next_due.clone().unwrap_or_else(|| "None".to_string()),
                "the reason this record was opened",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Fills on record",
                board.fills.to_string(),
                "kept for the period the regulator requires",
                None,
                mode,
            ),
        ]))
}

/// The record for the state it is in.
pub(crate) fn draw(state: &RecordState, cx: &mut Cx) -> Div {
    match state {
        RecordState::Initial | RecordState::Loading => loading(),
        RecordState::Loaded { patient } => content(&board(patient), cx),
        RecordState::Error { message } => failed(message),
    }
}
