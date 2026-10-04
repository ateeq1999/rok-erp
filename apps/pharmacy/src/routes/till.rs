// `/till` - sell over the counter: the basket, the split payment, the label.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: TillPage }

/// The dispensary till page.
#[component]
fn TillPage() -> impl IntoElement {
    frame::page("/till")
        .child(screens::dispensary_till::view())
}
