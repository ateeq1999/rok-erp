// `/recalls` - the open batch recalls, and the batches they cover.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: RecallsPage }

/// The recalls page.
#[component]
fn RecallsPage() -> impl IntoElement {
    frame::page("/recalls")
        .child(screens::recalls::list())
}
