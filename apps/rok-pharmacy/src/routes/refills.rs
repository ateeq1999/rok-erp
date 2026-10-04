// `/refills` - who is due a refill this week, and who has been told.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: RefillsPage }

/// The refills page.
#[component]
fn RefillsPage() -> impl IntoElement {
    Page::new(navigation::label("/refills"), navigation::subheading("/refills"))
        .child(screens::refills::view())
}
