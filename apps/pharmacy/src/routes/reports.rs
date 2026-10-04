// `/reports` - sales by medicine and schedule, margin, expiry losses, claims
// aging and controlled movements.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: ReportsPage }

/// The reports page.
#[component]
fn ReportsPage() -> impl IntoElement {
    frame::page("/reports").child(screens::reports::view())
}
