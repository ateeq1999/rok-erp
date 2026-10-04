// `/patients` - find a patient to open their record.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: PatientsPage }

/// The patient list page.
#[component]
fn PatientsPage() -> impl IntoElement {
    Page::new(navigation::label("/patients"), navigation::subheading("/patients"))
        .child(screens::patient_profile::list())
}
