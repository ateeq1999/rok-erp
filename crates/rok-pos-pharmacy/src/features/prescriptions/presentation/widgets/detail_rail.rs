//! The rail: the open prescription, its alert, its items and its buttons.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;
use rok_ui::router::navigate;

use crate::features::prescriptions::domain::enums::Destination;
use crate::features::prescriptions::presentation::prescriptions_screen::route;
use crate::features::prescriptions::presentation::styles::QUEUE;
use crate::features::shared::board;

/// One prescribed medicine, as the screen resolves it.
pub(crate) struct ItemView {
    /// The medicine as prescribed.
    pub name: String,
    /// How many were prescribed.
    pub quantity: u32,
    /// The directions.
    pub directions: String,
    /// Whether it can be sold right now, and from which batch.
    pub availability: String,
    /// Whether the shelf has it.
    pub in_stock: bool,
}

/// Everything the rail draws about the open prescription.
pub(crate) struct RailView {
    /// Its reference.
    pub reference: String,
    /// How it reached the pharmacy and when.
    pub arrived: String,
    /// Where it has got to, as the board writes it.
    pub status: String,
    /// How the status chip is coloured.
    pub tone: Tone,
    /// Who it is for.
    pub patient: String,
    /// The patient line under the name.
    pub patient_detail: String,
    /// The clinical alert, if the check found one.
    pub alert: Option<(String, String)>,
    /// What was prescribed.
    pub items: Vec<ItemView>,
    /// What the patient already takes.
    pub also_takes: String,
    /// Known allergies.
    pub allergies: String,
    /// Who pays.
    pub paid_by: String,
    /// Who prescribed it.
    pub prescriber: String,
    /// The rule under the buttons.
    pub rule: String,
    /// Whether a pharmacist has to sign this one before it is dispensed.
    pub needs_a_pharmacist: bool,
}

/// The rail.
pub(crate) fn card(open: &RailView, mode: ThemeMode) -> Div {
    let items: Vec<AnyElement> = open
        .items
        .iter()
        .map(|item| {
            board::line(vec![
                board::cell_stack(item.name.clone(), item.directions.clone()),
                board::cell_number(format!("\u{d7} {}", item.quantity)),
                board::chip(
                    item.availability.clone(),
                    if item.in_stock {
                        Tone::Success
                    } else {
                        Tone::Warning
                    },
                ),
            ])
            .into_any_element()
        })
        .collect();
    let check: &str =
        Box::leak(format!("/prescriptions/{}/check", open.reference).into_boxed_str());
    let approve = board::actions(vec![
        Button::new("check-and-dispense")
            .on_click(move |_, _, cx| navigate(check, cx))
            .label(if open.needs_a_pharmacist {
                "Pharmacist check"
            } else {
                "Check and dispense"
            })
            .variant(ButtonVariant::Primary)
            .into_any_element(),
        Button::new("view-image")
            .label("View prescription image")
            .into_any_element(),
        Button::new("put-on-hold")
            .label("Put on hold")
            .into_any_element(),
    ]);
    board::card(1.1)
        .child(board::card_head(
            open.reference.clone(),
            open.arrived.clone(),
        ))
        .child(board::chip_line(vec![board::chip(
            open.status.clone(),
            open.tone,
        )]))
        .child(board::cell_stack(
            open.patient.clone(),
            open.patient_detail.clone(),
        ))
        .child(
            div()
                .id("queue-patient-record")
                .tab_index(0)
                .on_click(|_, _, cx| navigate(route(Destination::PatientRecord), cx))
                .sx(&QUEUE.way)
                .child("Record"),
        )
        .when_some(open.alert.clone(), |rail, (title, body)| {
            rail.child(board::alert(title, body, Tone::Danger, mode))
        })
        .child(board::section_label(
            "New prescription \u{b7} dental clinic",
        ))
        .children(items)
        .child(board::panel(vec![
            board::key_value("Also takes:", open.also_takes.clone()).into_any_element(),
            board::key_value("Allergies:", open.allergies.clone()).into_any_element(),
            board::key_value("Paid by:", open.paid_by.clone()).into_any_element(),
            board::key_value("Prescriber:", open.prescriber.clone()).into_any_element(),
        ]))
        .child(approve)
        .child(board::footnote(open.rule.clone()))
}
