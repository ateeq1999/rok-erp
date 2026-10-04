// `/recalls` - the open batch recalls, and the batches they cover.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: RecallsPage }

/// The recalls page.
#[component]
fn RecallsPage() -> impl IntoElement {
    Page::new(navigation::label("/recalls"), navigation::subheading("/recalls"))
        .child(screens::recalls::list())
}
