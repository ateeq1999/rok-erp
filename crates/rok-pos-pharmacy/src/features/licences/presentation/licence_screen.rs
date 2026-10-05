//! What the licences screen draws, and for which state.
//!
//! The screen is one match over [`LicenceState`]. Each arm resolves the
//! figures through the domain's rules and formats them here; the cards draw
//! what they are handed.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;
use rust_i18n::t;

use super::styles::LICENCE;
use crate::features::licences::application::licence_event::LicenceEvent;
use crate::features::licences::application::licence_state::LicenceState;
use crate::features::licences::domain::calculations::{self, BOARD_TODAY};
use crate::features::licences::domain::entities::{CivilDate, Licences};
use crate::features::licences::domain::enums::{Reminder, Standing};
use crate::features::shared::board;

/// How the screen asks the `BLoC` to do something.
pub(crate) type Dispatch = std::rc::Rc<dyn Fn(LicenceEvent, &mut Window, &mut App)>;

/// Every figure the screen shows, already resolved from the folder.
pub(crate) struct Board {
    /// The folder itself: the cards draw from it.
    pub licences: Licences,
    /// How many licences the pharmacy holds.
    pub held: u32,
    /// How many licences the pharmacy has to act on.
    pub renewing: u32,
    /// How many of the inspector's requirements are satisfied.
    pub ready: u32,
    /// How many requirements there are.
    pub total: u32,
    /// How many documents the folder is missing.
    pub missing: u32,
    /// How many days the lead renewal has left.
    pub days_left: i32,
}

/// Every figure the screen shows, worked out through the domain's rules.
pub(crate) fn board(licences: &Licences) -> Board {
    Board {
        held: calculations::held(&licences.licences),
        renewing: calculations::renewing(&licences.licences),
        ready: calculations::ready(&licences.readiness),
        total: calculations::total(&licences.readiness),
        missing: calculations::missing_documents(&licences.documents),
        days_left: calculations::days_until_renewal(licences, BOARD_TODAY).unwrap_or_default(),
        licences: licences.clone(),
    }
}

/// A date as the folder spells it: `30 Nov 2026`.
pub(crate) fn day(date: CivilDate) -> String {
    let month = match date.month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        _ => "Dec",
    };
    format!("{} {month} {}", date.day, date.year)
}

/// What the board's chip says for a standing.
fn standing_label(standing: Standing) -> String {
    match standing {
        Standing::Valid => t!("licence.held.valid").to_string(),
        Standing::Renewing => t!("licence.held.renew_soon").to_string(),
        Standing::Missing => t!("licence.held.not_held").to_string(),
    }
}

/// How the board's chip looks for a standing.
fn standing_tone(standing: Standing) -> Tone {
    match standing {
        Standing::Valid => Tone::Success,
        Standing::Renewing => Tone::Warning,
        Standing::Missing => Tone::Danger,
    }
}

/// When the folder reminds the pharmacy about a document.
fn reminder_label(reminder: Reminder) -> String {
    match reminder {
        Reminder::DaysBefore(days) => {
            t!("licence.documents.reminder_days", days = days).to_string()
        }
        Reminder::SubmitBy => t!("licence.documents.reminder_submit").to_string(),
    }
}

/// The board's headline: the renewal with the days left on it.
fn renewal_alert(board: &Board, mode: ThemeMode) -> Option<Div> {
    let renewal = board.licences.renewal()?;
    let title = if board.days_left >= 0 {
        t!(
            "licence.alert.renewal_due_in",
            title = renewal.title.to_string(),
            days = board.days_left,
        )
        .to_string()
    } else {
        t!(
            "licence.alert.lapsed",
            title = renewal.title.to_string(),
            days = board.days_left.abs(),
        )
        .to_string()
    };
    let issued = t!(
        "licence.alert.issued",
        issuer = renewal.issuer.to_string(),
        branch = renewal.branch.to_string(),
        expires = day(renewal.expires_on),
    )
    .to_string();
    let deadline = renewal.renewal_application_due.map(|due| {
        t!(
            "licence.alert.submit_by",
            deadline = day(due),
            days = board.days_left,
        )
        .to_string()
    });
    let lines = match deadline {
        Some(deadline) => vec![issued, deadline],
        None => vec![
            issued,
            t!("licence.alert.days_left", days = board.days_left).to_string(),
        ],
    };
    let mut lines = lines;
    lines.push(t!("licence.alert.note").to_string());
    Some(board::alert(
        title,
        board::dotted(&lines.iter().map(String::as_str).collect::<Vec<_>>()),
        Tone::Warning,
        mode,
    ))
}

/// The folder while it is on its way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&LICENCE.notice)
            .child(
                div()
                    .sx(&LICENCE.notice_title)
                    .child(t!("licence.loading.title").to_string()),
            )
            .child(
                div()
                    .sx(&LICENCE.notice_body)
                    .child(t!("licence.loading.body").to_string()),
            ),
    )
}

/// The folder when it could not be read.
fn failed(message: &str, dispatch: &Dispatch) -> Div {
    let retry = dispatch.clone();
    let retry_button = Button::new("licences-retry")
        .label(t!("common.retry").to_string())
        .on_click(move |_, window, cx| retry(LicenceEvent::Retry, window, cx));
    div().sx(board::root()).child(
        div()
            .sx(&LICENCE.notice)
            .child(
                div()
                    .sx(&LICENCE.notice_title)
                    .child(t!("licence.failed.title").to_string()),
            )
            .child(div().sx(&LICENCE.notice_body).child(message.to_string()))
            .child(retry_button),
    )
}

/// The licences the pharmacy holds.
fn held(board: &Board) -> Div {
    let rows = board
        .licences
        .licences
        .iter()
        .map(|licence| {
            board::line(vec![
                board::cell_stack(licence.title.to_string(), licence.issuer.to_string()),
                board::cell(licence.branch.to_string()),
                board::cell(licence.number.to_string()),
                board::cell_fixed(day(licence.expires_on)),
                board::chip(
                    standing_label(licence.standing),
                    standing_tone(licence.standing),
                )
                .into_any_element(),
            ])
        })
        .collect::<Vec<_>>();
    board::card(1.)
        .child(board::card_head(
            t!("licence.held.title").to_string(),
            t!("licence.held.meta").to_string(),
        ))
        .child(board::head(vec![
            board::cell(t!("licence.held.col.licence").to_string()),
            board::cell(t!("licence.held.col.branch").to_string()),
            board::cell(t!("licence.held.col.number").to_string()),
            board::cell_fixed(t!("licence.held.col.expires").to_string()),
            board::cell(t!("licence.held.col.state").to_string()),
        ]))
        .children(rows)
}

/// What an inspector will ask for, which is the list that actually matters.
fn readiness(board: &Board, mode: ThemeMode) -> Div {
    let rows = board
        .licences
        .readiness
        .iter()
        .map(|requirement| {
            let mark = if requirement.ready {
                t!("licence.readiness.ready").to_string()
            } else {
                t!("licence.readiness.missing").to_string()
            };
            let tone = if requirement.ready {
                Tone::Success
            } else {
                Tone::Danger
            };
            board::line(vec![
                board::chip(mark, tone).into_any_element(),
                board::cell_stack(requirement.name.to_string(), requirement.detail.to_string()),
            ])
        })
        .collect::<Vec<_>>();
    board::card(1.)
        .child(board::card_head(
            t!("licence.readiness.title").to_string(),
            t!(
                "licence.readiness.meta",
                missing = board.total - board.ready,
                total = board.total,
            )
            .to_string(),
        ))
        .children(rows)
        .child(board::actions(vec![
            Button::new("licences-renew")
                .label(t!("licence.readiness.start_renewal").to_string())
                .into_any_element(),
            Button::new("licences-export")
                .label(t!("licence.readiness.export").to_string())
                .into_any_element(),
        ]))
        .child(board::alert(
            t!("licence.readiness.folder_alert_title").to_string(),
            t!("licence.readiness.folder_alert_body").to_string(),
            Tone::Info,
            mode,
        ))
}

/// The documents themselves, and whether the file is actually there.
fn documents(board: &Board) -> Div {
    let rows = board
        .licences
        .documents
        .iter()
        .map(|document| {
            let expires = document.expires_on.map(day).unwrap_or_default();
            board::line(vec![
                board::cell(document.name.to_string()),
                board::cell(document.branch.to_string()),
                board::cell_fixed(expires),
                board::cell(reminder_label(document.reminder)),
                board::chip(
                    if document.on_file {
                        t!("licence.documents.on_file").to_string()
                    } else {
                        t!("licence.documents.missing").to_string()
                    },
                    if document.on_file {
                        Tone::Success
                    } else {
                        Tone::Danger
                    },
                )
                .into_any_element(),
            ])
        })
        .collect::<Vec<_>>();
    board::card(1.)
        .child(board::card_head(
            t!("licence.documents.title").to_string(),
            t!("licence.documents.meta", missing = board.missing).to_string(),
        ))
        .child(board::head(vec![
            board::cell(t!("licence.documents.col.document").to_string()),
            board::cell(t!("licence.documents.col.branch").to_string()),
            board::cell_fixed(t!("licence.documents.col.expires").to_string()),
            board::cell(t!("licence.documents.col.reminder").to_string()),
            board::cell(t!("licence.documents.col.file").to_string()),
        ]))
        .children(rows)
}

/// The folder for a reading on screen.
fn content(board: &Board, cx: &mut Cx) -> Div {
    let mode = board::mode(cx);
    let mut root = div().sx(board::root());
    if let Some(alert) = renewal_alert(board, mode) {
        root = root.child(alert);
    }
    root.child(board::stat_row(vec![
        board::stat(
            t!("licence.stat.held").to_string(),
            board.held.to_string(),
            t!("licence.stat.held_note").to_string(),
            None,
            mode,
        ),
        board::stat(
            t!("licence.stat.renewing").to_string(),
            board.renewing.to_string(),
            t!("licence.stat.renewing_note").to_string(),
            Some(Tone::Warning),
            mode,
        ),
        board::stat(
            t!("licence.stat.ready").to_string(),
            format!("{}/{}", board.ready, board.total),
            t!("licence.stat.ready_note").to_string(),
            Some(Tone::Brand),
            mode,
        ),
        board::stat(
            t!("licence.stat.missing").to_string(),
            board.missing.to_string(),
            t!("licence.stat.missing_note").to_string(),
            Some(Tone::Danger),
            mode,
        ),
    ]))
    .child(held(board))
    .child(
        div()
            .sx(board::row())
            .child(readiness(board, mode))
            .child(documents(board)),
    )
    .child(board::footnote(t!("licence.footnote").to_string()))
}

/// The folder for the state it is in.
pub(crate) fn draw(state: &LicenceState, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    match state {
        LicenceState::Initial | LicenceState::Loading => loading(),
        LicenceState::Loaded { licences } => content(&board(licences), cx),
        LicenceState::Error { message } => failed(message, dispatch),
    }
}
