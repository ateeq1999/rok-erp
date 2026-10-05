//! The directory: one row per patient, opening the record it names.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::patient_profile::domain::entities::Listing;
use crate::features::patient_profile::domain::enums::Urgency;
use crate::features::shared::board;

/// How the list tints a row's next-due chip.
fn row_tone(listing: &Listing) -> Tone {
    match listing.urgency {
        Urgency::ThisWeek => Tone::Warning,
        Urgency::Planned => Tone::Brand,
        Urgency::Nothing => Tone::Neutral,
    }
}

/// The pharmacy's patients, most urgent first.
pub(crate) fn card(listings: &[Listing]) -> Div {
    let rows = listings
        .iter()
        .enumerate()
        .map(|(index, listing)| {
            let href = listing.href.to_string();
            board::link_to(("patient-row", index), href.clone())
                .sx(board::line_style())
                .child(board::cell_stack(
                    board::dotted(&[&listing.initials, &listing.name]),
                    listing.phone.to_string(),
                ))
                .child(board::cell(listing.phone.to_string()))
                .child(board::cell(listing.cover.label()))
                .child(board::cell(listing.conditions.to_string()))
                .child(
                    listing
                        .next_due
                        .as_ref()
                        .map_or_else(
                            || board::chip("Nothing due", Tone::Neutral),
                            |due| board::chip(due.to_string(), row_tone(listing)),
                        )
                        .into_any_element(),
                )
                .child(board::link(("patient-open", index), href, "Open record"))
                .into_any_element()
        })
        .collect::<Vec<_>>();
    board::card(1.)
        .child(board::card_head("Patients", "Newest refill first"))
        .child(board::head(vec![
            board::cell("Patient"),
            board::cell("Phone"),
            board::cell("Cover"),
            board::cell("Conditions"),
            board::cell("Next due"),
            board::cell_fixed(""),
        ]))
        .children(rows)
        .child(board::footnote(
            "The row and the link both open the record; the phone is there for the call.",
        ))
}
