//! What the patient list draws, and for which state.
//!
//! The screen is one match over [`ListState`]. Each arm resolves the figures
//! through the domain's rules and formats them here; the widgets draw what they
//! are handed.

use std::rc::Rc;

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;
use rust_i18n::t;

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

/// The filters the list offers, as the board draws them, in the session's
/// language.
fn filters() -> [(std::borrow::Cow<'static, str>, bool); 4] {
    [
        (t!("patient.filters.all"), true),
        (t!("patient.filters.refill"), false),
        (t!("patient.filters.insurance"), false),
        (t!("patient.filters.recent"), false),
    ]
}

/// The list while it is on its way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&PATIENT.notice)
            .child(
                div()
                    .sx(&PATIENT.notice_title)
                    .child(t!("patient.list.loading.title").to_string()),
            )
            .child(
                div()
                    .sx(&PATIENT.notice_body)
                    .child(t!("patient.list.loading.body").to_string()),
            ),
    )
}

/// The list when it could not be read.
fn failed(message: &str, dispatch: &Dispatch) -> Div {
    let retry = dispatch.clone();
    let retry_button = Button::new("patient-list-retry")
        .label(t!("common.retry").to_string())
        .on_click(move |_, window, cx| retry(ListEvent::Retry, window, cx));
    div().sx(board::root()).child(
        div()
            .sx(&PATIENT.notice)
            .child(
                div()
                    .sx(&PATIENT.notice_title)
                    .child(t!("patient.list.failed.title").to_string()),
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
        .label(if refreshing {
            t!("patient.list.refreshing").to_string()
        } else {
            t!("patient.list.refresh").to_string()
        })
        .on_click(move |_, window, cx| refresh(ListEvent::Refresh, window, cx));
    div()
        .sx(board::root())
        .child(board::stat_row(vec![
            board::stat(
                t!("patient.list.stat.on_file").to_string(),
                board.listed.to_string(),
                t!(
                    "patient.list.stat.on_file_note",
                    branch = crate::story::BRANCH
                )
                .to_string(),
                None,
                mode,
            ),
            board::stat(
                t!("patient.list.stat.due").to_string(),
                board.due_this_week.to_string(),
                t!("patient.list.stat.due_note").to_string(),
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                t!("patient.list.stat.insured").to_string(),
                board.insured.to_string(),
                t!("patient.list.stat.insured_note").to_string(),
                Some(Tone::Brand),
                mode,
            ),
        ]))
        .child(board::filters_in(mode, &filters()))
        .child(directory::card(&board.listings))
        .child(board::footnote(t!("patient.list.footnote").to_string()))
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
