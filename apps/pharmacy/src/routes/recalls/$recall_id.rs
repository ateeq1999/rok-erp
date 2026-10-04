// `/recalls/:recall_id` - one recall: the batches held, the customers to tell,
// the returns to book.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::screens;
use rok_pos_shell::Page;

file_route! { component: RecallPage }

/// The recall page, for the recall in the route.
#[component]
fn RecallPage(cx: &mut App) -> impl IntoElement {
    let route = params(cx);
    Page::new("Batch recall", "Afya Pharmacy - Mwenge branch")
        .child(screens::recalls::view(&route.recall_id))
}
