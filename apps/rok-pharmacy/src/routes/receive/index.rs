// `/receive` - the deliveries booked in and not yet received.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: ReceivePage }

/// The deliveries page.
#[component]
fn ReceivePage() -> impl IntoElement {
    Page::new(navigation::label("/receive"), navigation::subheading("/receive"))
        .child(screens::receive_delivery::list())
}
