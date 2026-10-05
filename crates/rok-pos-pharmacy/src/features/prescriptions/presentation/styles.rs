//! The queue's own styles on top of the shared board toolkit.
//!
//! The queue draws with `screens::board`, so most of its shapes come from there;
//! what is here is what only this board needs.

use rok_ui::prelude::*;

styles! {
    pub QUEUE = {
        status_box: {
            display: flex,
            flex_direction: row,
            gap: 2,
            align: center,
            padding_y: 1,
            padding_x: 2,
            radius: none,
        },
        status_count: { font_family: mono, text: {18.}, font: semibold },
        ways: { display: flex, flex_direction: row, gap: 1.5 },
        way: {
            padding_x: 2,
            padding_y: {px(4.)},
            text: {13.},
            border: 1,
            border_color: border,
            radius: none,
            cursor: pointer,
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
        selected_row: { background: muted },
    }
}
