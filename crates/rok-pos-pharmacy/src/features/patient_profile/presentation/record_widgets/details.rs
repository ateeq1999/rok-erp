//! Who the patient is, who pays for them, and what they agreed to.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;
use rust_i18n::t;

use crate::features::patient_profile::domain::entities::Patient;
use crate::features::shared::board;

/// The details and the insurance panel.
pub(crate) fn card(patient: &Patient) -> Div {
    board::card(1.)
        .child(board::card_head(
            board::dotted(&[&patient.initials, &patient.name]),
            t!("patient.record.head_meta").to_string(),
        ))
        .child(board::panel(
            patient
                .details
                .iter()
                .map(|detail| {
                    board::key_value(detail.label.to_string(), detail.value.to_string())
                        .into_any_element()
                })
                .collect::<Vec<_>>(),
        ))
        .child(board::card_title(
            t!("patient.record.insurance").to_string(),
        ))
        .child(board::meta(patient.insurer.to_string()))
        .child(board::chip(patient.cover.to_string(), Tone::Success))
        .child(board::card_title(
            t!("patient.record.reminders").to_string(),
        ))
        .child(board::chip(
            if patient.wants_reminders() {
                t!("patient.record.reminders_on").to_string()
            } else {
                t!("patient.record.reminders_off").to_string()
            },
            Tone::Brand,
        ))
        .child(board::meta(patient.consent.to_string()))
}
