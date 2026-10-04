//! The round help button the boards pin to the bottom right of every screen.

use rok_ui::prelude::*;

use crate::theme::{ACCENT_STRONG, hsla};

styles! {
    ASSISTANT_BUTTON = {
        root: {
            position: absolute,
            right: 6,
            bottom: 6,
            size: 15,
            radius: full,
            display: flex,
            align: center,
            justify: center,
            background: primary,
            color: primary_foreground,
            border: 1,
            border_color: border,
            cursor: pointer,
            shadow: lg,
        },
    }
}

/// The floating assistant button, in the brand teal.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::AssistantButton;
/// let button = AssistantButton::new().on_press(|(), _, _| {});
/// ```
#[component]
pub fn AssistantButton(
    #[default] on_press: Option<EventHandler<()>>,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let colors: ThemeColors = cx.theme().colors.clone();
    let hover = hsla(ACCENT_STRONG);
    let keys = on_press.clone();
    div()
        .id("assistant-button")
        .tab_index(0)
        .on_click(move |_, window, cx| {
            if let Some(handler) = &on_press {
                handler(&(), window, cx);
            }
        })
        .on_key_down(move |event, window, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                if let Some(handler) = &keys {
                    handler(&(), window, cx);
                }
            }
        })
        .sx(sx![
            &ASSISTANT_BUTTON.root,
            style! { hover: { background: {hover} } },
            &sx,
        ])
        .child(
            Icon::new(IconName::Sparkles)
                .size(px(22.))
                .color(colors.primary_foreground),
        )
}

#[cfg(test)]
mod tests {
    use super::AssistantButton;
    use rok_ui::prelude::*;

    struct Floating;

    impl Render for Floating {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().child(AssistantButton::new().on_press(|(), _, _| {}))
        }
    }

    #[gpui::test]
    fn floats_over_a_page(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Floating);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
