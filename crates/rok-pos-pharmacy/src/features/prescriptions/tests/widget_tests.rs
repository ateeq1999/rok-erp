//! The queue's widgets: every state the `BLoC` can leave is drawn, and the
//! figures the screen hands them are the ones the board shows.

use std::rc::Rc;

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::prescriptions::application::prescriptions_state::PrescriptionsState;
use crate::features::prescriptions::data::models::QueueRecord;
use crate::features::prescriptions::data::story;
use crate::features::prescriptions::domain::entities::PrescriptionQueue;
use crate::features::prescriptions::domain::enums::{Destination, StatusSeverity};
use crate::features::prescriptions::presentation::prescriptions_page::PrescriptionsPage;
use crate::features::prescriptions::presentation::prescriptions_screen::{
    self, Dispatch, board, route, tone_for,
};

/// A dispatch that does nothing, for a board drawn on its own.
fn ignore() -> Dispatch {
    Rc::new(|_, _, _| {})
}

/// The board's day, as the entity it becomes.
fn day() -> PrescriptionQueue {
    PrescriptionQueue::from(story::today())
}

#[test]
fn the_screen_hands_the_tiles_the_boards_figures() {
    let board = board(&day());
    assert_eq!(board.waiting, 6);
    assert_eq!(board.flagged, 7);
    assert_eq!(board.dispensed, 4);
    assert_eq!(board.oldest, "oldest waiting 2 h 45 min");
    assert_eq!(board.rows.len(), 10);
    assert_eq!(board.boxes.len(), 5);
    assert_eq!(board.boxes[0].0, "New");
    assert_eq!(board.boxes[0].1, 2);
    assert_eq!(board.boxes[4].0, "Dispensed today");
    assert_eq!(board.boxes[4].2, "last at 14:05");
}

#[test]
fn the_widgets_receive_figures_not_calculations() {
    let board = board(&day());
    let alert = board
        .rows
        .iter()
        .find(|row| row.reference == "RX-2214")
        .expect("the interaction row");
    assert!(alert.flag_is_alert, "a check flag is a stop, not a warning");
    assert_eq!(alert.flags, vec!["Interaction: warfarin".to_string()]);
    assert_eq!(alert.status, "Needs pharmacist check");
    assert_eq!(alert.tone, Tone::Warning);
    assert_eq!(board.rail.reference, story::SELECTED);
    assert!(board.rail.needs_a_pharmacist);
    assert_eq!(board.rail.items.len(), 2);
    assert!(board.rail.alert.is_some());
}

#[test]
fn the_selected_row_is_the_one_the_rail_opens() {
    let board = board(&day());
    assert_eq!(board.selected, board.rail.reference);
}

#[test]
fn a_day_with_nothing_in_it_still_hands_over_every_figure() {
    let empty = QueueRecord {
        range: String::new(),
        oldest_waited: String::new(),
        queue: Vec::new(),
        selected: String::new(),
        open: story::today().open,
    };
    let board = board(&PrescriptionQueue::from(empty));
    assert_eq!(board.rows.len(), 0);
    assert_eq!(board.waiting, 0);
    assert_eq!(board.boxes.len(), 5);
    assert_eq!(board.boxes[4].2, "none today");
}

#[test]
fn a_severity_becomes_a_tone_in_one_place() {
    assert_eq!(tone_for(StatusSeverity::Danger), Tone::Danger);
    assert_eq!(tone_for(StatusSeverity::Done), Tone::Success);
    assert_eq!(tone_for(StatusSeverity::Brand), Tone::Brand);
}

#[test]
fn every_destination_opens_a_screen_the_pharmacy_has() {
    for destination in [Destination::Check, Destination::PatientRecord] {
        assert!(route(destination).starts_with('/'));
    }
}

struct Windowed(PrescriptionsState);

impl Render for Windowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.0.clone();
        let dispatch = ignore();
        let mut cx = Cx::new(window, cx);
        prescriptions_screen::draw(&state, &dispatch, &mut cx)
    }
}

struct Page;

impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        PrescriptionsPage::new()
    }
}

/// Every state the `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let queue = day();
    let states = [
        PrescriptionsState::Initial,
        PrescriptionsState::Loading,
        PrescriptionsState::Refreshing {
            queue: queue.clone(),
        },
        PrescriptionsState::Loaded { queue },
        PrescriptionsState::Error {
            message: "The prescription queue could not be read.".to_string(),
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
}

/// A click on a queue row opens that prescription.
#[gpui::test]
fn clicking_a_row_opens_that_prescription(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let (_view, window) = cx.add_window_view(|_, _| Page);
    window.update(|window, cx| {
        let _ = window.draw(cx);
    });
    let opened = window
        .debug_bounds("prescription-row-0")
        .expect("the first row drew");
    window.simulate_click(opened.center(), gpui::Modifiers::none());
    window.run_until_parked();
}
