//! The window: the navigation column, and whatever the current route draws
//! beside it.
//!
//! A route draws its own page, top bar included, so the shell only owns the
//! column that does not change.

use rok_ui::prelude::*;

use crate::assistant_button::AssistantButton;
use crate::sidebar::WIDTH_PX;

styles! {
    APP_FRAME = {
        root: {
            position: relative,
            size: full,
            display: flex,
            flex_direction: row,
            gap: 0,
            background: background,
            color: foreground,
            font_family: sans,
            text: base,
        },
        sidebar: { width: {px(WIDTH_PX)}, shrink: 0, height: full },
        main: { flex: 1, min_width: 0, display: flex, flex_direction: column },
    }
}

/// The window, with `sidebar` down the left and `children` filling the rest.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::{AppFrame, PageHeader, Sidebar};
/// let window = AppFrame::new(Sidebar::new("Afya Pharmacy", "Mwenge branch").into_any_element())
///     .child(PageHeader::new("Dashboard", "Afya Pharmacy - Mwenge branch"));
/// ```
#[component]
pub fn AppFrame(
    sidebar: AnyElement,
    #[default] on_assistant: Option<EventHandler<()>>,
    #[children] children: Vec<AnyElement>,
    #[sx] sx: Sx,
) -> impl IntoElement {
    div()
        .sx((&APP_FRAME.root, &sx))
        .child(div().sx(&APP_FRAME.sidebar).child(sidebar))
        .child(div().sx(&APP_FRAME.main).children(children))
        .when_some(on_assistant, |frame, on_press| {
            frame.child(
                AssistantButton::new().on_press(move |(), window, cx| on_press(&(), window, cx)),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::AppFrame;
    use rok_ui::prelude::*;

    struct PharmacyWindow {
        assistant: bool,
    }

    impl Render for PharmacyWindow {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            AppFrame::new(crate::Sidebar::new("Afya Pharmacy", "Mwenge branch").into_any_element())
                .when(self.assistant, |frame| frame.on_assistant(|(), _, _| {}))
                .child(
                    crate::Page::new(
                        "Dashboard",
                        "Afya Pharmacy - Mwenge branch - Fri 2 Oct 2026, 15:30",
                    )
                    .actions(vec![
                        Button::new("shift")
                            .label("Open shift")
                            .icon(IconName::ArrowUp)
                            .into_any_element(),
                    ])
                    .child(crate::PageHeader::new(
                        "Dashboard",
                        "Today: sales and waiting prescriptions",
                    )),
                )
        }
    }

    #[gpui::test]
    fn draws_a_window_with_and_without_the_assistant(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| PharmacyWindow { assistant: true });
        window.update(|window, cx| {
            let _ = window.draw(cx);
            let _ = window.draw(cx);
        });
    }
}
