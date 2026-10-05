//! The patient's fills, newest first, each opening the prescription it came
//! from.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::patient_profile::domain::entities::Patient;
use crate::features::shared::board;

/// The dispensing history.
pub(crate) fn card(patient: &Patient) -> Div {
    let rows = patient
        .fills
        .iter()
        .enumerate()
        .map(|(index, fill)| {
            board::link_to(("patient-fill", index), "/prescriptions")
                .sx(board::line_style())
                .child(board::cell_fixed_stack(
                    fill.date.to_string(),
                    fill.reference.to_string(),
                ))
                .child(board::cell_stack(
                    fill.medicines.to_string(),
                    fill.prescriber.to_string(),
                ))
                .child(board::cell_fixed(fill.pharmacist.to_string()))
                .child(board::chip(fill.status.to_string(), Tone::Success))
        })
        .collect::<Vec<_>>();
    board::card(2.2)
        .child(board::card_head(
            format!(
                "Dispensing history \u{b7} last {} fills",
                patient.fill_count()
            ),
            "Print history",
        ))
        .child(board::head(vec![
            board::cell_fixed("Date"),
            board::cell_fixed("Prescription"),
            board::cell_fixed("Medicines"),
            board::cell_fixed("Pharmacist"),
            board::cell_fixed("Status"),
        ]))
        .children(rows)
}
