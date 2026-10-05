//! The dashboard's widgets: every state the `BLoC` can leave is drawn, and the
//! figures the screen hands them are the ones the board shows.

use std::rc::Rc;

use rok_ui::prelude::*;

use crate::features::dashboard::application::dashboard_state::DashboardState;
use crate::features::dashboard::data::models::DashboardRecord;
use crate::features::dashboard::data::story;
use crate::features::dashboard::domain::entities::Dashboard;
use crate::features::dashboard::domain::enums::Destination;
use crate::features::dashboard::domain::measures::MeasureValue;
use crate::features::dashboard::presentation::dashboard_page::DashboardPage;
use crate::features::dashboard::presentation::dashboard_screen::{
    self, Dispatch, board, measure_text, route,
};

/// A dispatch that records nothing, for a widget drawn on its own.
fn ignore() -> Dispatch {
    Rc::new(|_, _, _| {})
}

/// The board's day, as the domain entity it becomes.
fn day() -> Dashboard {
    Dashboard::from(story::mwenge_day())
}

#[test]
fn the_screen_hands_the_tiles_the_boards_figures() {
    let tiles = board(&day()).tiles;
    let drawn: Vec<(String, String, String)> = tiles
        .iter()
        .map(|tile| (tile.label.clone(), tile.value.clone(), tile.note.clone()))
        .collect();
    assert_eq!(
        drawn,
        [
            (
                "Sales today".to_string(),
                "1,846,200".to_string(),
                "Insurance share 38% \u{b7} 701,600".to_string(),
            ),
            (
                "Prescriptions".to_string(),
                "23".to_string(),
                "23 dispensed \u{b7} 6 waiting \u{b7} oldest 34 min".to_string(),
            ),
            (
                "Expiring in 30 days".to_string(),
                "186,400".to_string(),
                "5 batches at cost \u{b7} 2 expired batches blocked".to_string(),
            ),
            (
                "Claims queried".to_string(),
                "11".to_string(),
                "National health insurance \u{b7} September batch".to_string(),
            ),
        ]
    );
}

#[test]
fn the_widgets_receive_figures_not_calculations() {
    let board = board(&day());
    assert_eq!(board.hours.len(), 8);
    assert!(
        board.hours.last().expect("an hour").in_progress,
        "the last hour is the one still going"
    );
    assert!(
        !board.hours[0].in_progress,
        "the first hour of the day is finished"
    );
    assert_eq!(board.payments[0].percent, 38);
    assert!(board.payments[0].fraction > 0.);
    assert_eq!(board.top_medicines[0].name, "Metformin 500mg tablets");
    assert_eq!(board.top_medicines[0].sales, "50,400");
    assert_eq!(board.branches.len(), 5);
    assert_eq!(board.branch, story::MWENGE);
    assert_eq!(board.other_branch, story::TEGETA);
}

#[test]
fn a_figure_is_written_the_way_the_board_writes_it() {
    use rok_pos_domain::Money;
    assert_eq!(
        measure_text(MeasureValue::Money(Money::from_shillings(1_846_200))),
        "1,846,200"
    );
    assert_eq!(measure_text(MeasureValue::Count(23)), "23");
    assert_eq!(measure_text(MeasureValue::Percent(38)), "38%");
}

#[test]
fn a_day_with_nothing_on_it_still_hands_over_every_figure() {
    let empty = DashboardRecord {
        branch: "Tegeta".to_string(),
        hours: Vec::new(),
        payments: Vec::new(),
        prescriptions_dispensed: 0,
        prescriptions_waiting: 0,
        oldest_waiting_minutes: 0,
        expiring_within_30_days: 0,
        expiring_batches: 0,
        expired_batches_blocked: 0,
        claims_queried: 0,
        claims_queried_in: "nothing queried".to_string(),
        average_basket: 0,
        tasks: Vec::new(),
        top_medicines: Vec::new(),
        other_branch: crate::features::dashboard::data::models::OtherBranchRecord {
            name: "Mwenge".to_string(),
            sales: 0,
            prescriptions_dispensed: 0,
            insurance_share_percent: 0,
            average_basket: 0,
            prescriptions_waiting: 0,
        },
    };
    let board = board(&Dashboard::from(empty));
    assert_eq!(board.hours.len(), 0);
    assert_eq!(board.tasks.len(), 0);
    assert_eq!(board.payments.len(), 0);
    assert_eq!(board.branches[0].here, "0");
}

#[test]
fn every_destination_opens_a_screen_the_pharmacy_has() {
    for destination in [
        Destination::Reports,
        Destination::Prescriptions,
        Destination::PrescriptionCheck,
        Destination::Batches,
        Destination::Claims,
        Destination::Recall,
        Destination::Receive,
        Destination::Licences,
        Destination::ControlledRegister,
    ] {
        let path = route(destination);
        assert!(path.starts_with('/'), "{path} is not a path");
    }
}

struct Windowed(DashboardState);

impl Render for Windowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.0.clone();
        let dispatch = ignore();
        let mut cx = Cx::new(window, cx);
        dashboard_screen::draw(&state, &dispatch, &mut cx)
    }
}

struct Page;

impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        DashboardPage::new()
    }
}

/// Every state the `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let dashboard = day();
    let states = [
        DashboardState::Initial,
        DashboardState::Loading,
        DashboardState::Refreshing {
            dashboard: dashboard.clone(),
        },
        DashboardState::Loaded {
            dashboard: dashboard.clone(),
        },
        DashboardState::Error {
            message: "The dashboard could not be read.".to_string(),
        },
    ];
    for state in states {
        let (_view, window) = cx.add_window_view(|_, _| Windowed(state.clone()));
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}

/// The whole page, dispatched and drawn, which is the only way the feature is
/// reached from outside it.
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
}
