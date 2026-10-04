// `/refills` - who is due a refill this week, and who has been told.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: RefillsPage }

/// The refills page.
#[component]
fn RefillsPage() -> impl IntoElement {
    frame::page("/refills")
        .child(screens::refills::view())
}
