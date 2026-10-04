// `/controlled-register` - the running balance of every controlled medicine.

use rok_ui::prelude::*;
use rok_ui::router::file_route;
use rok_pos_pharmacy::{navigation, screens};
use rok_pos_shell::Page;

file_route! { component: ControlledRegisterPage }

/// The controlled register page.
#[component]
fn ControlledRegisterPage() -> impl IntoElement {
    Page::new(
        navigation::label("/controlled-register"),
        navigation::subheading("/controlled-register"),
    )
    .child(screens::controlled_register::view())
}
