//! The boards' semantic colours: the status tones a chip, an alert or a table
//! row is filled with.
//!
//! shadcn/ui's tokens cover the brand and the destructive case. A pharmacy also
//! needs to say "this batch expires in 30 days", "this claim was queried" and
//! "this patient consented" at a glance, so the boards give each of those a
//! background and a text colour. Those pairs live here, in both modes.

use gpui::Hsla;
use rok_ui::prelude::ThemeMode;

use crate::theme::{ACCENT_STRONG, ACCENT_TINT, hsla};

/// What a chip, an alert or a row is telling the user.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Tone {
    /// Nothing to act on: a closed claim, a stock count that matched.
    #[default]
    Neutral,
    /// The pharmacy's own action: a recall to open, a link to follow.
    Brand,
    /// Worth knowing: a delivery to receive, a query to read.
    Info,
    /// Done, allowed or in range: consent given, cold chain within 2-8 degrees.
    Success,
    /// Time is running out: a licence expiring, a shelf life about to end.
    Warning,
    /// Stop: an expired batch, a failed interaction check, a blocked recall.
    Danger,
}

impl Tone {
    /// Every tone, in the order a screen usually lists them.
    pub const ALL: [Tone; 6] = [
        Tone::Neutral,
        Tone::Brand,
        Tone::Info,
        Tone::Success,
        Tone::Warning,
        Tone::Danger,
    ];
}

/// A tone's background and the text drawn on it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToneColors {
    /// The fill behind the text.
    pub background: Hsla,
    /// The text, always at least 4.5:1 against `background`.
    pub foreground: Hsla,
}

/// The colours for `tone` in `mode`, as the boards draw them in light mode and
/// as the same hues keep their meaning on a dark background.
#[must_use]
pub fn colors(tone: Tone, mode: ThemeMode) -> ToneColors {
    let light = !mode.is_dark();
    let pair = match tone {
        Tone::Neutral => {
            if light {
                (0xF5_F5_F4, 0x44_40_3C)
            } else {
                (0x44_40_3C, 0xFA_FA_F9)
            }
        }
        Tone::Brand => {
            if light {
                (ACCENT_TINT, ACCENT_STRONG)
            } else {
                (0x13_4E_4A, 0x5E_EA_D4)
            }
        }
        Tone::Info => {
            if light {
                (0xEF_F6_FF, 0x1E_40_AF)
            } else {
                (0x1E_3A_8A, 0xBF_DB_FE)
            }
        }
        Tone::Success => {
            if light {
                (0xEC_FD_F3, 0x16_65_34)
            } else {
                (0x14_53_2D, 0x86_EF_AC)
            }
        }
        Tone::Warning => {
            if light {
                (0xFF_FB_EB, 0x92_40_0E)
            } else {
                (0x78_35_0F, 0xFD_E6_8A)
            }
        }
        Tone::Danger => {
            if light {
                (0xFE_F2_F2, 0xB9_1C_1C)
            } else {
                (0x7F_1D_1D, 0xFE_CA_CA)
            }
        }
    };
    ToneColors {
        background: hsla(pair.0),
        foreground: hsla(pair.1),
    }
}

#[cfg(test)]
mod tests {
    use super::{Tone, colors};
    use crate::theme::{MINIMUM_TEXT_CONTRAST, contrast_ratio};
    use gpui::Rgba;
    use rok_ui::theme::ThemeMode;

    #[test]
    fn every_tone_is_readable_in_both_modes() {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            for tone in Tone::ALL {
                let pair = colors(tone, mode);
                let ratio =
                    contrast_ratio(Rgba::from(pair.foreground), Rgba::from(pair.background));
                assert!(
                    ratio >= MINIMUM_TEXT_CONTRAST,
                    "{tone:?} in {mode:?} is only {ratio:.2}:1"
                );
            }
        }
    }

    #[test]
    fn the_boards_brand_tone_is_its_tint_and_hover_colour() {
        let brand = colors(Tone::Brand, ThemeMode::Light);
        assert_eq!(brand.background, crate::theme::hsla(0xF0_FD_FA));
        assert_eq!(brand.foreground, crate::theme::hsla(0x11_5E_59));
    }
}
