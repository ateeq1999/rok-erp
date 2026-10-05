//! The pharmacy theme: the Rok preset with Afya Pharmacy's teal accent.
//!
//! Every colour here is taken from the boards in `design/`. A screen never
//! hard-codes one: it reads a theme token, so light and dark both work and the
//! supplier app can use the same frame with its own accent.

use gpui::{App, Hsla, Rgba, SharedString, Window, WindowAppearance, px, rgb};
use rok_ui::hooks::State;
use rok_ui::prelude::{IconName, Theme, ThemeColors, ThemeMode, ThemePreset};

use crate::fonts;

/// The brand teal, as a hexadecimal value so tests and docs can name it.
pub const ACCENT: u32 = 0x0F_76_6E;
/// The teal a link or a pressed row turns to.
pub const ACCENT_STRONG: u32 = 0x11_5E_59;
/// The tint a selected row, a hover or a chip is filled with.
pub const ACCENT_TINT: u32 = 0xF0_FD_FA;

/// The rok POS mark's ember square. It is the product's own colour, not a
/// business's, so it stays the same whatever accent the business picks.
pub const ROK_MARK: u32 = 0xB4_40_0F;

/// The three colours a chart's series are drawn in, darkest first, so each
/// series differs in lightness as well as hue.
#[must_use]
pub fn chart_series(mode: ThemeMode) -> [Hsla; 3] {
    match mode {
        ThemeMode::Light => [hsla(ACCENT), hsla(0x5E_EA_D4), hsla(0xA8_A2_9E)],
        ThemeMode::Dark => [hsla(0x2D_D4_BF), hsla(0x13_4E_4A), hsla(0x78_71_6C)],
    }
}

/// The colour of a bar still filling up, such as the hour in progress.
#[must_use]
pub fn chart_in_progress(mode: ThemeMode) -> Hsla {
    match mode {
        ThemeMode::Light => hsla(0x99_F6_E4),
        ThemeMode::Dark => hsla(0x11_5E_59),
    }
}

/// The smallest contrast ratio a text colour may have against its surface.
pub const MINIMUM_TEXT_CONTRAST: f32 = 4.5;

/// A colour from a board, as GPUI wants it.
#[must_use]
pub fn hsla(value: u32) -> Hsla {
    Hsla::from(rgb(value))
}

/// The pharmacy theme in `mode`, with the app's fonts and square corners.
#[must_use]
pub fn pharmacy_theme(mode: ThemeMode) -> Theme {
    Theme {
        name: SharedString::from("Afya Pharmacy"),
        colors: colors(mode),
        radius: px(0.),
        font_family: SharedString::from(fonts::TEXT_FAMILY),
        monospace_font_family: SharedString::from(fonts::NUMBERS_FAMILY),
        ..Theme::from_preset(ThemePreset::Rok, mode)
    }
}

/// Install the pharmacy theme. Called once at startup, after `rok_ui::init`.
pub fn install_theme(cx: &mut App) {
    Theme::set_global(pharmacy_theme(ThemeMode::Light), cx);
}

/// What the signed-in user has chosen, and what it falls back to.
///
/// The boards are drawn in light, and a dark desktop is not a request to
/// override an explicit choice, so this is only consulted when the user has
/// not said. That is what makes the window a light pharmacy and the same window
/// a dark one when the operating system says so.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Appearance {
    /// Follow whatever the operating system says.
    #[default]
    System,
    /// Light, whatever the operating system says.
    Light,
    /// Dark, whatever the operating system says.
    Dark,
}

impl Appearance {
    /// The mode to draw in, given what the window currently reports.
    #[must_use]
    pub fn resolve(self, window: &Window) -> ThemeMode {
        self.resolve_mode(mode_of(window))
    }

    /// The mode to draw in, given the mode the system reports.
    ///
    /// Split out from [`Appearance::resolve`] so the pinned-versus-following
    /// rule is testable without a window.
    #[must_use]
    pub const fn resolve_mode(self, system: ThemeMode) -> ThemeMode {
        match self {
            Appearance::System => system,
            Appearance::Light => ThemeMode::Light,
            Appearance::Dark => ThemeMode::Dark,
        }
    }

    /// What choosing this one gives, as the toggle's label.
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Appearance::System => Appearance::Light,
            Appearance::Light => Appearance::Dark,
            Appearance::Dark => Appearance::System,
        }
    }

    /// The word the toggle shows: what the app is doing now.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Appearance::System => "System",
            Appearance::Light => "Light",
            Appearance::Dark => "Dark",
        }
    }

    /// The icon beside the word.
    ///
    /// rok-ui ships no `monitor` icon, so `System` borrows the gear: it is the
    /// one setting that is neither the sun nor the moon.
    #[must_use]
    pub const fn icon(self) -> IconName {
        match self {
            Appearance::System => IconName::Settings,
            Appearance::Light => IconName::Sun,
            Appearance::Dark => IconName::Moon,
        }
    }
}

/// Every appearance, in the order the toggle cycles through them.
pub const APPEARANCES: [Appearance; 3] = [Appearance::System, Appearance::Light, Appearance::Dark];

/// Install the theme `appearance` asks for on `window`.
///
/// Idempotent and side-effect free beyond the theme itself, so it is safe to
/// call on every frame, on every click and on every system appearance change:
/// the same answer is the same theme. What keeps the two paths in step is
/// [`observe_appearance`], which re-runs this on every system change.
pub fn apply_appearance(appearance: Appearance, window: &Window, cx: &mut App) {
    Theme::set_global(pharmacy_theme(appearance.resolve(window)), cx);
}

/// Re-apply the window's chosen `appearance` every time the window's own
/// appearance changes.
///
/// Hold the returned [`gpui::Subscription`] for as long as the window is open.
/// This is what makes a window following the system repaint when the system
/// flips, while a window the user pinned to light or dark does not: both go
/// through [`apply_appearance`], and a pinned choice resolves the same way
/// every time.
///
/// The choice is read from `chosen` on every callback rather than captured, so
/// one subscription registered at startup stays correct for as long as it
/// lives. Registering a fresh observer per click instead would accumulate
/// observers that each held a stale answer, and the last one to be created
/// would not be the last one to fire.
///
/// ```no_run
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::theme::{Appearance, observe_appearance};
/// # fn example(cx: &mut Cx) {
/// let chosen = cx.use_state(|| Appearance::System);
/// let subscription = observe_appearance(chosen.clone(), cx.window);
/// let _subscribed = cx.use_state(|| subscription);
/// # }
/// ```
pub fn observe_appearance(chosen: State<Appearance>, window: &Window) -> gpui::Subscription {
    window.observe_window_appearance(Box::new(move |window: &mut Window, cx: &mut App| {
        apply_appearance(chosen.get(cx), window, cx);
    }))
}

/// The pharmacy's tokens in `mode`.
///
/// Light is the boards' palette exactly. Dark keeps the same hues on a stone
/// background, lightened where a colour has to read as text.
#[must_use]
pub fn colors(mode: ThemeMode) -> ThemeColors {
    match mode {
        ThemeMode::Light => ThemeColors {
            background: hsla(0xFA_FA_F9),
            foreground: hsla(0x1C_19_17),
            card: hsla(0xFF_FF_FF),
            card_foreground: hsla(0x1C_19_17),
            popover: hsla(0xFF_FF_FF),
            popover_foreground: hsla(0x1C_19_17),
            primary: hsla(ACCENT),
            primary_foreground: hsla(0xFF_FF_FF),
            secondary: hsla(0xF5_F5_F4),
            secondary_foreground: hsla(0x1C_19_17),
            muted: hsla(0xF5_F5_F4),
            muted_foreground: hsla(0x57_53_4E),
            accent: hsla(ACCENT_TINT),
            accent_foreground: hsla(ACCENT_STRONG),
            destructive: hsla(0xB9_1C_1C),
            destructive_foreground: hsla(0xFF_FF_FF),
            destructive_text: hsla(0xB9_1C_1C),
            border: hsla(0xE7_E5_E4),
            input: hsla(0xD6_D3_D1),
            ring: hsla(ACCENT),
            overlay: hsla(0x1C_19_17).alpha(0.4),
        },
        ThemeMode::Dark => ThemeColors {
            background: hsla(0x1C_19_17),
            foreground: hsla(0xFA_FA_F9),
            card: hsla(0x29_25_24),
            card_foreground: hsla(0xFA_FA_F9),
            popover: hsla(0x29_25_24),
            popover_foreground: hsla(0xFA_FA_F9),
            primary: hsla(0x2D_D4_BF),
            primary_foreground: hsla(0x13_4E_4A),
            secondary: hsla(0x29_25_24),
            secondary_foreground: hsla(0xFA_FA_F9),
            muted: hsla(0x29_25_24),
            muted_foreground: hsla(0xA8_A2_9E),
            accent: hsla(0x13_4E_4A),
            accent_foreground: hsla(0x5E_EA_D4),
            destructive: hsla(0xF8_71_71),
            destructive_foreground: hsla(0x45_0A_0A),
            destructive_text: hsla(0xFC_A5_A5),
            border: hsla(0x44_40_3C),
            input: hsla(0x57_53_4E),
            ring: hsla(0x2D_D4_BF),
            overlay: hsla(0x00_00_00).alpha(0.6),
        },
    }
}

/// Follow the window's light or dark appearance, keeping the pharmacy's colours.
///
/// # Panics
///
/// Panics when `rok_ui::init` has not installed a theme.
pub fn sync_with_system_appearance(window: &Window, cx: &mut App) {
    Theme::set_global(pharmacy_theme(mode_of(window)), cx);
}

/// What `window` reports: dark or light, collapsing the vibrant variants.
fn mode_of(window: &Window) -> ThemeMode {
    match window.appearance() {
        WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeMode::Dark,
        WindowAppearance::Light | WindowAppearance::VibrantLight => ThemeMode::Light,
    }
}

/// The contrast ratio between two opaque colours, from 1 to 21, as WCAG 2.1
/// defines it.
#[must_use]
pub fn contrast_ratio(foreground: Rgba, background: Rgba) -> f32 {
    let foreground = relative_luminance(foreground);
    let background = relative_luminance(background);
    (foreground.max(background) + 0.05) / (foreground.min(background) + 0.05)
}

fn relative_luminance(color: Rgba) -> f32 {
    let channel = |value: f32| {
        if value <= 0.039_28 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(color.r) + 0.7152 * channel(color.g) + 0.0722 * channel(color.b)
}

/// The text-on-surface pairs every screen relies on, for the contrast test.
#[cfg(test)]
fn pairs(mode: ThemeMode) -> Vec<(&'static str, Hsla, Hsla)> {
    let colors = colors(mode);
    vec![
        ("body text on a card", colors.card_foreground, colors.card),
        ("body text on the app", colors.foreground, colors.background),
        (
            "secondary text on a card",
            colors.muted_foreground,
            colors.card,
        ),
        (
            "text on the accent",
            colors.accent_foreground,
            colors.accent,
        ),
        (
            "text on the brand button",
            colors.primary_foreground,
            colors.primary,
        ),
        (
            "text on the destructive button",
            colors.destructive_foreground,
            colors.destructive,
        ),
        (
            "error text on the app",
            colors.destructive_text,
            colors.background,
        ),
        ("error text on a card", colors.destructive_text, colors.card),
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        ACCENT, APPEARANCES, Appearance, MINIMUM_TEXT_CONTRAST, colors, contrast_ratio, hsla,
        pharmacy_theme,
    };
    use gpui::Rgba;
    use gpui::rgb;
    use rok_ui::prelude::IconName;
    use rok_ui::theme::{ThemeMode, ThemePreset};

    #[test]
    fn the_toggle_cycles_through_all_three_and_returns_home() {
        let mut seen = Vec::new();
        let mut appearance = Appearance::default();
        for _ in 0..APPEARANCES.len() {
            assert!(
                !seen.contains(&appearance),
                "{appearance:?} came round twice before all three were seen"
            );
            seen.push(appearance);
            appearance = appearance.next();
        }
        assert_eq!(seen, APPEARANCES.to_vec());
        assert_eq!(
            appearance,
            Appearance::System,
            "the toggle should land back where it started"
        );
    }

    #[test]
    fn a_pinned_choice_ignores_the_window() {
        // A pinned light window on a dark desktop is still light: that is the
        // whole point of pinning, so both windows resolve the same way.
        assert_eq!(
            Appearance::Light.resolve_mode(ThemeMode::Dark),
            ThemeMode::Light
        );
        assert_eq!(
            Appearance::Dark.resolve_mode(ThemeMode::Light),
            ThemeMode::Dark
        );
        assert_eq!(
            Appearance::System.resolve_mode(ThemeMode::Dark),
            ThemeMode::Dark
        );
        assert_eq!(
            Appearance::System.resolve_mode(ThemeMode::Light),
            ThemeMode::Light
        );
    }

    #[test]
    fn each_appearance_names_itself_and_its_icon() {
        assert_eq!(Appearance::System.label(), "System");
        assert_eq!(Appearance::Light.label(), "Light");
        assert_eq!(Appearance::Dark.label(), "Dark");
        assert_ne!(Appearance::Light.icon(), Appearance::Dark.icon());
        assert_ne!(Appearance::System.icon(), Appearance::Light.icon());
        assert_ne!(Appearance::System.icon(), Appearance::Dark.icon());
    }

    #[test]
    fn the_toggle_reads_as_a_distinct_icon_in_every_mode() {
        // The icon is the only thing that tells a pinned dark window from a
        // system window that happens to be dark, so the three never collide.
        let icons: Vec<IconName> = APPEARANCES.iter().map(|a| a.icon()).collect();
        for (index, icon) in icons.iter().enumerate() {
            assert!(
                !icons[..index].contains(icon),
                "{icon:?} is used twice in the toggle"
            );
        }
    }

    #[test]
    fn every_text_pair_reaches_the_contrast_floor() {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            for (name, foreground, background) in super::pairs(mode) {
                let ratio = contrast_ratio(Rgba::from(foreground), Rgba::from(background));
                assert!(
                    ratio >= MINIMUM_TEXT_CONTRAST,
                    "{name} in {mode:?} is only {ratio:.2}:1"
                );
            }
        }
    }

    #[test]
    fn the_accent_is_the_boards_teal() {
        assert_eq!(hsla(ACCENT), hsla(0x0F_76_6E));
        assert_eq!(colors(ThemeMode::Light).primary, hsla(ACCENT));
        assert_eq!(colors(ThemeMode::Light).accent, hsla(0xF0_FD_FA));
    }

    #[test]
    fn the_theme_keeps_the_preset_shape_and_the_app_fonts() {
        let theme = pharmacy_theme(ThemeMode::Light);
        assert_eq!(theme.preset, ThemePreset::Rok);
        assert_eq!(theme.radius, gpui::px(0.));
        assert_eq!(theme.font_family.as_str(), "Outfit");
        assert_eq!(theme.monospace_font_family.as_str(), "Oxanium");
    }

    #[test]
    fn white_on_the_brand_beats_the_floor() {
        let ratio = contrast_ratio(rgb(0xFF_FF_FF), rgb(ACCENT));
        assert!(
            ratio >= MINIMUM_TEXT_CONTRAST,
            "white on the brand is {ratio:.2}:1"
        );
    }
}
