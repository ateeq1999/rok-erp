// `/receive/:order_id` - book in one delivery: its batches, their expiry, and
// the cold chain readings.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::screens;
use rok_pos_shell::Page;

file_route! { component: ReceiveOrderPage }

/// The receiving page, for the order in the route.
#[component]
fn ReceiveOrderPage(cx: &mut App) -> impl IntoElement {
    let route = params(cx);
    Page::new("Receive delivery", "Afya Pharmacy - Mwenge branch")
        .child(screens::receive_delivery::view(&route.order_id))
}
