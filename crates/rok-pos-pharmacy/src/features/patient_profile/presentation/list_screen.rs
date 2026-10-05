//! What the patient list draws, and for which state.
//!
//! The screen is one match over [`ListState`]. Each arm resolves the figures
//! through the domain's rules and formats them here; the widgets draw what they
//! are handed.

use std::rc::Rc;

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use super::list_widgets::directory;
use super::styles::PATIENT;
use crate::features::patient_profile::application::list_event::ListEvent;
use crate::features::patient_profile::application::list_state::ListState;
use crate::features::patient_profile::domain::calculations;
use crate::features::patient_profile::domain::entities::Listing;
use crate::features::shared::board;

/// How the screen asks the `BLoC` to do something.
pub(crate) type Dispatch = Rc<dyn Fn(ListEvent, &mut Window, &mut App)>;

/// Every figure the list's widgets draw, already resolved.
pub(crate) struct Board {
    /// The pharmacy's patients, most urgent first.
    pub listings: Vec<Listing>,
    /// How many the list shows.
    pub listed: u32,
    /// How many are due within the week.
    pub due_this_week: u32,
    /// How many an insurer pays for.
    pub insured: u32,
}

/// Every figure the list shows, worked out through the domain's rules.
pub(crate) fn board(listings: &[Listing]) -> Board {
    Board {
        listed: calculations::listed(listings),
        due_this_week: calculations::due_this_week(listings),
        insured: calculations::insured(listings),
        listings: listings.to_vec(),
    }
}

/// The filters the list offers, as the board draws them.
const FILTERS: [(&str, bool); 4] = [
    ("All patients", true),
    ("Refill due", false),
    ("Insurance", false),
    ("Recent", false),
];

/// The list while it is on its way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&PATIENT.notice)
            .child(div().sx(&PATIENT.notice_title).child("Opening the list"))
            .child(
                div()
                    .sx(&PATIENT.notice_body)
                    .child("Every patient the pharmacy serves, most urgent first."),
            ),
    )
}

/// The list when it could not be read.
fn failed(message: &str, dispatch: &Dispatch) -> Div {
    let retry = dispatch.clone();
    let retry_button = Button::new("patient-list-retry")
        .label("Try again")
        .on_click(move |_, window, cx| retry(ListEvent::Retry, window, cx));
    div().sx(board::root()).child(
        div()
            .sx(&PATIENT.notice)
            .child(
                div()
                    .sx(&PATIENT.notice_title)
                    .child("The list could not be read"),
            )
            .child(div().sx(&PATIENT.notice_body).child(message.to_string()))
            .child(retry_button),
    )
}

/// The list for a directory on screen.
fn content(board: &Board, refreshing: bool, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    let mode = board::mode(cx);
    let refresh = dispatch.clone();
    let refresh_button = Button::new("patient-list-refresh")
        .label(if refreshing { "Refreshing" } else { "Refresh" })
        .on_click(move |_, window, cx| refresh(ListEvent::Refresh, window, cx));
    div()
        .sx(board::root())
        .child(board::stat_row(vec![
            board::stat(
                "Patients on file",
                board.listed.to_string(),
                "Mwenge branch \u{b7} all insurances",
                None,
                mode,
            ),
            board::stat(
                "Refill due in 7 days",
                board.due_this_week.to_string(),
                "the reason to open a record today",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Insurance covered",
                board.insured.to_string(),
                "the rest pay at the counter",
                Some(Tone::Brand),
                mode,
            ),
        ]))
        .child(board::filters_in(mode, &FILTERS))
        .child(directory::card(&board.listings))
        .child(board::footnote(
            "A record is the only place a patient's consent, allergies and pharmacist's notes live.",
        ))
        .child(board::actions(vec![refresh_button.into_any_element()]))
}

/// The list for the state it is in.
pub(crate) fn draw(state: &ListState, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    match state {
        ListState::Initial | ListState::Loading => loading(),
        ListState::Refreshing { listings } => content(&board(listings), true, dispatch, cx),
        ListState::Loaded { listings } => content(&board(listings), false, dispatch, cx),
        ListState::Error { message } => failed(message, dispatch),
    }
}
