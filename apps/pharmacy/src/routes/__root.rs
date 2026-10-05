// The window's frame: the navigation column, the current page beside it, and
// the assistant floating over both.

use rok_ui::components::direction::{self, Direction};
use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::frame;
use rok_pos_shell::AppFrame;

file_route! { layout: Root }

/// The frame every route is drawn inside, in the direction the session's
/// language draws in: left to right in English, right to left in Arabic.
#[component]
fn Root(#[children] outlet: Vec<AnyElement>) -> impl IntoElement {
    Direction::build(direction::current_direction(), || {
        AppFrame::new(frame::sidebar().into_any_element())
            // Msaidizi opens from here in Phase 14; until then the button only
            // marks where it will live.
            .on_assistant(|(), _, _| {})
            .child(div().size_full().children(outlet))
    })
}
