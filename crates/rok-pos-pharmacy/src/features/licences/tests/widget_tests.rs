//! The licences widgets: every state the `BLoC` can leave is drawn, and the
//! figures the screen resolves are the board's.

use gpui::prelude::*;
use rok_ui::prelude::*;

use crate::features::licences::application::licence_event::LicenceEvent;
use crate::features::licences::application::licence_state::LicenceState;
use crate::features::licences::data::story;
use crate::features::licences::domain::entities::{CivilDate, Licences};
use crate::features::licences::presentation::licence_page::LicencesPage;
use crate::features::licences::presentation::licence_screen;

/// A dispatch that does nothing, for a board drawn on its own.
fn ignore() -> licence_screen::Dispatch {
    std::rc::Rc::new(|_, _, _| {})
}

/// The board's folder, as the entities it becomes.
fn folder() -> Licences {
    Licences::from(story::folder())
}

#[test]
fn the_screen_resolves_the_boards_figures() {
    let board = licence_screen::board(&folder());
    assert_eq!(board.held, 5);
    assert_eq!(board.renewing, 2);
    assert_eq!(board.ready, 5);
    assert_eq!(board.total, 6);
    assert_eq!(board.missing, 1);
    assert_eq!(board.days_left, 57);
}

#[test]
fn a_date_draws_as_the_folder_spells_it() {
    assert_eq!(
        licence_screen::day(CivilDate::new(2026, 11, 30)),
        "30 Nov 2026"
    );
    assert_eq!(
        licence_screen::day(CivilDate::new(2027, 2, 14)),
        "14 Feb 2027"
    );
}

struct Windowed(LicenceState);

impl Render for Windowed {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut cx = Cx::new(window, cx);
        licence_screen::draw(&self.0, &ignore(), &mut cx)
    }
}

struct Page;

impl Render for Page {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        LicencesPage::new()
    }
}

/// Every state the folder's `BLoC` can leave is drawn in a window.
#[gpui::test]
fn every_state_is_drawn(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        rok_ui::init(cx);
        rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
    });
    let states = [
        LicenceState::Initial,
        LicenceState::Loading,
        LicenceState::Loaded {
            licences: Box::new(folder()),
        },
        LicenceState::Error {
            message: "The folder could not be read.".to_string(),
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

/// A `LicenceEvent` is one the page can send, which keeps the type live.
#[test]
fn the_page_can_ask_for_the_folder_twice() {
    let first = LicenceEvent::Load;
    let again = LicenceEvent::Retry;
    assert_ne!(first, again);
}
