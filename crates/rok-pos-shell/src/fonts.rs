//! The two families the boards use: Outfit for text, Oxanium for numbers.
//!
//! Both are static TrueType weights under `assets/fonts`, compiled into the
//! binary so an installed app needs no font files beside it. Re-download them
//! with `scripts/fetch-fonts.sh`; both are licensed under the SIL Open Font
//! License 1.1, whose text is in `assets/fonts/OFL.txt`.

use std::borrow::Cow;

use gpui::{App, SharedString};
use rok_ui::prelude::{Theme, ThemeMode};

use crate::theme;

/// The family every word on a screen is drawn in.
pub const TEXT_FAMILY: &str = "Outfit";

/// The family every amount, quantity, batch code and number is drawn in.
pub const NUMBERS_FAMILY: &str = "Oxanium";

static TEXT_WEIGHTS: [(&str, &[u8]); 4] = [
    (
        "Outfit-400",
        include_bytes!("../assets/fonts/Outfit-400.ttf"),
    ),
    (
        "Outfit-500",
        include_bytes!("../assets/fonts/Outfit-500.ttf"),
    ),
    (
        "Outfit-600",
        include_bytes!("../assets/fonts/Outfit-600.ttf"),
    ),
    (
        "Outfit-700",
        include_bytes!("../assets/fonts/Outfit-700.ttf"),
    ),
];

static NUMBERS_WEIGHTS: [(&str, &[u8]); 3] = [
    (
        "Oxanium-500",
        include_bytes!("../assets/fonts/Oxanium-500.ttf"),
    ),
    (
        "Oxanium-600",
        include_bytes!("../assets/fonts/Oxanium-600.ttf"),
    ),
    (
        "Oxanium-700",
        include_bytes!("../assets/fonts/Oxanium-700.ttf"),
    ),
];

/// Every embedded weight, for registration and for the tests.
fn weights() -> Vec<(&'static str, &'static [u8])> {
    TEXT_WEIGHTS.into_iter().chain(NUMBERS_WEIGHTS).collect()
}

/// Register both families with GPUI. Called once, before the first window opens.
///
/// # Errors
///
/// Fails when the text system rejects a font file.
pub fn register(cx: &App) -> gpui::Result<()> {
    rok_ui::fonts::register_fonts(
        cx,
        weights().into_iter().map(|(_, bytes)| Cow::Borrowed(bytes)),
    )
}

/// Register both families and point the theme at them: Outfit for text,
/// Oxanium for the `monospace` token, which is where amounts are drawn.
///
/// # Errors
///
/// Fails when the text system rejects a font file.
pub fn install(cx: &mut App) -> gpui::Result<()> {
    register(cx)?;
    let mut theme = theme::pharmacy_theme(ThemeMode::Light);
    theme.font_family = SharedString::from(TEXT_FAMILY);
    theme.monospace_font_family = SharedString::from(NUMBERS_FAMILY);
    Theme::set_global(theme, cx);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{NUMBERS_FAMILY, TEXT_FAMILY, weights};

    /// A TrueType or OpenType file starts with one of these tags.
    const SFNT_TAGS: [&[u8; 4]; 3] = [b"\x00\x01\x00\x00", b"true", b"OTTO"];

    #[test]
    fn every_embedded_weight_is_a_whole_font_file() {
        for (name, bytes) in weights() {
            assert!(
                SFNT_TAGS.iter().any(|tag| bytes.starts_with(&tag[..])),
                "{name} does not start with a font tag"
            );
            assert!(
                bytes.len() > 10_000,
                "{name} looks truncated at {} bytes",
                bytes.len()
            );
        }
    }

    #[test]
    fn the_family_names_are_the_ones_the_theme_asks_for() {
        assert_eq!(TEXT_FAMILY, "Outfit");
        assert_eq!(NUMBERS_FAMILY, "Oxanium");
    }

    #[test]
    fn weights_come_back_named_after_their_family() {
        let names: Vec<&str> = weights().into_iter().map(|(name, _)| name).collect();
        assert_eq!(
            names,
            [
                "Outfit-400",
                "Outfit-500",
                "Outfit-600",
                "Outfit-700",
                "Oxanium-500",
                "Oxanium-600",
                "Oxanium-700"
            ]
        );
    }
}
