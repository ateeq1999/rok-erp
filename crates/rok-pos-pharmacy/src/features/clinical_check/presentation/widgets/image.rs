//! The scan of the paper and the details that came with it.

use gpui::prelude::*;
use rok_ui::prelude::*;

use crate::features::clinical_check::domain::enums::Destination;
use crate::features::clinical_check::presentation::check_screen::route;
use crate::features::clinical_check::presentation::styles::CHECK;
use crate::features::shared::board;

/// The rail: the scan, its controls, and the prescription's details.
pub(crate) fn card(view: &RailView) -> Div {
    board::column()
        .child(board::link(
            "check-back",
            route(Destination::Prescriptions),
            "Back to prescription queue",
        ))
        .child(board::section_label("PRESCRIPTION IMAGE"))
        .child(
            div()
                .sx(&CHECK.scan)
                .child(div().sx(&CHECK.scan_title).child("PRESCRIPTION SCAN"))
                .child(
                    div()
                        .sx(&CHECK.scan_meta)
                        .child(format!("Paper from {}", view.clinic)),
                )
                .child(
                    div()
                        .sx(&CHECK.scan_meta)
                        .child(format!("Scanned {}", view.scanned)),
                ),
        )
        .child(board::actions(vec![
            Button::new("check-zoom").label("Zoom").into_any_element(),
            Button::new("check-rotate")
                .label("Rotate")
                .into_any_element(),
        ]))
        .child(board::card(1.).child(board::panel(vec![
            board::link(
                "check-patient",
                route(Destination::PatientRecord),
                view.patient.clone(),
            )
            .into_any_element(),
            board::key_value("Age", view.age.clone()).into_any_element(),
            board::key_value("Prescriber", view.prescriber.clone()).into_any_element(),
            board::key_value("Clinic", view.clinic.clone()).into_any_element(),
            board::key_value("Issued", view.issued.clone()).into_any_element(),
            board::key_value("Reference", view.reference.clone()).into_any_element(),
        ])))
}

/// Everything the rail draws, as the screen resolves it.
pub(crate) struct RailView {
    /// Its reference.
    pub reference: String,
    /// The patient it is for.
    pub patient: String,
    /// The patient's age and sex.
    pub age: String,
    /// Who prescribed it.
    pub prescriber: String,
    /// Where they practice.
    pub clinic: String,
    /// When the paper was issued.
    pub issued: String,
    /// When the paper was scanned.
    pub scanned: String,
}
