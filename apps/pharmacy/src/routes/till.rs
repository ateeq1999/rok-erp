// `/till` - sell over the counter: the basket, the split payment, the label.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: TillPage }

/// The dispensary till page.
#[component]
fn TillPage() -> impl IntoElement {
    Page::new(navigation::label("/till"), navigation::subheading("/till"))
        .child(screens::dispensary_till::view())
}
