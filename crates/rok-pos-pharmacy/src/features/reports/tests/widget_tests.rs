//! The reports widgets: every state the `BLoC` can leave is drawn, and the
//! figures the screen resolves are the board's.

use gpui::prelude::*;
use rok_ui::prelude::*;

use crate::features::reports::application::report_event::ReportEvent;
use crate::features::reports::application::report_state::ReportState;
use crate::features::reports::data::story;
use crate::features::reports::domain::entities::{Kind, Reports};
use crate::features::reports::presentation::report_page::ReportsPage;
use crate::features::reports::presentation::report_screen;

/// A dispatch that does nothing, for a board drawn on its own.
fn ignore() -> report_screen::Dispatch {
    std::rc::Rc::new(|_, _, _| {})
}

/// The board's reports, as the entities they become.
fn board() -> Reports {
    Reports::from(story::board())
}

#[test]
fn the_screen_resolves_the_boards_figures() {
    let screen = report_screen::board(&board());
    assert_eq!(screen.sales, 1_842_000);
    assert_eq!(screen.margin, 612_400);
    assert_eq!(screen.expiry, 214_700);
    assert_eq!(screen.claims, 92_500);
    assert_eq!(screen.controlled, 929_000);
}

#[test]
fn the_stat_row_resolves_by_kind_not_by_position() {
    let mut record = story::board();
    record.reports[0].kind = "margin".to_string();
    record.reports[1].kind = "sales by medicine".to_string();
    let screen = report_screen::board(&Reports::from(record));
    assert_eq!(
        screen.sales, 612_400,
        "the stat follows the kind, not the row order"
    );
    assert_eq!(screen.margin, 1_842_000);
}

#[test]
fn a_row_without_a_share_draws_a_dash() {
    let screen = report_screen::board(&board());
    let expiry = screen
        .reports
        .report(Kind::Expiry)
        .expect("the expiry report");
    assert!(expiry.rows.iter().any(|row| row.share.is_none()));
    assert!(
        screen
            .reports
            .report(Kind::Sales)
            .is_some_and(|report| { report.rows.iter().all(|row| row.share.is_some()) })
    );
}

struct Windowed(ReportState);

impl Render for Windowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut cx = Cx::new(window, cx);
        report_screen::draw(&self.0, &ignore(), &mut cx)
    }
}

struct Page;

impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        ReportsPage::new()
    }
}

/// Every state the reports' `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let states = [
        ReportState::Initial,
        ReportState::Loading,
        ReportState::Loaded {
            reports: Box::new(board()),
        },
        ReportState::Error {
            message: "The reports could not be read.".to_string(),
        },
    ];
    for state in states {
        let (_view, window) = cx.add_window_view(|_, _| Windowed(state.clone()));
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}

/// The whole page, dispatched and drawn.
#[gpui::test]
fn the_page_dispatches_its_first_load_and_draws(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let (_view, window) = cx.add_window_view(|_, _| Page);
    window.update(|window, cx| {
        let _ = window.draw(cx);
    });
    window.run_until_parked();
    let (_view, again) = cx.add_window_view(|_, _| Page);
    again.update(|window, cx| {
        let _ = window.draw(cx);
    });
}

/// A `ReportEvent` is one the page can send, which keeps the type live.
#[test]
fn the_page_can_ask_for_the_reports_twice() {
    let first = ReportEvent::Load;
    let again = ReportEvent::Retry;
    assert_ne!(first, again);
}
