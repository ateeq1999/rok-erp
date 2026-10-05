// `/prescriptions/:prescription_id/check` - the pharmacist's check, and the label
// to print after it.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{features, frame};

file_route! { component: CheckPage }

/// The clinical check page: the route only frames the feature's page.
#[component]
fn CheckPage() -> impl IntoElement {
    frame::titled("Clinical check").child(features::ClinicalCheckPage::new())
}