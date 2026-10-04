// `/batches` - stock by batch and expiry, first expiry first out.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: BatchesPage }

/// The batches and expiry page.
#[component]
fn BatchesPage() -> impl IntoElement {
    frame::page("/batches")
        .child(screens::batches_and_expiry::view())
}
