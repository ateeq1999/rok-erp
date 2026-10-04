// `/medicines` - the catalogue the pharmacy may dispense from.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: MedicinesPage }

/// The medicine catalogue page.
#[component]
fn MedicinesPage() -> impl IntoElement {
    frame::page("/medicines")
        .child(screens::medicines::view())
}
