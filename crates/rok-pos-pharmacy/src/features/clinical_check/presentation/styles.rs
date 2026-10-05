//! The clinical check's own styles on top of the shared board toolkit.

use rok_ui::prelude::*;

styles! {
    pub CHECK = {
        scan: {
            height: 40,
            display: flex,
            flex_direction: column,
            align: center,
            justify: center,
            gap: 1.5,
            background: muted,
            border: 1,
            border_color: border,
        },
        scan_title: { font: semibold, text_align: center },
        scan_meta: { text: {12.}, color: muted_foreground, text_align: center },
        numbered: { width: 5, shrink: 0, text_align: center, color: muted_foreground },
        label: {
            display: flex,
            flex_direction: column,
            gap: 1.5,
            padding: 3,
            background: card,
            border: 1,
            border_color: border,
        },
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
