// `/patients` - find a patient to open their record.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{features, frame};

file_route! { component: PatientsPage }

/// The patient list page: the route only frames the feature's page.
#[component]
fn PatientsPage() -> impl IntoElement {
    frame::page("/patients").child(features::PatientListPage::new())
}
