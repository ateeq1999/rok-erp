// The 404: the address that was asked for, and the way back.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_shell::{Page, PageHeader};

file_route! { component: NotFound }

/// The page a router sends a path no route file claims.
#[component]
fn NotFound() -> impl IntoElement {
    Page::new("No such page", "Afya Pharmacy - Mwenge branch").child(
        PageHeader::new("404", "This pharmacy has no page at that address")
            .actions(vec![Button::new("home")
                .icon(IconName::Home)
                .label("Back to the dashboard")
                .into_any_element()]),
    )
}
