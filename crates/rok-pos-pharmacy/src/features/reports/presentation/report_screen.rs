//! What the reports screen draws, and for which state.
//!
//! The screen is one match over [`ReportState`]. Its rule is the board's: a
//! heading is a sum of its own rows, so the cards draw from the entities and
//! the stat row resolves its figures through [`Kind`], never from a position
//! in a list. Story figures - medicine names, schedule names, the notes - are
//! data and come from the folder; the board's own furniture speaks through
//! `rust_i18n` in both languages.

use gpui::prelude::*;
use rok_pos_shell::{Tone, group_digits};
use rok_ui::prelude::*;
use rust_i18n::t;

use super::styles::REPORT;
use crate::features::reports::application::report_event::ReportEvent;
use crate::features::reports::application::report_state::ReportState;
use crate::features::reports::domain::entities::{Kind, Report, Reports, Row};
use crate::features::shared::board;

/// How the screen asks the `BLoC` to do something.
pub(crate) type Dispatch = std::rc::Rc<dyn Fn(ReportEvent, &mut Window, &mut App)>;

/// Every figure the screen shows, already resolved from the reports.
pub(crate) struct Board {
    /// The reports themselves: the cards draw from them.
    pub reports: Reports,
    /// What sold in the period, which the board leads with.
    pub sales: i64,
    /// What each schedule left after cost.
    pub margin: i64,
    /// What stock has been, and is about to be, written off.
    pub expiry: i64,
    /// How long the insurers have been sitting on the queries.
    pub claims: i64,
    /// What was sold against the controlled register.
    pub controlled: i64,
}

/// Every figure the screen shows, worked out through the domain's rules.
///
/// A stat that needs a report the source did not hand over resolves to zero,
/// which is what the rows it would sum to add up to; the cards themselves
/// simply do not draw.
pub(crate) fn board(reports: &Reports) -> Board {
    let figure = |kind: Kind| reports.report(kind).map_or(0, |report| report.total);
    Board {
        reports: reports.clone(),
        sales: figure(Kind::Sales),
        margin: figure(Kind::Margin),
        expiry: figure(Kind::Expiry),
        claims: figure(Kind::Claims),
        controlled: figure(Kind::Controlled),
    }
}

/// A share cell draws as the row carries it, or as a dash when there is none.
fn share(row: &Row) -> String {
    row.share.as_ref().map_or_else(
        || String::from("\u{2014}"),
        std::string::ToString::to_string,
    )
}

/// One report as a card: the heading figure, the rows and the note.
fn card(report: &Report, mode: ThemeMode) -> Div {
    let rows = report
        .rows
        .iter()
        .map(|row| {
            let cells = vec![
                board::cell(row.label.to_string()),
                board::cell(row.detail.to_string()),
                board::cell_number(group_digits(row.value)),
                board::cell_fixed(share(row)),
            ];
            if row.flagged {
                board::toned_row(Tone::Warning, mode, cells)
            } else {
                board::line(cells)
            }
        })
        .collect::<Vec<_>>();
    board::card(1.)
        .child(board::card_head(
            report.title.to_string(),
            format!("{} {}", group_digits(report.total), report.unit),
        ))
        .child(board::meta(report.question.to_string()))
        .child(board::head(vec![
            board::cell(t!("reports.col.row").to_string()),
            board::cell(t!("reports.col.detail").to_string()),
            board::cell_fixed(t!("reports.col.value").to_string()),
            board::cell_fixed(t!("reports.col.share").to_string()),
        ]))
        .children(rows)
        .child(board::footnote(report.note.to_string()))
        .child(board::actions(vec![
            Button::new("report-export")
                .label(t!("reports.export").to_string())
                .into_any_element(),
            Button::new("report-print")
                .label(t!("reports.print").to_string())
                .into_any_element(),
        ]))
}

/// The reports while they are on their way.
fn loading() -> Div {
    div().sx(board::root()).child(
        div()
            .sx(&REPORT.notice)
            .child(
                div()
                    .sx(&REPORT.notice_title)
                    .child(t!("reports.loading.title").to_string()),
            )
            .child(
                div()
                    .sx(&REPORT.notice_body)
                    .child(t!("reports.loading.body").to_string()),
            ),
    )
}

/// The reports when they could not be read.
fn failed(message: &str, dispatch: &Dispatch) -> Div {
    let retry = dispatch.clone();
    let retry_button = Button::new("reports-retry")
        .label(t!("common.retry").to_string())
        .on_click(move |_, window, cx| retry(ReportEvent::Retry, window, cx));
    div().sx(board::root()).child(
        div()
            .sx(&REPORT.notice)
            .child(
                div()
                    .sx(&REPORT.notice_title)
                    .child(t!("reports.failed.title").to_string()),
            )
            .child(div().sx(&REPORT.notice_body).child(message.to_string()))
            .child(retry_button),
    )
}

/// The board's middle split: the sales card beside the margin and controlled
/// cards, then the expiry and claims cards below them.
fn cards(screen: &Board, mode: ThemeMode) -> Div {
    let reports = &screen.reports;
    let at = |kind: Kind| {
        reports
            .report(kind)
            .map(|report| card(report, mode).into_any_element())
    };
    div()
        .sx(board::row())
        .children(
            reports
                .sales()
                .map(|report| card(report, mode).into_any_element()),
        )
        .child(
            board::column()
                .children(at(Kind::Margin))
                .children(at(Kind::Controlled)),
        )
        .children(at(Kind::Expiry))
        .children(at(Kind::Claims))
}

/// The board for a reading on screen.
fn content(screen: &Board, cx: &mut Cx) -> Div {
    let mode = board::mode(cx);
    let periods = vec![
        (t!("reports.period.month").to_string(), true),
        (t!("reports.period.last_month").to_string(), false),
        (t!("reports.period.quarter").to_string(), false),
    ];
    let branches = vec![
        (t!("reports.branch.both").to_string(), true),
        (t!("reports.branch.mwenge").to_string(), false),
        (t!("reports.branch.tegeta").to_string(), false),
    ];
    div()
        .sx(board::root())
        .child(
            div()
                .sx(board::row())
                .child(board::filters_in(mode, &periods))
                .child(board::filters_in(mode, &branches)),
        )
        .child(board::stat_row(vec![
            board::stat(
                t!("reports.stat.sales").to_string(),
                group_digits(screen.sales),
                t!("reports.period.month").to_string(),
                None,
                mode,
            ),
            board::stat(
                t!("reports.stat.margin").to_string(),
                group_digits(screen.margin),
                t!("reports.stat.margin_note").to_string(),
                Some(Tone::Success),
                mode,
            ),
            board::stat(
                t!("reports.stat.expiry").to_string(),
                group_digits(screen.expiry),
                t!("reports.stat.expiry_note").to_string(),
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                t!("reports.stat.claims").to_string(),
                group_digits(screen.claims),
                t!("reports.stat.claims_note").to_string(),
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                t!("reports.stat.controlled").to_string(),
                group_digits(screen.controlled),
                t!("reports.stat.controlled_note").to_string(),
                Some(Tone::Info),
                mode,
            ),
        ]))
        .child(cards(screen, mode))
        .child(board::footnote(
            t!(
                "reports.footnote",
                period = t!("reports.period.month").to_string(),
                branch = t!("reports.branch.both").to_string(),
            )
            .to_string(),
        ))
}

/// The reports for the state they are in.
pub(crate) fn draw(state: &ReportState, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    match state {
        ReportState::Initial | ReportState::Loading => loading(),
        ReportState::Loaded { reports } => content(&board(reports), cx),
        ReportState::Error { message } => failed(message, dispatch),
    }
}
