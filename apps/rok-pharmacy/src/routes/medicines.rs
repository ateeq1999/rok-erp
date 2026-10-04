// `/medicines` - the catalogue the pharmacy may dispense from.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: MedicinesPage }

/// The medicine catalogue page.
#[component]
fn MedicinesPage() -> impl IntoElement {
    Page::new(navigation::label("/medicines"), navigation::subheading("/medicines"))
        .child(screens::medicines::view())
}
