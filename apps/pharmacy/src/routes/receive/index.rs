// `/receive` - the deliveries booked in and not yet received.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: ReceivePage }

/// The deliveries page.
#[component]
fn ReceivePage() -> impl IntoElement {
    frame::page("/receive")
        .child(screens::receive_delivery::list())
}
