//! The bar across the top, as the `PosTopBar` board draws it: the page's
//! heading and the line under it, a page's own actions, then search, the sync
//! status, the assistant and the notifications bell.
//!
//! Every board opens with it at 64 pixels tall, full width, above a `main`
//! padded `20px 28px`.

use rok_ui::prelude::*;

use crate::tone::{self, Tone};

/// How tall the top bar is on the boards, in pixels.
pub const HEIGHT_PX: f32 = 64.;

/// How tall the top bar is in spacing units, which is how the styles read it.
pub const HEIGHT: f32 = HEIGHT_PX / 4.;

/// How wide the search field is, in pixels.
pub const SEARCH_WIDTH_PX: f32 = 280.;

/// What the search field says before anything is typed.
pub const SEARCH_PLACEHOLDER: &str = "Search products, sales, customers";

/// The assistant's button: Msaidizi is Kiswahili for "helper".
pub const ASSISTANT_LABEL: &str = "Ask Msaidizi";

/// The chip beside search: "Synced 2 min ago", "Offline, 3 sales queued".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopBarStatus {
    /// What the chip says.
    pub label: SharedString,
    /// [`Tone::Success`] when all is well, [`Tone::Warning`] when it is not.
    pub tone: Tone,
}

impl TopBarStatus {
    /// A status chip.
    #[must_use]
    pub fn new(label: impl Into<SharedString>, tone: Tone) -> Self {
        Self {
            label: label.into(),
            tone,
        }
    }
}

styles! {
    TOP_BAR = {
        root: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 4,
            height: HEIGHT,
            shrink: 0,
            padding_x: 7,
            background: card,
            color: foreground,
            border_bottom: 1,
            border_color: border,
            font_family: sans,
            text: {14.},
        },
        titles: { display: flex, flex_direction: column, grow: 1, min_width: 0 },
        heading: { font_family: mono, text: {20.}, font: semibold, truncate: true },
        subheading: { text: {13.}, color: muted_foreground, truncate: true },
        actions: { display: flex, flex_direction: row, align: center, gap: 2 },
        search: { position: relative, width: {px(SEARCH_WIDTH_PX)}, shrink: 0 },
        shortcut: { position: absolute, right: 3, top: {px(10.)} },
        status: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 1.5,
            shrink: 0,
            padding_x: 2.5,
            padding_y: 1.5,
            text: {13.},
            font: medium,
        },
        assistant: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 2,
            height: 10,
            shrink: 0,
            padding_x: 3,
            background: foreground,
            color: background,
            font: semibold,
            cursor: pointer,
            hover: { background: foreground/85 },
        },
        bell: {
            position: relative,
            display: flex,
            align: center,
            justify: center,
            size: 10,
            shrink: 0,
            border: 1,
            border_color: border,
            background: card,
            cursor: pointer,
        },
        unread: {
            position: absolute,
            top: {px(7.)},
            right: 2,
            size: 2,
            radius: full,
            background: destructive,
        },
    }
}

/// A clickable box that also answers Enter and Space.
fn pressable(id: &'static str, handler: Option<EventHandler<()>>) -> gpui::Stateful<Div> {
    let keys = handler.clone();
    div()
        .id(id)
        .tab_index(0)
        .on_click(move |_, window, cx| {
            if let Some(handler) = &handler {
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
}

/// The bar across the top of a page.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::{Tone, TopBar, TopBarStatus};
/// let bar = TopBar::new("Prescriptions", "Afya Pharmacy - Mwenge branch")
///     .status(TopBarStatus::new("Synced 2 min ago", Tone::Success))
///     .unread(true);
/// ```
#[component]
pub fn TopBar(
    heading: SharedString,
    subheading: SharedString,
    #[default] status: Option<TopBarStatus>,
    #[default] unread: bool,
    #[default] on_assistant: Option<EventHandler<()>>,
    #[default] on_notifications: Option<EventHandler<()>>,
    #[children] children: Vec<AnyElement>,
    #[sx] sx: Sx,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let mode = cx.theme().mode;
    let search = use_input_state("top-bar-search", window, cx, |state| {
        state.with_placeholder(SEARCH_PLACEHOLDER)
    });

    div()
        .sx((&TOP_BAR.root, &sx))
        .child(
            div()
                .sx(&TOP_BAR.titles)
                .child(div().sx(&TOP_BAR.heading).child(heading))
                .when(!subheading.is_empty(), |titles| {
                    titles.child(div().sx(&TOP_BAR.subheading).child(subheading))
                }),
        )
        .when(!children.is_empty(), |bar| {
            bar.child(div().sx(&TOP_BAR.actions).children(children))
        })
        .child(
            div()
                .sx(&TOP_BAR.search)
                .child(Input::new(&search).leading_icon(IconName::Search))
                .child(div().sx(&TOP_BAR.shortcut).child(Kbd::new("Ctrl K"))),
        )
        .when_some(status, |bar, status| {
            let palette = tone::colors(status.tone, mode);
            let icon = if matches!(status.tone, Tone::Success) {
                IconName::Check
            } else {
                IconName::CircleAlert
            };
            bar.child(
                div()
                    .sx(sx![
                        &TOP_BAR.status,
                        style! { background: {palette.background}, color: {palette.foreground} },
                    ])
                    .child(Icon::new(icon).size(px(14.)).color(palette.foreground))
                    .child(status.label),
            )
        })
        .child(
            pressable("top-bar-assistant", on_assistant)
                .sx(&TOP_BAR.assistant)
                .child(
                    Icon::new(IconName::Sparkles)
                        .size(px(16.))
                        .color(colors.background),
                )
                .child(ASSISTANT_LABEL),
        )
        .child(
            pressable("top-bar-notifications", on_notifications)
                .sx(&TOP_BAR.bell)
                .child(
                    Icon::new(IconName::Bell)
                        .size(px(18.))
                        .color(colors.foreground),
                )
                .when(unread, |bell| bell.child(div().sx(&TOP_BAR.unread))),
        )
}

#[cfg(test)]
mod tests {
    use super::{TopBar, TopBarStatus};
    use crate::tone::Tone;
    use rok_ui::prelude::*;

    struct Bar;

    impl Render for Bar {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children([
                TopBar::new(
                    "Dashboard",
                    "Afya Pharmacy - Mwenge branch - Fri 2 Oct 2026, 15:30",
                )
                .status(TopBarStatus::new("Synced 2 min ago", Tone::Success))
                .unread(true)
                .into_any_element(),
                TopBar::new("Prescriptions", "")
                    .status(TopBarStatus::new("Offline", Tone::Warning))
                    .child(Button::new("new").label("New"))
                    .into_any_element(),
            ])
        }
    }

    #[gpui::test]
    fn draws_with_a_status_actions_and_the_unread_dot(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Bar);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
