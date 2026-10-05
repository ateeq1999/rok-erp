//! What the clinical check draws, and for which state.
//!
//! The screen is one match over [`CheckState`]. Each arm resolves the figures
//! through the domain's rules and formats them here; the widgets draw what they
//! are handed.

use std::rc::Rc;

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use super::styles::CHECK;
use super::widgets::{approval, counselling, findings, image};
use crate::features::clinical_check::application::check_event::CheckEvent;
use crate::features::clinical_check::application::check_state::CheckState;
use crate::features::clinical_check::domain::calculations;
use crate::features::clinical_check::domain::entities::ClinicalCheck;
use crate::features::clinical_check::domain::enums::{Destination, Severity};
use crate::features::shared::board;

/// How the screen asks the `BLoC` to do something.
pub(crate) type Dispatch = Rc<dyn Fn(CheckEvent, &mut Window, &mut App)>;

/// Every figure the clinical check's widgets draw, already resolved.
pub(crate) struct Board {
    /// How many medicines are on the paper.
    pub medicines: u32,
    /// How many checks found nothing to act on, and how many there are.
    pub cleared: u32,
    pub total: u32,
    /// How many findings hold the prescription.
    pub alerts: u32,
    /// Whether the call is recorded.
    pub call_recorded: bool,
    /// Whether the check can be approved.
    pub can_approve: bool,
    /// The rail: the scan and the details.
    pub rail: image::RailView,
    /// The medicines, with the batch each will draw from.
    pub medicines_table: Vec<findings::MedicineView>,
    /// The checks.
    pub checks: Vec<findings::CheckView>,
    /// The call's outcomes.
    pub outcomes: Vec<findings::OutcomeView>,
    /// The note for the patient's record.
    pub note: String,
    /// What the patient is told.
    pub counselling: Vec<String>,
    /// The label preview.
    pub label: Option<approval::LabelView>,
    /// When the paper was issued.
    pub issued: String,
    /// Who is checking.
    pub pharmacist: String,
    /// Their initials.
    pub initials: String,
}

/// How much attention a finding is asking for, in the shell's tones.
pub(crate) fn tone_for(severity: Severity) -> Tone {
    match severity {
        Severity::Clear => Tone::Success,
        Severity::Alert => Tone::Danger,
    }
}

/// Where each destination opens, which no other layer knows.
pub(crate) fn route(destination: Destination) -> &'static str {
    match destination {
        Destination::PatientRecord => "/patients",
        Destination::Prescriptions => "/prescriptions",
        Destination::Till => "/till",
    }
}

/// Every figure the check shows, worked out through the domain's rules.
pub(crate) fn board(check: &ClinicalCheck) -> Board {
    let label = calculations::preview(check).map(|medicine| approval::LabelView {
        patient: check.patient.to_string(),
        headline: format!("{} \u{d7} {}", medicine.name, medicine.quantity),
        words: medicine.label_words.to_string(),
        batch: medicine.batch.to_string(),
        labels: calculations::labels(check),
    });
    Board {
        medicines: u32::try_from(check.medicines.len()).unwrap_or(u32::MAX),
        cleared: calculations::cleared(check),
        total: calculations::checks_total(check),
        alerts: u32::try_from(calculations::blocking(check).len()).unwrap_or(u32::MAX),
        call_recorded: calculations::outcome(check).is_some(),
        can_approve: calculations::can_approve(check),
        rail: image::RailView {
            reference: check.reference.to_string(),
            patient: check.patient.to_string(),
            age: check.age.to_string(),
            prescriber: check.prescriber.to_string(),
            clinic: check.clinic.to_string(),
            issued: check.issued.to_string(),
            scanned: check.scanned.to_string(),
        },
        medicines_table: check
            .medicines
            .iter()
            .map(|medicine| findings::MedicineView {
                name: medicine.name.to_string(),
                quantity: medicine.quantity.to_string(),
                batch: medicine.batch.to_string(),
                detail: board::dotted(&[
                    &medicine.directions,
                    &medicine.supply,
                    &format!("exp {}", medicine.expiry),
                ]),
            })
            .collect(),
        checks: check
            .checks
            .iter()
            .map(|one| findings::CheckView {
                label: one.label.to_string(),
                detail: one.detail.to_string(),
                result: one.result.to_string(),
                tone: tone_for(one.finding.severity()),
                alert: one.finding.blocks(),
            })
            .collect(),
        outcomes: check
            .outcomes
            .iter()
            .map(|outcome| findings::OutcomeView {
                label: outcome.label.to_string(),
                chosen: outcome.chosen,
            })
            .collect(),
        note: check.note.to_string(),
        counselling: check.counselling.iter().map(ToString::to_string).collect(),
        label,
        issued: check.issued.to_string(),
        pharmacist: check.pharmacist.to_string(),
        initials: check.initials.to_string(),
    }
}

/// The board while the prescription is on its way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&CHECK.notice)
            .child(div().sx(&CHECK.notice_title).child("Reading the check"))
            .child(
                div()
                    .sx(&CHECK.notice_body)
                    .child("Interactions, allergies and dose against the rule tables."),
            ),
    )
}

/// The board when the prescription could not be read.
fn failed(message: &str, dispatch: &Dispatch) -> Div {
    let retry = dispatch.clone();
    let retry_button = Button::new("check-retry")
        .label("Try again")
        .on_click(move |_, window, cx| retry(CheckEvent::Retry, window, cx));
    div().sx(board::root()).child(
        div()
            .sx(&CHECK.notice)
            .child(
                div()
                    .sx(&CHECK.notice_title)
                    .child("This prescription could not be read"),
            )
            .child(div().sx(&CHECK.notice_body).child(message.to_string()))
            .child(retry_button),
    )
}

/// The board for a check that is on screen.
fn content(board: &Board, refreshing: bool, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    let mode = board::mode(cx);
    let refresh = dispatch.clone();
    let refresh_button = Button::new("check-refresh")
        .label(if refreshing { "Refreshing" } else { "Refresh" })
        .on_click(move |_, window, cx| refresh(CheckEvent::Refresh, window, cx));
    div()
        .sx(board::root())
        .child(board::stat_row(vec![
            board::stat(
                "Medicines",
                board.medicines.to_string(),
                "read off the paper",
                None,
                mode,
            ),
            board::stat(
                "Checks clear",
                format!("{}/{}", board.cleared, board.total),
                "against the rule tables",
                Some(Tone::Success),
                mode,
            ),
            board::stat(
                "Alerts",
                board.alerts.to_string(),
                "each one needs a recorded call",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Call recorded",
                if board.call_recorded { "Yes" } else { "No" }.to_string(),
                "the prescriber's answer, on the record",
                if board.can_approve {
                    Some(Tone::Brand)
                } else {
                    Some(Tone::Warning)
                },
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(image::card(&board.rail))
                .child(
                    board::column()
                        .child(findings::medicines(&board.medicines_table))
                        .child(findings::checks(&board.checks, board.cleared, mode))
                        .child(findings::call(
                            &board.outcomes,
                            &board.rail.clinic,
                            &board.note,
                            mode,
                        )),
                )
                .child(
                    board::column()
                        .child(counselling::card(&board.counselling))
                        .child(match &board.label {
                            Some(label) => approval::label(label, &board.issued, &board.initials),
                            None => board::card(1.)
                                .child(board::meta("No medicine on this prescription")),
                        })
                        .child(approval::approval(&board.pharmacist, board.can_approve)),
                ),
        )
        .child(board::actions(vec![refresh_button.into_any_element()]))
}

/// The check for the state it is in.
pub(crate) fn draw(state: &CheckState, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    match state {
        CheckState::Initial | CheckState::Loading => loading(),
        CheckState::Refreshing { check } => content(&board(check), true, dispatch, cx),
        CheckState::Loaded { check } => content(&board(check), false, dispatch, cx),
        CheckState::Error { message } => failed(message, dispatch),
    }
}
