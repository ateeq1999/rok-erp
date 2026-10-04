// `/` - the dashboard: today's sales, waiting prescriptions, expiring value.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: DashboardPage }

/// The dashboard page.
#[component]
fn DashboardPage() -> impl IntoElement {
    frame::page("/").child(screens::dashboard::view())
}
