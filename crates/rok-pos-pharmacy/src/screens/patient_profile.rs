//! A patient's record: who they are, what they take, what has been dispensed
//! and what the pharmacists have written down.
//!
//! Board: `Pharmacy_patient_record.html`. The board's own patient lives in
//! [`Patient::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// One fact about a patient, as the board's detail panel pairs it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Detail {
    /// What the fact is.
    pub label: &'static str,
    /// The value beside it.
    pub value: &'static str,
}

/// One of the patient's long-term conditions, as the board lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Condition {
    /// What it is.
    pub name: &'static str,
    /// Where it came from.
    pub source: &'static str,
}

/// One medicine the patient takes, and when it is next due.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Medicine {
    /// The medicine.
    pub name: &'static str,
    /// How it is taken.
    pub directions: &'static str,
    /// When it was last filled.
    pub last_filled: &'static str,
    /// When the next one is due, or who sets that.
    pub due: &'static str,
}

/// One past fill.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fill {
    /// When.
    pub date: &'static str,
    /// Which prescription.
    pub reference: &'static str,
    /// What was dispensed.
    pub medicines: &'static str,
    /// Who prescribed it.
    pub prescriber: &'static str,
    /// Who dispensed it.
    pub pharmacist: &'static str,
    /// Where it got to.
    pub status: &'static str,
}

/// One clinical note, which only a pharmacist may read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    /// Its title.
    pub title: &'static str,
    /// When it was written and by whom.
    pub meta: &'static str,
    /// What it says.
    pub body: &'static str,
}

/// Everything the record draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Patient {
    /// Their initials, as the board's avatar shows them.
    pub initials: &'static str,
    /// Their name.
    pub name: &'static str,
    /// The details the board pairs off.
    pub details: Vec<Detail>,
    /// Who pays for them and whether they are covered.
    pub insurer: &'static str,
    /// Whether the insurer covers them today.
    pub cover: &'static str,
    /// Their known allergies.
    pub allergies: &'static str,
    /// When the allergies were confirmed.
    pub allergies_confirmed: &'static str,
    /// What the board says about where allergies come from.
    pub allergy_source: &'static str,
    /// Their long-term conditions.
    pub conditions: Vec<Condition>,
    /// Whether they get refill reminders, and on what terms.
    pub reminders: &'static str,
    /// When consent was given.
    pub consent: &'static str,
    /// What they take now.
    pub medicines: Vec<Medicine>,
    /// Their fills, newest first.
    pub fills: Vec<Fill>,
    /// Their notes, newest first.
    pub notes: Vec<Note>,
}

impl Patient {
    /// The board's patient, Mzee Salim R.
    #[must_use]
    pub fn story() -> Self {
        Self {
            initials: "SR",
            name: "Mzee Salim R.",
            details: vec![
                Detail {
                    label: "Patient since",
                    value: "03/2024",
                },
                Detail {
                    label: "Phone",
                    value: "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 508",
                },
                Detail {
                    label: "Date of birth",
                    value: "[Date of birth] \u{b7} 68 years",
                },
                Detail {
                    label: "Sex",
                    value: "Male",
                },
                Detail {
                    label: "Area",
                    value: "Mwenge, Dar es Salaam",
                },
                Detail {
                    label: "Preferred language",
                    value: "Kiswahili",
                },
            ],
            insurer: "National health insurance \u{b7} member 10-xxxx-508",
            cover: "Active \u{b7} checked 02 Oct 14:50",
            allergies: "none known \u{b7} confirmed 02 Oct",
            allergies_confirmed: "Allergies:",
            allergy_source: "From prescriptions on file. As written by prescribers. Not a diagnosis by the pharmacy.",
            conditions: vec![
                Condition {
                    name: "On warfarin therapy (followed by warfarin clinic)",
                    source: "prescription",
                },
                Condition {
                    name: "Type 2 diabetes",
                    source: "prescription",
                },
                Condition {
                    name: "High blood pressure",
                    source: "prescription",
                },
            ],
            reminders: "On",
            consent: "Consent given at the counter 14 Mar 2024. Kiswahili messages.",
            medicines: vec![
                Medicine {
                    name: "Warfarin tablets",
                    directions: "As directed by the clinic",
                    last_filled: "18 Sep 2026",
                    due: "Set by clinic",
                },
                Medicine {
                    name: "Metformin 500mg tablets",
                    directions: "1 tablet twice a day with food",
                    last_filled: "05 Sep 2026",
                    due: "Due 5 Oct",
                },
                Medicine {
                    name: "Amlodipine 5mg tablets",
                    directions: "1 tablet once a day",
                    last_filled: "12 Sep 2026",
                    due: "Due 12 Oct",
                },
            ],
            fills: vec![
                fill(
                    "02 Oct 2026",
                    "RX-2214",
                    "Metronidazole 400mg \u{d7} 21, Amoxicillin 500mg \u{d7} 15",
                    "[Dental clinic name]",
                    "Grace N.",
                    "Approved, at till",
                ),
                fill(
                    "18 Sep 2026",
                    "RX-2068",
                    "Warfarin, as directed",
                    "[Warfarin clinic]",
                    "Grace N.",
                    "Collected",
                ),
                fill(
                    "12 Sep 2026",
                    "RX-2012",
                    "Amlodipine 5mg \u{d7} 30",
                    "[Prescriber name]",
                    "John M.",
                    "Collected",
                ),
                fill(
                    "05 Sep 2026",
                    "RX-1944",
                    "Metformin 500mg \u{d7} 60",
                    "[Prescriber name]",
                    "Grace N.",
                    "Collected",
                ),
                fill(
                    "21 Aug 2026",
                    "RX-1871",
                    "Warfarin, as directed",
                    "[Warfarin clinic]",
                    "Grace N.",
                    "Collected",
                ),
                fill(
                    "13 Aug 2026",
                    "RX-1839",
                    "Amlodipine 5mg \u{d7} 30",
                    "[Prescriber name]",
                    "John M.",
                    "Collected",
                ),
            ],
            notes: vec![
                Note {
                    title: "Prescriber call \u{b7} RX-2214",
                    meta: "02 Oct 15:10 \u{b7} Grace N.",
                    body: "Interaction alert warfarin + metronidazole. Called [Dental clinic name], spoke to [Prescriber name]. Prescriber agreed to keep both and asked for an extra INR check.",
                },
                Note {
                    title: "Counselling",
                    meta: "18 Sep 14:22 \u{b7} Grace N.",
                    body: "Patient asked about pain relief. Advised to ask the pharmacist before taking any new medicine, including painkillers bought elsewhere.",
                },
            ],
        }
    }

    /// How many fills the board shows.
    #[must_use]
    pub fn fill_count(&self) -> usize {
        self.fills.len()
    }

    /// The medicine whose refill is due soonest, which is the reason to open
    /// this record from the refills board.
    #[must_use]
    pub fn next_due(&self) -> Option<&Medicine> {
        self.medicines
            .iter()
            .find(|medicine| medicine.due.starts_with("Due"))
    }

    /// Whether the patient has consented to refill reminders.
    #[must_use]
    pub fn wants_reminders(&self) -> bool {
        self.reminders == "On"
    }
}

/// Build one fill; the history is a list of the same shape.
#[allow(clippy::too_many_arguments)]
const fn fill(
    date: &'static str,
    reference: &'static str,
    medicines: &'static str,
    prescriber: &'static str,
    pharmacist: &'static str,
    status: &'static str,
) -> Fill {
    Fill {
        date,
        reference,
        medicines,
        prescriber,
        pharmacist,
        status,
    }
}

/// The details and the insurance panel.
fn details(patient: &Patient) -> Div {
    board::card(1.)
        .child(board::card_head(
            board::dotted(&[patient.initials, patient.name]),
            "Record",
        ))
        .child(board::panel(
            patient
                .details
                .iter()
                .map(|detail| board::key_value(detail.label, detail.value).into_any_element())
                .collect::<Vec<_>>(),
        ))
        .child(board::card_title("Health insurance"))
        .child(board::meta(patient.insurer))
        .child(board::chip(patient.cover, Tone::Success))
        .child(board::card_title("Reminders"))
        .child(board::chip(patient.reminders, Tone::Brand))
        .child(board::meta(patient.consent))
}

/// The allergies and conditions.
fn clinical(patient: &Patient) -> Div {
    board::card(1.)
        .child(board::card_head("Allergies", patient.allergies_confirmed))
        .child(board::chip(patient.allergies, Tone::Success))
        .child(board::footnote(patient.allergy_source))
        .child(board::card_title("From prescriptions on file"))
        .child(board::list(
            patient
                .conditions
                .iter()
                .map(|condition| board::option(condition.name).into_any_element())
                .collect::<Vec<_>>(),
        ))
}

/// The medicines the patient takes now.
fn current(patient: &Patient) -> Div {
    let rows = patient
        .medicines
        .iter()
        .enumerate()
        .map(|(index, medicine)| {
            board::line(vec![
                board::cell_fixed_stack(medicine.name, medicine.directions),
                board::cell_fixed(medicine.last_filled),
                board::chip(
                    medicine.due,
                    if medicine.due.starts_with("Due") {
                        Tone::Warning
                    } else {
                        Tone::Neutral
                    },
                ),
            ])
            .id(("patient-medicine", index))
            .into_any_element()
        });
    board::card(2.2)
        .child(board::card_head("Current medicines", "All refills due"))
        .child(board::head(vec![
            board::cell_fixed("Medicine"),
            board::cell_fixed("Last filled"),
            board::cell_fixed("Refill due"),
        ]))
        .children(rows.collect::<Vec<_>>())
}

/// The dispensing history.
fn history(patient: &Patient) -> Div {
    let rows = patient.fills.iter().enumerate().map(|(index, fill)| {
        board::link_to(("patient-fill", index), "/prescriptions")
            .sx(board::line_style())
            .child(board::cell_fixed_stack(fill.date, fill.reference))
            .child(board::cell_stack(fill.medicines, fill.prescriber))
            .child(board::cell_fixed(fill.pharmacist))
            .child(board::chip(fill.status, Tone::Success))
    });
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
        .children(rows.collect::<Vec<_>>())
}

/// The clinical notes.
fn notes(patient: &Patient) -> Div {
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
                            board::cell_stack(note.title, note.meta),
                            board::meta(note.body).into_any_element(),
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

/// The patient record board.
#[component]
pub fn PatientProfile(patient: Patient, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(
            div()
                .sx(board::row())
                .child(details(&patient))
                .child(clinical(&patient)),
        )
        .child(
            div()
                .sx(board::row())
                .child(current(&patient))
                .child(history(&patient))
                .child(notes(&patient)),
        )
        .child(board::stat_row(vec![
            board::stat(
                "Refill reminders",
                if patient.wants_reminders() {
                    "On"
                } else {
                    "Off"
                }
                .to_string(),
                patient.consent,
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Next refill due",
                patient
                    .next_due()
                    .map_or("None".to_string(), |medicine| medicine.name.to_string()),
                "the reason this record was opened",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Fills on record",
                patient.fill_count().to_string(),
                "kept for the period the regulator requires",
                None,
                mode,
            ),
        ]))
}

/// The record with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    PatientProfile::new(Patient::story())
}

#[cfg(test)]
mod tests {
    use super::{Patient, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_record_opens_on_the_patient_with_a_refill_due() {
        let patient = Patient::story();
        assert_eq!(patient.name, "Mzee Salim R.");
        assert_eq!(
            patient.next_due().map(|m| m.name),
            Some("Metformin 500mg tablets")
        );
        assert_eq!(
            patient.medicines[0].due, "Set by clinic",
            "the clinic sets the warfarin interval, so it is not a pharmacy refill"
        );
    }

    #[test]
    fn only_one_of_the_three_medicines_is_a_pharmacy_refill() {
        let patient = Patient::story();
        let due: Vec<&str> = patient
            .medicines
            .iter()
            .filter(|medicine| medicine.due.starts_with("Due"))
            .map(|medicine| medicine.name)
            .collect();
        assert_eq!(due, ["Metformin 500mg tablets", "Amlodipine 5mg tablets"]);
    }

    #[test]
    fn the_history_is_the_boards_six_fills() {
        let patient = Patient::story();
        assert_eq!(patient.fill_count(), 6);
        assert_eq!(patient.fills[0].reference, "RX-2214");
        assert_eq!(patient.fills[0].status, "Approved, at till");
    }

    #[test]
    fn the_notes_are_the_boards_two_and_carry_a_pharmacist() {
        let patient = Patient::story();
        assert_eq!(patient.notes.len(), 2);
        assert!(
            patient
                .notes
                .iter()
                .all(|note| note.meta.contains("Grace N."))
        );
    }

    #[test]
    fn the_patient_has_consented_to_reminders() {
        assert!(Patient::story().wants_reminders());
        assert!(Patient::story().consent.contains("14 Mar 2024"));
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
        }
    }

    #[gpui::test]
    fn draws_the_patient_record(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Screen);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
