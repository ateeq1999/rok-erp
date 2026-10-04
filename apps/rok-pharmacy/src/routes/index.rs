// `/` - the dashboard: today's sales, waiting prescriptions, expiring value.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: DashboardPage }

/// The dashboard page.
#[component]
fn DashboardPage() -> impl IntoElement {
    Page::new(navigation::label("/"), navigation::subheading("/")).child(screens::dashboard::view())
}
