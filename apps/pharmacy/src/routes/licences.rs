// `/licences` - the licences, their expiry, and the inspection checklist.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: LicencesPage }

/// The licences and inspection page.
#[component]
fn LicencesPage() -> impl IntoElement {
    Page::new(navigation::label("/licences"), navigation::subheading("/licences"))
        .child(screens::licences_and_inspection::view())
}
