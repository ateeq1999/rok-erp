//! The bar across the top, as the `PosTopBar` board draws it: the page's
//! heading and the line under it, a page's own actions, then search, the sync
//! status, the assistant, the appearance toggle and the notifications bell.
//!
//! Every board opens with it at 64 pixels tall, full width, above a `main`
//! padded `20px 28px`.

use rok_ui::prelude::*;

use crate::theme::{self, Appearance};
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

/// What the appearance toggle says on hover, as a tooltip would.
pub const APPEARANCE_HINT: &str = "Appearance: System, Light, Dark";

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
        appearance: {
            display: flex,
            flex_direction: row,
            align: center,
            justify: center,
            gap: 1.5,
            height: 10,
            shrink: 0,
            padding_x: 2.5,
            border: 1,
            border_color: border,
            background: card,
            cursor: pointer,
            hover: { background: muted },
        },
        appearance_word: { text: {13.}, font: medium },
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
/// ```no_run
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
    cx: &mut Cx,
) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let mode = cx.theme().mode;

    // What the user chose. Default is `System`, so a window that has never been
    // touched follows the desktop, which is what the boards assume.
    let chosen = cx.use_state(Appearance::default);

    // Keep the choice applied when the desktop flips. The subscription is held
    // in the component's own state, so it lives as long as the bar is drawn and
    // is dropped with it.
    let subscription = theme::observe_appearance(chosen.clone(), cx.window);
    let _subscribed = cx.use_state(|| subscription);

    let search = use_input_state("top-bar-search", cx.window, cx.app, |state| {
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
        .child(appearance_toggle(&chosen, &colors, cx))
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

/// The System / Light / Dark button, showing the current choice.
///
/// One click steps to the next choice and repaints; there is no separate
/// "apply", because a theme that has not changed yet is not a state the user
/// can be stranded in.
fn appearance_toggle(
    chosen: &State<Appearance>,
    colors: &ThemeColors,
    cx: &App,
) -> impl IntoElement {
    let current = chosen.get(cx);
    let next = current.next();
    let clicked = chosen.clone();
    let keyed = chosen.clone();
    div()
        .id("top-bar-appearance")
        .debug_selector(|| "top-bar-appearance".to_string())
        .tab_index(0)
        .sx(&TOP_BAR.appearance)
        .on_click(move |_, window, cx| {
            clicked.set(next, cx);
            theme::apply_appearance(next, window, cx);
        })
        .on_key_down(move |event, window, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                keyed.set(next, cx);
                theme::apply_appearance(next, window, cx);
            }
        })
        .child(
            Icon::new(current.icon())
                .size(px(16.))
                .color(colors.foreground),
        )
        .child(div().sx(&TOP_BAR.appearance_word).child(current.label()))
}

#[cfg(test)]
mod tests {
    use super::{APPEARANCE_HINT, TopBar, TopBarStatus};
    use crate::theme::Appearance;
    use crate::tone::Tone;
    use gpui::Hsla;
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

    /// A page with a single bar, for tests that press one of its buttons.
    struct OneBar;

    impl Render for OneBar {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            TopBar::new("Dashboard", "Afya Pharmacy - Mwenge branch")
                .status(TopBarStatus::new("Synced 2 min ago", Tone::Success))
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

    #[gpui::test]
    fn clicking_the_appearance_button_repaints_the_window(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
            crate::theme::install_theme(cx);
        });
        // One bar, so the button the test clicks is the only one on screen.
        let (_view, window) = cx.add_window_view(|_, _| OneBar);
        window.run_until_parked();

        // The test window reports a light appearance, so an untouched bar
        // follows the system and the boards' light theme is installed.
        assert_eq!(
            mode(window),
            Appearance::System.resolve_mode(ThemeMode::Light),
            "a window that has not been touched follows the system"
        );

        // System -> Light stays light on a light desktop, so step past it.
        click(window);
        assert_eq!(mode(window), ThemeMode::Light, "one click pins Light");
        click(window);
        assert_eq!(mode(window), ThemeMode::Dark, "two clicks pin Dark");
        assert_eq!(
            background(window),
            crate::theme::colors(ThemeMode::Dark).background,
            "the dark theme is really the dark palette"
        );

        // Dark -> System hands the choice back to the desktop, which is light.
        click(window);
        assert_eq!(
            mode(window),
            ThemeMode::Light,
            "three clicks return to System"
        );
        assert_eq!(
            background(window),
            crate::theme::colors(ThemeMode::Light).background,
            "returning to System restores the boards' palette"
        );
    }

    /// The mode the window is drawing in right now.
    fn mode(window: &mut gpui::VisualTestContext) -> ThemeMode {
        window.update(|_, cx| cx.theme().mode)
    }

    /// The app background right now, which is the token that proves the whole
    /// palette moved rather than one element being recoloured.
    fn background(window: &mut gpui::VisualTestContext) -> Hsla {
        window.update(|_, cx| cx.theme().colors.background)
    }

    /// One press on the appearance button, then let the frame settle.
    fn click(window: &mut gpui::VisualTestContext) {
        let bounds = window
            .debug_bounds("top-bar-appearance")
            .expect("the appearance button is drawn");
        window.simulate_click(bounds.center(), gpui::Modifiers::none());
        window.run_until_parked();
    }

    #[test]
    fn the_hint_names_the_three_choices_in_order() {
        assert_eq!(APPEARANCE_HINT, "Appearance: System, Light, Dark");
        let words: Vec<&str> = APPEARANCE_HINT
            .split(": ")
            .nth(1)
            .expect("a list of choices")
            .split(", ")
            .collect();
        assert_eq!(words, ["System", "Light", "Dark"]);
    }
}
