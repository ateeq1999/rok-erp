//! The reports screen's own styles on top of the shared board toolkit.
//!
//! The board draws with `features::shared::board`, so most of its shapes come
//! from there; what is here is what the loading and failure notices need and
//! the board does not have.

use rok_ui::prelude::*;

styles! {
    pub REPORT = {
        notice: {
            display: flex,
            flex_direction: column,
            gap: 2,
            padding: 6,
            background: card,
            border: 1,
            border_color: border,
        },
        notice_title: { font: semibold },
        notice_body: { text: {13.}, color: muted_foreground },
    }
}
