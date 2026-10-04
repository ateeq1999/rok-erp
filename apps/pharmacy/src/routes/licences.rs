// `/licences` - the licences, their expiry, and the inspection checklist.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: LicencesPage }

/// The licences and inspection page.
#[component]
fn LicencesPage() -> impl IntoElement {
    frame::page("/licences")
        .child(screens::licences_and_inspection::view())
}
