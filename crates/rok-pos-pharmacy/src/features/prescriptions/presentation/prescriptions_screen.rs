//! What the prescription queue draws, and for which state.
//!
//! The screen is one match over [`PrescriptionsState`]. Each arm resolves the
//! counts through the domain's rules and formats them here; the widgets draw
//! what they are handed.

use std::rc::Rc;

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use super::styles::QUEUE;
use super::widgets::{add_prescription, detail_rail, queue_table, status_boxes};
use crate::features::prescriptions::application::prescriptions_event::PrescriptionsEvent;
use crate::features::prescriptions::application::prescriptions_state::PrescriptionsState;
use crate::features::prescriptions::domain::calculations;
use crate::features::prescriptions::domain::entities::PrescriptionQueue;
use crate::features::prescriptions::domain::enums::{Destination, StatusSeverity};
use crate::features::shared::board;

/// How the screen asks the `BLoC` to do something.
pub(crate) type Dispatch = Rc<dyn Fn(PrescriptionsEvent, &mut Window, &mut App)>;

/// Every figure the queue's widgets draw, already resolved.
pub(crate) struct Board {
    /// The range of references, for the table's meta line.
    pub range: String,
    /// How long the oldest has waited.
    pub oldest: String,
    /// How many are still to be dispensed.
    pub waiting: u32,
    /// How many carry at least one flag.
    pub flagged: u32,
    /// How many have left the pharmacy today.
    pub dispensed: u32,
    /// The five status boxes: label, count, note and colour.
    pub boxes: Vec<(String, u32, String, Tone)>,
    /// The table's rows.
    pub rows: Vec<queue_table::RowView>,
    /// Which reference the rail is on.
    pub selected: String,
    /// The rail.
    pub rail: detail_rail::RailView,
}

/// How much attention a state is asking for, in the shell's tones.
pub(crate) fn tone_for(severity: StatusSeverity) -> Tone {
    match severity {
        StatusSeverity::Info => Tone::Info,
        StatusSeverity::Warning => Tone::Warning,
        StatusSeverity::Danger => Tone::Danger,
        StatusSeverity::Brand => Tone::Brand,
        StatusSeverity::Done => Tone::Success,
    }
}

/// Where each destination opens, which no other layer knows.
pub(crate) fn route(destination: Destination) -> &'static str {
    match destination {
        Destination::Check => "/prescriptions/RX-2214/check",
        Destination::PatientRecord => "/patients",
    }
}

/// Every figure the queue shows, worked out through the domain's rules.
pub(crate) fn board(queue: &PrescriptionQueue) -> Board {
    let open = &queue.open;
    Board {
        range: queue.range.to_string(),
        oldest: queue.oldest_waited.to_string(),
        waiting: calculations::waiting(queue),
        flagged: calculations::flagged(queue),
        dispensed: calculations::dispensed(queue),
        boxes: calculations::status_counts(queue)
            .into_iter()
            .map(|count| {
                (
                    count.status.label().to_string(),
                    count.count,
                    note_for(count.status, count.count).to_string(),
                    tone_for(count.status.severity()),
                )
            })
            .collect(),
        rows: queue
            .queue
            .iter()
            .map(|queued| queue_table::RowView {
                reference: queued.reference.to_string(),
                received: queued.received.to_string(),
                patient: queued.patient.to_string(),
                medicines: queued.medicines.to_string(),
                source: queued.source.label().to_string(),
                status: queued.status.label().to_string(),
                tone: tone_for(queued.status.severity()),
                flags: queued.flags.iter().map(ToString::to_string).collect(),
                flag_is_alert: queued.status
                    == crate::features::prescriptions::domain::enums::Status::NeedsCheck,
            })
            .collect(),
        selected: queue.selected.to_string(),
        rail: detail_rail::RailView {
            reference: open.reference.to_string(),
            arrived: open.arrived.to_string(),
            status: open.status.label().to_string(),
            tone: tone_for(open.status.severity()),
            patient: open.patient.to_string(),
            patient_detail: open.patient_detail.to_string(),
            alert: open
                .alert
                .as_ref()
                .map(|(title, body)| (title.to_string(), body.to_string())),
            items: open
                .items
                .iter()
                .map(|item| detail_rail::ItemView {
                    name: item.name.to_string(),
                    quantity: item.quantity,
                    directions: item.directions.to_string(),
                    availability: item.availability.to_string(),
                    in_stock: item.in_stock,
                })
                .collect(),
            also_takes: open.also_takes.to_string(),
            allergies: open.allergies.to_string(),
            paid_by: open.paid_by.to_string(),
            prescriber: open.prescriber.to_string(),
            rule: open.rule.to_string(),
            needs_a_pharmacist: open.needs_a_pharmacist(),
        },
    }
}

/// The line under a status box, in the board's own words.
fn note_for(
    status: crate::features::prescriptions::domain::enums::Status,
    count: u32,
) -> &'static str {
    use crate::features::prescriptions::domain::enums::Status;
    match status {
        Status::New => "to read and enter",
        Status::NeedsCheck => "1 interaction, 1 recall flag",
        Status::WaitingPrescriber => "call-back due 15:30",
        Status::ReadyToCollect => "patient texted",
        Status::Dispensed => {
            if count == 0 {
                "none today"
            } else {
                "last at 14:05"
            }
        }
    }
}

/// The board while the day's queue is on its way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&QUEUE.notice)
            .child(
                div()
                    .sx(&QUEUE.notice_title)
                    .child("Loading today's prescriptions"),
            )
            .child(
                div()
                    .sx(&QUEUE.notice_body)
                    .child("Every prescription received today, waiting first."),
            ),
    )
}

/// The board when the queue could not be read.
fn failed(message: &str, dispatch: &Dispatch) -> Div {
    let retry = dispatch.clone();
    let retry_button = Button::new("queue-retry")
        .label("Try again")
        .on_click(move |_, window, cx| retry(PrescriptionsEvent::Retry, window, cx));
    div().sx(board::root()).child(
        div()
            .sx(&QUEUE.notice)
            .child(
                div()
                    .sx(&QUEUE.notice_title)
                    .child("Today's queue could not be read"),
            )
            .child(div().sx(&QUEUE.notice_body).child(message.to_string()))
            .child(retry_button),
    )
}

/// The board for a day that is on screen.
fn content(board: &Board, refreshing: bool, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    let mode = board::mode(cx);
    let refresh = dispatch.clone();
    let refresh_button = Button::new("queue-refresh")
        .label(if refreshing { "Refreshing" } else { "Refresh" })
        .on_click(move |_, window, cx| refresh(PrescriptionsEvent::Refresh, window, cx));
    div()
        .sx(board::root())
        .child(board::stat_row(vec![
            board::stat(
                "Waiting",
                board.waiting.to_string(),
                board.oldest.clone(),
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Flagged for the pharmacist",
                board.flagged.to_string(),
                "interaction, recall or unread photo",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Dispensed today",
                board.dispensed.to_string(),
                "last at 14:05",
                Some(Tone::Success),
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .children(board.boxes.iter().map(|(label, count, note, tone)| {
                    status_boxes::card(label, *count, note, *tone, mode)
                }))
                .child(add_prescription::card()),
        )
        .child(
            div()
                .sx(board::row())
                .child(queue_table::card(
                    &board.range,
                    board.waiting,
                    &board.oldest,
                    &board.rows,
                    &board.selected,
                    dispatch,
                ))
                .child(detail_rail::card(&board.rail, mode)),
        )
        .child(board::actions(vec![refresh_button.into_any_element()]))
}

/// The queue for the state it is in.
pub(crate) fn draw(state: &PrescriptionsState, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    match state {
        PrescriptionsState::Initial | PrescriptionsState::Loading => loading(),
        PrescriptionsState::Refreshing { queue } => content(&board(queue), true, dispatch, cx),
        PrescriptionsState::Loaded { queue } => content(&board(queue), false, dispatch, cx),
        PrescriptionsState::Error { message } => failed(message, dispatch),
    }
}
