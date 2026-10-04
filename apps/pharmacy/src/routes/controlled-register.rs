// `/controlled-register` - the running balance of every controlled medicine.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{frame, screens};

file_route! { component: ControlledRegisterPage }

/// The controlled register page.
#[component]
fn ControlledRegisterPage() -> impl IntoElement {
    frame::page("/controlled-register")
    .child(screens::controlled_register::view())
}
