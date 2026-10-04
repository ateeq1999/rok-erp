// `/prescriptions` - the queue: signed, waiting for check, ready to dispense.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: PrescriptionsPage }

/// The prescription queue page.
#[component]
fn PrescriptionsPage() -> impl IntoElement {
    frame::page("/prescriptions")
        .child(screens::prescriptions::view())
}
