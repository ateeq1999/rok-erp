// `/` - the dashboard: today's sales, waiting prescriptions, expiring value.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, features};

file_route! { component: DashboardPage }

/// The dashboard page: the route only frames the feature's page.
#[component]
fn DashboardPage() -> impl IntoElement {
    frame::page("/").child(features::DashboardPage::new())
}
