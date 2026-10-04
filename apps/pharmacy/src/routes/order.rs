// `/order` - what to order from the licensed suppliers, and what is on its way.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: OrderPage }

/// The ordering page.
#[component]
fn OrderPage() -> impl IntoElement {
    Page::new(navigation::label("/order"), navigation::subheading("/order"))
        .child(screens::order_medicines::view())
}
