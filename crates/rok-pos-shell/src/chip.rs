//! `Chip`: the status label the boards put next to a claim, a batch or a
//! controlled-substance row.
//!
//! shadcn/ui's badge has one destructive variant; the boards need six tones, so
//! [`Chip`] is a [`Badge`] filled from [`crate::tone`] in the current mode.

use rok_ui::prelude::*;
use rok_ui::sx::SxStyled;

use crate::tone::{self, Tone};

/// A short status: "Ready to dispense", "Consent on file", "Expires soon".
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::{Chip, Tone};
/// # let _ = Chip::new("Ready to dispense").tone(Tone::Success).icon(IconName::Check);
/// ```
#[component]
pub fn Chip(
    label: SharedString,
    #[default] tone: Tone,
    #[default] icon: Option<IconName>,
    cx: &mut Cx,
) -> impl IntoElement {
    let colors = tone::colors(tone, cx.theme().mode);
    let mut badge = Badge::new(label).sx(style! {
        background: {colors.background},
        color: {colors.foreground},
        border_color: {colors.background},
        radius: none,
        padding_x: 2,
    });
    if let Some(icon) = icon {
        badge = badge.icon(icon);
    }
    badge
}

#[cfg(test)]
mod tests {
    use super::Chip;
    use crate::tone::Tone;
    use rok_ui::prelude::*;

    struct EveryTone;

    impl Render for EveryTone {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children(Tone::ALL.into_iter().map(|tone| {
                Chip::new("Ready to dispense")
                    .tone(tone)
                    .icon(IconName::Check)
            }))
        }
    }

    #[gpui::test]
    fn draws_every_tone(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| EveryTone);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
