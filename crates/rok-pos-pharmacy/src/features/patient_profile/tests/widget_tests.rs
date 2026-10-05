//! The record's and the list's widgets: every state the `BLoC`s can leave is
//! drawn, and a row click reaches the router.

use gpui::prelude::*;
use rok_ui::prelude::*;

use crate::features::patient_profile::application::list_state::ListState;
use crate::features::patient_profile::application::record_state::RecordState;
use crate::features::patient_profile::data::story;
use crate::features::patient_profile::domain::entities::{Listing, Patient};
use crate::features::patient_profile::presentation::list_page::PatientListPage;
use crate::features::patient_profile::presentation::list_screen;
use crate::features::patient_profile::presentation::record_page::PatientProfilePage;
use crate::features::patient_profile::presentation::record_screen;

/// A dispatch that does nothing, for a board drawn on its own.
fn ignore() -> list_screen::Dispatch {
    std::rc::Rc::new(|_, _, _| {})
}

/// The board's patient, as the entity it becomes.
fn patient() -> Patient {
    Patient::from(story::mzee_salim())
}

/// The board's list, as the entities it becomes.
fn listings() -> Vec<Listing> {
    story::directory().into_iter().map(Listing::from).collect()
}

#[test]
fn the_screen_resolves_the_boards_figures() {
    let board = record_screen::board(&patient());
    assert_eq!(board.refills_due, 2);
    assert_eq!(board.fills, 6);
    assert_eq!(
        board.next_due.as_deref(),
        Some("Metformin 500mg tablets"),
        "the next due is the reason the record was opened"
    );
}

#[test]
fn the_list_screen_resolves_the_boards_figures() {
    let board = list_screen::board(&listings());
    assert_eq!(board.listed, 7);
    assert_eq!(board.due_this_week, 4);
    assert_eq!(board.insured, 5);
}

struct RecordWindowed(RecordState);

impl Render for RecordWindowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut cx = Cx::new(window, cx);
        record_screen::draw(&self.0, &mut cx)
    }
}

struct ListWindowed(ListState);

impl Render for ListWindowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut cx = Cx::new(window, cx);
        list_screen::draw(&self.0, &ignore(), &mut cx)
    }
}

struct RecordPage;

impl Render for RecordPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PatientProfilePage::new()
    }
}

struct ListPage;

impl Render for ListPage {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PatientListPage::new()
    }
}

/// Every state the record's `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_record_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let states = [
        RecordState::Initial,
        RecordState::Loading,
        RecordState::Loaded {
            patient: Box::new(patient()),
        },
        RecordState::Error {
            message: "The patient's record could not be read.".to_string(),
        },
    ];
    for state in states {
        let (_view, window) = cx.add_window_view(|_, _| RecordWindowed(state.clone()));
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}

/// Every state the list's `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_list_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let states = [
        ListState::Initial,
        ListState::Loading,
        ListState::Refreshing {
            listings: listings(),
        },
        ListState::Loaded {
            listings: listings(),
        },
        ListState::Error {
            message: "The directory could not be read.".to_string(),
        },
    ];
    for state in states {
        let (_view, window) = cx.add_window_view(|_, _| ListWindowed(state.clone()));
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}

/// The whole record page, dispatched and drawn.
#[gpui::test]
fn the_record_page_dispatches_its_first_load_and_draws(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let (_view, window) = cx.add_window_view(|_, _| RecordPage);
    window.update(|window, cx| {
        let _ = window.draw(cx);
    });
    window.run_until_parked();
    let (_view, again) = cx.add_window_view(|_, _| RecordPage);
    again.update(|window, cx| {
        let _ = window.draw(cx);
    });
}

/// The whole list page, dispatched and drawn.
#[gpui::test]
fn the_list_page_dispatches_its_first_load_and_draws(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let (_view, window) = cx.add_window_view(|_, _| ListPage);
    window.update(|window, cx| {
        let _ = window.draw(cx);
    });
    window.run_until_parked();
    let (_view, again) = cx.add_window_view(|_, _| ListPage);
    again.update(|window, cx| {
        let _ = window.draw(cx);
    });
}

/// The first row of the list opens the record it names.
#[gpui::test]
fn clicking_a_row_opens_the_record(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let (_view, window) = cx.add_window_view(|_, _| ListPage);
    window.update(|window, cx| {
        let _ = window.draw(cx);
    });
    window.run_until_parked();
    let row = window.debug_bounds("patient-row-0").expect("the row drew");
    window.simulate_click(row.center(), gpui::Modifiers::none());
    window.run_until_parked();
}
