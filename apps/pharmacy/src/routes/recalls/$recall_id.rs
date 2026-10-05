// `/recalls/:recall_id` - one recall: the batches held, the customers to tell,
// the returns to book.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: RecallPage }

/// The recall page, for the recall in the route.
#[component]
fn RecallPage(cx: &mut App) -> impl IntoElement {
    let route = params(cx);
    frame::titled("Batch recall")
        .child(screens::recalls::recall_view(&route.recall_id))
}
