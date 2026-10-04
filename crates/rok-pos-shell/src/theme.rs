//! The pharmacy theme: the Rok preset with Afya Pharmacy's teal accent.
//!
//! Every colour here is taken from the boards in `design/`. A screen never
//! hard-codes one: it reads a theme token, so light and dark both work and the
//! supplier app can use the same frame with its own accent.

use gpui::{App, Hsla, Rgba, SharedString, Window, WindowAppearance, px, rgb};
use rok_ui::prelude::{Theme, ThemeColors, ThemeMode, ThemePreset};

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
    let mode = match window.appearance() {
        WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeMode::Dark,
        WindowAppearance::Light | WindowAppearance::VibrantLight => ThemeMode::Light,
    };
    Theme::set_global(pharmacy_theme(mode), cx);
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
    use super::{ACCENT, MINIMUM_TEXT_CONTRAST, colors, contrast_ratio, hsla, pharmacy_theme};
    use gpui::Rgba;
    use gpui::rgb;
    use rok_ui::theme::{ThemeMode, ThemePreset};

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
