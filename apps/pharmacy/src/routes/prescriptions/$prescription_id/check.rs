// `/prescriptions/:prescription_id/check` - the pharmacist's check, and the label
// to print after it.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: CheckPage }

/// The clinical check page, for the prescription in the route.
#[component]
fn CheckPage(cx: &mut App) -> impl IntoElement {
    let route = params(cx);
    frame::titled("Clinical check")
        .child(screens::prescription_check::view(&route.prescription_id))
}
