//! The pharmacist's check of one prescription: what was read off the paper, the
//! six checks against the rule tables, the call to the prescriber when one of
//! them alerts, the label to print, and the approval that moves it to the till.
//!
//! Board: `Pharmacy_clinical_check_label.html`. The board's own prescription
//! lives in [`CheckPage::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// One medicine as read off the paper, with the batch that will be dispensed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Medicine {
    /// What it is called.
    pub name: &'static str,
    /// How many were prescribed.
    pub quantity: &'static str,
    /// The batch to draw from, which the check has already chosen.
    pub batch: &'static str,
    /// The directions as written on the prescription.
    pub directions: &'static str,
    /// The same directions in the words the patient is given.
    pub label_words: &'static str,
    /// How long the supply lasts.
    pub supply: &'static str,
    /// How long the batch has left.
    pub expiry: &'static str,
}

/// One thing the pharmacist has to confirm before dispensing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Check {
    /// What was checked.
    pub label: &'static str,
    /// What was found.
    pub detail: &'static str,
    /// What the chip says.
    pub result: &'static str,
    /// Whether the finding stops the prescription until it is resolved.
    pub alert: bool,
}

impl Check {
    /// How the board colours the finding.
    #[must_use]
    pub const fn tone(&self) -> Tone {
        if self.alert {
            Tone::Danger
        } else {
            Tone::Success
        }
    }
}

/// What the prescriber said on the phone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome {
    /// What the call came to.
    pub label: &'static str,
    /// Whether this is the outcome that was recorded.
    pub chosen: bool,
}

/// Everything the clinical check board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckPage {
    /// The prescription's reference.
    pub reference: &'static str,
    /// The patient it is for.
    pub patient: &'static str,
    /// Where the patient's record opens.
    pub patient_href: &'static str,
    /// The patient's age and sex, as asked at the counter.
    pub age: &'static str,
    /// Who prescribed it.
    pub prescriber: &'static str,
    /// Where they practice.
    pub clinic: &'static str,
    /// When the paper was issued.
    pub issued: &'static str,
    /// When the paper was scanned at the counter.
    pub scanned: &'static str,
    /// The medicines read off the paper.
    pub medicines: Vec<Medicine>,
    /// The checks the pharmacist works through.
    pub checks: Vec<Check>,
    /// The outcomes the call can have.
    pub outcomes: Vec<Outcome>,
    /// What the patient is told.
    pub counselling: Vec<&'static str>,
    /// The note that goes on the patient's record.
    pub note: &'static str,
    /// Who is checking.
    pub pharmacist: &'static str,
    /// Their initials, as the label prints them.
    pub initials: &'static str,
}

/// The medicines on RX-2214.
const MEDICINES: [Medicine; 2] = [
    Medicine {
        name: "Metronidazole 400mg tablets",
        quantity: "21",
        batch: "MTZ-2503",
        directions: "1 tablet 3 times a day for 7 days, avoid alcohol",
        label_words: "Take 1 tablet 3 times a day for 7 days. Avoid alcohol. Complete the course.",
        supply: "7 days",
        expiry: "06/2027",
    },
    Medicine {
        name: "Amoxicillin 500mg capsules",
        quantity: "15",
        batch: "AMX-2409",
        directions: "1 capsule 3 times a day for 5 days",
        label_words: "Take 1 capsule 3 times a day for 5 days. Finish the course.",
        supply: "5 days",
        expiry: "03/2027",
    },
];

/// The six checks, with the interaction alert in the third place.
const CHECKS: [Check; 6] = [
    Check {
        label: "Patient identity",
        detail: "Name and phone match the patient record. Age 68 confirmed with the patient.",
        result: "Confirmed",
        alert: false,
    },
    Check {
        label: "Allergies",
        detail: "None known. Asked the patient again today.",
        result: "None known",
        alert: false,
    },
    Check {
        label: "Interactions",
        detail: "Warfarin + metronidazole. Metronidazole may increase the effect of warfarin and the risk of bleeding. Contact the prescriber before dispensing, or agree extra INR monitoring.",
        result: "Alert",
        alert: true,
    },
    Check {
        label: "Duplicate therapy",
        detail: "No other antibiotic on file in the last 30 days.",
        result: "None found",
        alert: false,
    },
    Check {
        label: "Dose",
        detail: "Both doses are within the usual label range for adults.",
        result: "Within range",
        alert: false,
    },
    Check {
        label: "Insurance cover",
        detail: "National health insurance \u{b7} member active \u{b7} both items on the formulary.",
        result: "Covered",
        alert: false,
    },
];

/// The three things the call to the prescriber can have come to.
const OUTCOMES: [Outcome; 3] = [
    Outcome {
        label: "Prescriber changed the medicine",
        chosen: false,
    },
    Outcome {
        label: "Kept as written, extra INR check",
        chosen: true,
    },
    Outcome {
        label: "Not reached, hold",
        chosen: false,
    },
];

/// What Mzee Salim R. is told, in the pharmacist's own words.
const COUNSELLING: [&str; 5] = [
    "Take metronidazole with or after food. Do not drink alcohol during the course and for 48 hours after.",
    "Take amoxicillin at evenly spaced times and finish the full 5 days.",
    "Keep taking warfarin exactly as the clinic directs. Do not change the dose yourself.",
    "Book the INR check in 3\u{2013}5 days at the warfarin clinic.",
    "Contact the clinic quickly if you notice unusual bleeding, bruising, nosebleeds, blood in urine or dark stools.",
];

impl CheckPage {
    /// The board's own prescription.
    #[must_use]
    pub fn story() -> Self {
        Self {
            reference: "RX-2214",
            patient: "Mzee Salim R.",
            patient_href: "/patients/P-1042",
            age: "68 years \u{b7} male",
            prescriber: "[Prescriber name]",
            clinic: "[Dental clinic name]",
            issued: "02/10/2026",
            scanned: "14:48 at counter 2",
            medicines: MEDICINES.to_vec(),
            checks: CHECKS.to_vec(),
            outcomes: OUTCOMES.to_vec(),
            counselling: COUNSELLING.to_vec(),
            note: "Called [Dental clinic name] 15:10, spoke to [Prescriber name]. Prescriber agreed to keep metronidazole and amoxicillin as written. Patient to have an INR check in 3\u{2013}5 days at the warfarin clinic. Patient told.",
            pharmacist: "Grace N.",
            initials: "GN",
        }
    }

    /// The findings that stop the prescription until they are resolved.
    #[must_use]
    pub fn blocking(&self) -> Vec<&Check> {
        self.checks.iter().filter(|check| check.alert).collect()
    }

    /// The alert the board opens on, if there is one.
    #[must_use]
    pub fn alert(&self) -> Option<&Check> {
        self.checks.iter().find(|check| check.alert)
    }

    /// Whether the check is clear enough to approve for dispensing.
    ///
    /// An alert is only answered by a recorded call: an alert with no outcome
    /// behind it is a prescription still waiting on the phone.
    #[must_use]
    pub fn can_approve(&self) -> bool {
        match self.alert() {
            Some(_) => self.outcome().is_some_and(|outcome| outcome.chosen),
            None => true,
        }
    }

    /// The outcome the call came to.
    #[must_use]
    pub fn outcome(&self) -> Option<&Outcome> {
        self.outcomes.iter().find(|outcome| outcome.chosen)
    }

    /// The medicine the label preview is showing.
    #[must_use]
    pub fn preview(&self) -> Option<&Medicine> {
        self.medicines.first()
    }

    /// How many labels the board prints, one per medicine.
    #[must_use]
    pub fn labels(&self) -> usize {
        self.medicines.len()
    }
}

/// The scan of the paper, which the pharmacist reads the medicines off, and the
/// details that came with it.
fn image(page: &CheckPage) -> Div {
    board::column()
        .child(board::link(
            "check-back",
            "/prescriptions",
            "Back to prescription queue",
        ))
        .child(board::section_label("PRESCRIPTION IMAGE"))
        .child(board::panel(vec![
            board::card_title("PRESCRIPTION SCAN").into_any_element(),
            board::meta(format!("Paper from {}", page.clinic)).into_any_element(),
            board::meta(format!("Scanned {}", page.scanned)).into_any_element(),
        ]))
        .child(board::actions(vec![
            Button::new("check-zoom").label("Zoom").into_any_element(),
            Button::new("check-rotate")
                .label("Rotate")
                .into_any_element(),
        ]))
        .child(board::card(1.).child(board::panel(vec![
            board::link("check-patient", page.patient_href, page.patient).into_any_element(),
            board::key_value("Age", page.age).into_any_element(),
            board::key_value("Prescriber", page.prescriber).into_any_element(),
            board::key_value("Clinic", page.clinic).into_any_element(),
            board::key_value("Issued", page.issued).into_any_element(),
            board::key_value("Reference", page.reference).into_any_element(),
        ])))
}

/// The medicines read off the paper, with the batch each will draw from.
fn medicines(page: &CheckPage) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Medicines read from the prescription",
            format!("{} lines", page.medicines.len()),
        ))
        .child(board::head(vec![
            board::cell("Medicine"),
            board::cell_fixed("Qty"),
            board::cell_fixed("Batch"),
        ]))
        .children(
            page.medicines
                .iter()
                .map(|medicine| {
                    board::line(vec![
                        board::cell_stack(
                            medicine.name,
                            board::dotted(&[
                                medicine.directions,
                                medicine.supply,
                                &format!("exp {}", medicine.expiry),
                            ]),
                        ),
                        board::cell_fixed(medicine.quantity),
                        board::cell_fixed(medicine.batch),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::footnote(
            "The batch is chosen now so the label and the shelf agree.",
        ))
}

/// The checks, with the alert toned the way the board tints its one red row.
fn checks(page: &CheckPage, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Pharmacist checks",
            format!(
                "{} of {} clear",
                page.checks.len() - page.blocking().len(),
                page.checks.len()
            ),
        ))
        .children(
            page.checks
                .iter()
                .map(|check| {
                    let row = vec![
                        board::cell_fixed(check.label),
                        board::cell_truncating(check.detail),
                        board::chip(check.result, check.tone()).into_any_element(),
                    ];
                    if check.alert {
                        board::toned_row(check.tone(), mode, row)
                    } else {
                        board::line(row)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .child(board::footnote(
            "An alert is not a stop on its own: it is a stop until the call is recorded.",
        ))
}

/// What the prescriber said, and the note that goes on the patient's record.
fn call(page: &CheckPage, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Prescriber call outcome",
            format!("Called {} at 15:10", page.clinic),
        ))
        .children(
            page.outcomes
                .iter()
                .map(|outcome| {
                    let row = vec![
                        board::chip(
                            if outcome.chosen {
                                "Recorded"
                            } else {
                                "Not chosen"
                            },
                            if outcome.chosen {
                                Tone::Brand
                            } else {
                                Tone::Neutral
                            },
                        )
                        .into_any_element(),
                        board::cell(outcome.label),
                    ];
                    if outcome.chosen {
                        board::toned_row(Tone::Brand, mode, row)
                    } else {
                        board::line(row)
                    }
                })
                .collect::<Vec<_>>(),
        )
        .child(board::section_label("NOTE FOR THE PATIENT RECORD"))
        .child(board::footnote(page.note))
}

/// The counselling points: what the patient is told, which the label alone does
/// not carry.
fn counselling(page: &CheckPage) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Counselling points",
            format!("{} points", page.counselling.len()),
        ))
        .children(
            page.counselling
                .iter()
                .enumerate()
                .map(|(index, point)| {
                    board::line(vec![
                        board::chip((index + 1).to_string(), Tone::Neutral).into_any_element(),
                        board::cell(*point),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The label as the printer will set it out, one medicine at a time.
fn label_preview(page: &CheckPage) -> Div {
    let heading = format!("DOSAGE LABEL PREVIEW \u{b7} 1 OF {}", page.labels());
    let body = match page.preview() {
        Some(medicine) => vec![
            board::key_value("Patient", page.patient).into_any_element(),
            board::card_title(format!("{} \u{d7} {}", medicine.name, medicine.quantity))
                .into_any_element(),
            board::meta(medicine.label_words).into_any_element(),
            board::key_value(
                "Date",
                format!("{} \u{b7} Pharmacist {}", page.issued, page.initials),
            )
            .into_any_element(),
        ],
        None => vec![board::meta("No medicine on this prescription").into_any_element()],
    };
    board::card(1.)
        .child(board::section_label(heading))
        .child(board::panel(body))
        .child(board::footnote(format!(
            "One label per medicine, first expiry first out of {}.",
            page.preview().map_or("\u{2014}", |medicine| medicine.batch),
        )))
}

/// The approval: the pharmacist's PIN, and what happens after it.
fn approval(page: &CheckPage) -> Div {
    let blocked = !page.can_approve();
    board::card(1.)
        .child(board::card_head("Pharmacist approval", page.pharmacist))
        .child(board::option(format!("{} \u{b7} PIN", page.pharmacist)))
        .child(board::actions(vec![
            Button::new("check-hold").label("Hold").into_any_element(),
            Button::new("check-record")
                .label("Record outcome")
                .into_any_element(),
        ]))
        .child(
            Button::new("check-approve")
                .label(if blocked {
                    "Approval needs a recorded call"
                } else {
                    "Approve for dispensing"
                })
                .into_any_element(),
        )
        .child(board::footnote(
            "After approval the prescription moves to the till.",
        ))
        .child(board::link("check-till", "/till", "Open dispensary till"))
}

/// The clinical check board with the board's prescription.
#[component]
pub fn ClinicalCheck(page: CheckPage, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Medicines",
                page.medicines.len().to_string(),
                "read off the paper",
                None,
                mode,
            ),
            board::stat(
                "Checks clear",
                format!(
                    "{}/{}",
                    page.checks.len() - page.blocking().len(),
                    page.checks.len()
                ),
                "against the rule tables",
                Some(Tone::Success),
                mode,
            ),
            board::stat(
                "Alerts",
                page.blocking().len().to_string(),
                "each one needs a recorded call",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Call recorded",
                if page.outcome().is_some() {
                    "Yes"
                } else {
                    "No"
                }
                .to_string(),
                "the prescriber's answer, on the record",
                if page.can_approve() {
                    Some(Tone::Brand)
                } else {
                    Some(Tone::Warning)
                },
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(image(&page))
                .child(
                    board::column()
                        .child(medicines(&page))
                        .child(checks(&page, mode))
                        .child(call(&page, mode)),
                )
                .child(
                    board::column()
                        .child(counselling(&page))
                        .child(label_preview(&page))
                        .child(approval(&page)),
                ),
        )
}

/// The clinical check for `prescription_id`, a code like `RX-2214`. Phase 3
/// reads the prescription from the database; until then every id opens the
/// board's own prescription.
#[must_use]
pub fn view(_prescription_id: &str) -> impl IntoElement {
    ClinicalCheck::new(CheckPage::story())
}

#[cfg(test)]
mod tests {
    use super::{CheckPage, Outcome, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_interaction_alert_is_what_the_board_leads_with() {
        let page = CheckPage::story();
        let alert = page.alert().expect("the board has one alert");
        assert_eq!(alert.label, "Interactions");
        assert!(alert.detail.contains("warfarin"));
        assert_eq!(page.blocking().len(), 1);
    }

    #[test]
    fn an_alert_is_a_stop_until_the_call_is_recorded() {
        let mut page = CheckPage::story();
        assert!(page.can_approve(), "the call is already recorded");
        page.outcomes = page
            .outcomes
            .iter()
            .map(|outcome| Outcome {
                chosen: false,
                ..*outcome
            })
            .collect();
        assert!(
            !page.can_approve(),
            "an alert with no outcome behind it is waiting on the phone"
        );
    }

    #[test]
    fn the_recorded_outcome_keeps_the_prescription_as_written() {
        let page = CheckPage::story();
        let outcome = page.outcome().expect("the board records a call");
        assert_eq!(outcome.label, "Kept as written, extra INR check");
        assert!(
            page.note.contains("INR check"),
            "the agreed extra monitoring goes on the patient's record"
        );
    }

    #[test]
    fn the_label_starts_from_the_first_medicine_and_names_its_batch() {
        let page = CheckPage::story();
        assert_eq!(page.labels(), 2);
        let preview = page.preview().expect("two medicines");
        assert_eq!(preview.name, "Metronidazole 400mg tablets");
        assert_eq!(preview.batch, "MTZ-2503");
        assert!(
            preview.label_words.contains("Avoid alcohol"),
            "the label repeats what the counselling says"
        );
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view("RX-2214")
        }
    }

    #[gpui::test]
    fn draws_the_story(cx: &mut gpui::TestAppContext) {
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
