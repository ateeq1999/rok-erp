//! The counselling points: what the patient is told, which the label alone does
//! not carry.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::clinical_check::presentation::styles::CHECK;
use crate::features::shared::board;

/// The points, in the pharmacist's own order.
pub(crate) fn card(points: &[String]) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Counselling points",
            format!("{} points", points.len()),
        ))
        .children(points.iter().enumerate().map(|(index, point)| {
            board::line(vec![
                div()
                    .sx(&CHECK.numbered)
                    .child((index + 1).to_string())
                    .into_any_element(),
                board::cell(point.clone()),
            ])
        }))
        .child(board::chip_line(vec![board::chip(
            "Said before the label is handed over",
            Tone::Info,
        )]))
}
