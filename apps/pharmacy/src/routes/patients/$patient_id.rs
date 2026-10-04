// `/patients/:patient_id` - one patient's conditions, medicines and history.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: PatientPage }

/// The patient record page, for the patient in the route.
#[component]
fn PatientPage(cx: &mut App) -> impl IntoElement {
    let route = params(cx);
    frame::titled("Patient record")
        .child(screens::patient_profile::record(&route.patient_id))
}
