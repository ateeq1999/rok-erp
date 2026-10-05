//! The patient screens' own styles on top of the shared board toolkit.
//!
//! The record and the list draw with `features::shared::board`, so most of
//! their shapes come from there; what is here is what both of them need and the
//! board does not have.

use rok_ui::prelude::*;

styles! {
    pub PATIENT = {
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
