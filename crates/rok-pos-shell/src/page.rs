//! A whole page: the top bar above it, and the padded content under it.
//!
//! This is what a route draws. The sidebar stays put; the page scrolls.

use rok_ui::prelude::*;

use crate::page_main::PageMain;
use crate::top_bar::{TopBar, TopBarStatus};

styles! {
    PAGE = {
        root: {
            size: full,
            display: flex,
            flex_direction: column,
            background: background,
        },
    }
}

/// A page: a top bar named for where you are, and the content under it.
///
/// `status`, `unread` and the two handlers go straight to the [`TopBar`].
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::Page;
/// let page = Page::new("Prescriptions", "Afya Pharmacy - Mwenge branch")
///     .child(Button::new("new").label("New prescription"));
/// ```
#[component]
pub fn Page(
    heading: SharedString,
    subheading: SharedString,
    #[default] actions: Vec<AnyElement>,
    #[default] status: Option<TopBarStatus>,
    #[default] unread: bool,
    #[default] on_assistant: Option<EventHandler<()>>,
    #[default] on_notifications: Option<EventHandler<()>>,
    #[children] children: Vec<AnyElement>,
    #[sx] sx: Sx,
) -> impl IntoElement {
    let mut top_bar = TopBar::new(heading, subheading)
        .unread(unread)
        .children(actions);
    if let Some(status) = status {
        top_bar = top_bar.status(status);
    }
    if let Some(handler) = on_assistant {
        top_bar = top_bar.on_assistant(move |(), window, cx| handler(&(), window, cx));
    }
    if let Some(handler) = on_notifications {
        top_bar = top_bar.on_notifications(move |(), window, cx| handler(&(), window, cx));
    }
    div()
        .sx((&PAGE.root, &sx))
        .child(top_bar)
        .child(PageMain::new().children(children))
}

#[cfg(test)]
mod tests {
    use super::Page;
    use rok_ui::prelude::*;

    struct Prescriptions;

    impl Render for Prescriptions {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Page::new("Prescriptions", "Afya Pharmacy - Mwenge branch")
                .actions(vec![
                    Button::new("new")
                        .label("New prescription")
                        .into_any_element(),
                ])
                .child(crate::PageHeader::new("Queue", "12 waiting"))
                .child(crate::Chip::new("Waiting"))
        }
    }

    #[gpui::test]
    fn draws_a_page_with_a_top_bar_and_content(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Prescriptions);
        window.update(|window, cx| {
            let _ = window.draw(cx);
            let _ = window.draw(cx);
        });
    }
}
