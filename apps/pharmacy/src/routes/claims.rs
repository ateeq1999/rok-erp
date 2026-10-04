// `/claims` - the insurance claims, their status, and the queries to answer.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: ClaimsPage }

/// The claims page.
#[component]
fn ClaimsPage() -> impl IntoElement {
    frame::page("/claims")
        .child(screens::insurance_claims::view())
}
