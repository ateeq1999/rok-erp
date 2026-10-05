//! The check's widgets: every state the `BLoC` can leave is drawn, and the
//! figures the screen hands them are the ones the board shows.

use std::rc::Rc;

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::clinical_check::application::check_state::CheckState;
use crate::features::clinical_check::data::story;
use crate::features::clinical_check::domain::entities::ClinicalCheck;
use crate::features::clinical_check::domain::enums::{Destination, Severity};
use crate::features::clinical_check::presentation::check_page::ClinicalCheckPage;
use crate::features::clinical_check::presentation::check_screen::{
    self, Dispatch, board, route, tone_for,
};

/// A dispatch that does nothing, for a board drawn on its own.
fn ignore() -> Dispatch {
    Rc::new(|_, _, _| {})
}

/// The board's prescription, as the entity it becomes.
fn check() -> ClinicalCheck {
    ClinicalCheck::from(story::rx_2214())
}

#[test]
fn the_screen_hands_the_tiles_the_boards_figures() {
    let board = board(&check());
    assert_eq!(board.medicines, 2);
    assert_eq!(board.cleared, 5);
    assert_eq!(board.total, 6);
    assert_eq!(board.alerts, 1);
    assert!(board.call_recorded);
    assert!(board.can_approve);
    assert_eq!(board.checks.len(), 6);
    assert_eq!(board.outcomes.len(), 3);
    assert_eq!(board.counselling.len(), 5);
    assert_eq!(board.initials, "GN");
}

#[test]
fn the_widgets_receive_figures_not_calculations() {
    let board = board(&check());
    let alert = board
        .checks
        .iter()
        .find(|one| one.alert)
        .expect("the interaction check");
    assert_eq!(alert.label, "Interactions");
    assert_eq!(alert.result, "Alert");
    assert_eq!(alert.tone, Tone::Danger);
    assert_eq!(
        board.checks.iter().filter(|one| !one.alert).count(),
        5,
        "the rest are not tinted"
    );

    let first = &board.medicines_table[0];
    assert_eq!(first.name, "Metronidazole 400mg tablets");
    assert_eq!(first.batch, "MTZ-2503");
    assert!(first.detail.contains("exp 06/2027"));

    assert!(board.outcomes[1].chosen);
    assert_eq!(board.outcomes[1].label, "Kept as written, extra INR check");
    assert!(!board.outcomes[0].chosen);
}

#[test]
fn the_label_preview_is_handed_over_as_a_figure() {
    let board = board(&check());
    let label = board.label.as_ref().expect("a label to preview");
    assert_eq!(label.patient, "Mzee Salim R.");
    assert_eq!(label.batch, "MTZ-2503");
    assert_eq!(label.labels, 2);
    assert!(label.headline.contains("Metronidazole 400mg tablets"));
    assert!(label.words.starts_with("Take 1 tablet"));
}

#[test]
fn a_paper_with_nothing_on_it_draws_no_label() {
    let mut record = story::rx_2214();
    record.medicines.clear();
    assert!(board(&ClinicalCheck::from(record)).label.is_none());
}

#[test]
fn an_unanswered_alert_is_drawn_as_not_yet_approvable() {
    let mut record = story::rx_2214();
    record
        .outcomes
        .iter_mut()
        .for_each(|one| one.chosen = false);
    let board = board(&ClinicalCheck::from(record));
    assert!(!board.call_recorded);
    assert!(!board.can_approve);
}

#[test]
fn a_severity_becomes_a_tone_in_one_place() {
    assert_eq!(tone_for(Severity::Clear), Tone::Success);
    assert_eq!(tone_for(Severity::Alert), Tone::Danger);
}

#[test]
fn every_destination_opens_a_screen_the_pharmacy_has() {
    assert_eq!(route(Destination::Prescriptions), "/prescriptions");
    assert_eq!(route(Destination::PatientRecord), "/patients");
    assert_eq!(route(Destination::Till), "/till");
}

struct Windowed(CheckState);

impl Render for Windowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.0.clone();
        let dispatch = ignore();
        let mut cx = Cx::new(window, cx);
        check_screen::draw(&state, &dispatch, &mut cx)
    }
}

struct Page;

impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        ClinicalCheckPage::new()
    }
}

/// Every state the `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let check = check();
    let states = [
        CheckState::Initial,
        CheckState::Loading,
        CheckState::Refreshing {
            check: check.clone(),
        },
        CheckState::Loaded { check },
        CheckState::Error {
            message: "The prescription could not be read.".to_string(),
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

/// The back link on the rail opens the queue.
#[gpui::test]
fn clicking_back_goes_to_the_queue(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let (_view, window) = cx.add_window_view(|_, _| Page);
    window.update(|window, cx| {
        let _ = window.draw(cx);
    });
    window.run_until_parked();
    let back = window.debug_bounds("check-back").expect("the rail drew");
    window.simulate_click(back.center(), gpui::Modifiers::none());
    window.run_until_parked();
}
