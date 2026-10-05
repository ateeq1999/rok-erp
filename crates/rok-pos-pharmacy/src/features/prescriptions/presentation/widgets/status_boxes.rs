//! "Today's queue by status": the five boxes across the top.

use gpui::prelude::*;
use rok_pos_shell::{Tone, tone};
use rok_ui::prelude::*;

use crate::features::prescriptions::presentation::styles::QUEUE;
use crate::screens::board;

/// One box, with its chip, its count and the note under it.
pub(crate) fn card(label: &str, count: u32, note: &str, severity: Tone, mode: ThemeMode) -> Div {
    let palette = tone::colors(severity, mode);
    board::card(1.4)
        .child(board::card_title(label.to_string()))
        .child(
            div()
                .sx(sx![
                    &QUEUE.status_box,
                    style! { background: {palette.background} }
                ])
                .child(
                    div()
                        .sx(&QUEUE.status_count)
                        .text_color(palette.foreground)
                        .child(count.to_string()),
                )
                .child(board::meta(note.to_string())),
        )
}
