// The window's frame: the navigation column, and the current page beside it.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::navigation;
use rok_pos_shell::{AppFrame, Sidebar, UserChip};

file_route! { layout: Root }

/// The frame every route is drawn inside.
#[component]
fn Root(#[children] outlet: Vec<AnyElement>) -> impl IntoElement {
    AppFrame::new(
        Sidebar::new("Afya Pharmacy", "Mwenge branch")
            .groups(navigation::groups(&|_| true))
            .user(UserChip::new("AT", "Amani T.", "Pharmacist"))
            .into_any_element(),
    )
    .child(div().size_full().children(outlet))
}
