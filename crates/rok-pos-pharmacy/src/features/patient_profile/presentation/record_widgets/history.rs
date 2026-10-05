//! The patient's fills, newest first, each opening the prescription it came
//! from.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;
use rust_i18n::t;

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
            t!("patient.record.history", count = patient.fill_count()).to_string(),
            t!("patient.record.print").to_string(),
        ))
        .child(board::head(vec![
            board::cell_fixed(t!("patient.record.col.date").to_string()),
            board::cell_fixed(t!("patient.record.col.prescription").to_string()),
            board::cell_fixed(t!("patient.record.col.medicines").to_string()),
            board::cell_fixed(t!("patient.record.col.pharmacist").to_string()),
            board::cell_fixed(t!("patient.record.col.status").to_string()),
        ]))
        .children(rows)
}
