// `/patients/:patient_id` - one patient's conditions, medicines and history.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{features, frame};

file_route! { component: PatientPage }

/// The patient record page: the route only frames the feature's page. The
/// record follows the route's id in Phase 3; until then every id opens the
/// board's patient, as the check route opens the board's prescription.
#[component]
fn PatientPage() -> impl IntoElement {
    frame::titled("Patient record").child(features::PatientProfilePage::new())
}
