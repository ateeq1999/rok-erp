//! The pharmacist's notes, which only a pharmacist may read.

use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::patient_profile::domain::entities::Patient;
use crate::features::shared::board;

/// The clinical notes.
pub(crate) fn card(patient: &Patient) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Clinical notes",
            "Only pharmacists see clinical notes",
        ))
        .children(
            patient
                .notes
                .iter()
                .enumerate()
                .map(|(index, note)| {
                    board::toned_row(
                        Tone::Brand,
                        ThemeMode::Light,
                        vec![
                            board::cell_stack(note.title.to_string(), note.meta.to_string()),
                            board::meta(note.body.to_string()).into_any_element(),
                        ],
                    )
                    .id(("patient-note", index))
                    .into_any_element()
                })
                .collect::<Vec<_>>(),
        )
        .child(board::actions(vec![
            Button::new("add-note")
                .label("Add clinical note")
                .into_any_element(),
        ]))
}
