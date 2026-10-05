//! The medicines the patient takes now, and when each is next due.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::patient_profile::domain::entities::Patient;
use crate::features::shared::board;

/// The medicines the patient takes now.
pub(crate) fn card(patient: &Patient) -> Div {
    let rows = patient
        .medicines
        .iter()
        .enumerate()
        .map(|(index, medicine)| {
            board::line(vec![
                board::cell_fixed_stack(medicine.name.to_string(), medicine.directions.to_string()),
                board::cell_fixed(medicine.last_filled.to_string()),
                board::chip(
                    medicine.due.to_string(),
                    if medicine.is_a_pharmacy_refill() {
                        Tone::Warning
                    } else {
                        Tone::Neutral
                    },
                ),
            ])
            .id(("patient-medicine", index))
            .into_any_element()
        })
        .collect::<Vec<_>>();
    board::card(2.2)
        .child(board::card_head("Current medicines", "All refills due"))
        .child(board::head(vec![
            board::cell_fixed("Medicine"),
            board::cell_fixed("Last filled"),
            board::cell_fixed("Refill due"),
        ]))
        .children(rows)
}
