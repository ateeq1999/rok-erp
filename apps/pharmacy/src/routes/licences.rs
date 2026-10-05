// `/licences` - the licences, their expiry, and the inspection checklist.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{features, frame};

file_route! { component: LicencesPage }

/// The licences and inspection page.
#[component]
fn LicencesPage() -> impl IntoElement {
    frame::page("/licences").child(features::LicencesPage::new())
}
