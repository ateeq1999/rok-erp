//! Batch recalls: what a supplier or the authority has recalled, which batches
//! of it the pharmacy holds, who bought those batches, and what to send back.
//!
//! Board: `design/pharmacy/Pharmacy_batch_recall.html`. The board's own recall
//! lives in [`Recall::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// How far along a recall is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Nothing has been done yet.
    Open,
    /// The batches are off the shelf and the queue is blocked.
    Quarantined,
    /// Patients are being reached.
    Contacting,
    /// Everything is back and the credit is claimed.
    Closed,
}

impl Stage {
    /// What the board's chip says.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Stage::Open => "Open",
            Stage::Quarantined => "Quarantined",
            Stage::Contacting => "Contacting",
            Stage::Closed => "Closed",
        }
    }

    /// How urgent the chip looks.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Stage::Open => Tone::Warning,
            Stage::Quarantined => Tone::Danger,
            Stage::Contacting => Tone::Brand,
            Stage::Closed => Tone::Success,
        }
    }
}

/// One thing rok did on its own when the notice arrived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Automatic {
    /// When it happened.
    pub time: &'static str,
    /// What happened.
    pub title: &'static str,
    /// The detail line under it.
    pub detail: &'static str,
}

/// One patient who bought a recalled batch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bought {
    /// Their name.
    pub patient: &'static str,
    /// How to reach them.
    pub phone: &'static str,
    /// When they bought it.
    pub sold: &'static str,
    /// Which prescription it was against.
    pub prescription: &'static str,
    /// How many they took.
    pub bottles: u32,
    /// Whether they have been reached, and the button that says so.
    pub status: &'static str,
    /// The button on the row.
    pub call_label: &'static str,
}

impl Bought {
    /// Whether this patient still has to hear about it.
    #[must_use]
    pub fn outstanding(&self) -> bool {
        !self.status.starts_with("Called") && !self.status.starts_with("Returned")
    }
}

/// One step of the checklist a pharmacist works through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step {
    /// The step itself.
    pub label: &'static str,
    /// How it is done or why it is done.
    pub detail: &'static str,
    /// Whether it is done.
    pub checked: bool,
}

/// A recall the pharmacy has finished with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PastRecall {
    /// Its reference.
    pub code: &'static str,
    /// What was recalled.
    pub product: &'static str,
    /// When it was closed.
    pub closed: &'static str,
}

/// One batch the pharmacy holds that the notice covers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Held {
    /// The batch code.
    pub batch: &'static str,
    /// Where it is.
    pub shelf: &'static str,
    /// How many bottles.
    pub bottles: u32,
}

/// Everything the recall board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recall {
    /// Its reference, as the board names it in the sidebar badge.
    pub code: &'static str,
    /// The product the notice covers.
    pub product: &'static str,
    /// Which batch of it is recalled.
    pub batch: &'static str,
    /// Who issued the notice and when.
    pub from: &'static str,
    /// How far along it is.
    pub stage: Stage,
    /// The batches the pharmacy holds.
    pub held: Vec<Held>,
    /// What rok did by itself.
    pub automatic: Vec<Automatic>,
    /// Everyone who bought the batch.
    pub bought: Vec<Bought>,
    /// The checklist.
    pub checklist: Vec<Step>,
    /// What the pharmacy expects credit for.
    pub returns: &'static str,
    /// The recalls before this one.
    pub past: Vec<PastRecall>,
}

/// The batches the pharmacy holds that the notice covers.
const HELD: [Held; 2] = [
    Held {
        batch: "AMS-2404",
        shelf: "Mwenge shelf A3 \u{b7} quarantine box",
        bottles: 14,
    },
    Held {
        batch: "AMS-2311",
        shelf: "Tegeta shelf A2 \u{b7} quarantine box",
        bottles: 6,
    },
];

/// What rok did the moment the notice arrived.
const AUTOMATIC: [Automatic; 4] = [
    Automatic {
        time: "02 Oct 09:31",
        title: "Batch blocked at the till",
        detail: "AMS-2404 can no longer be sold from either branch",
    },
    Automatic {
        time: "02 Oct 09:31",
        title: "20 bottles quarantined",
        detail: "moved off the shelf into the quarantine box",
    },
    Automatic {
        time: "02 Oct 09:32",
        title: "6 prescriptions flagged",
        detail: "each patient is listed below with the batch they were given",
    },
    Automatic {
        time: "02 Oct 09:32",
        title: "Supplier and authority notified",
        detail: "return booked and the regulator's return form started",
    },
];

/// Everyone who bought the batch, oldest first.
const BOUGHT: [Bought; 5] = [
    Bought {
        patient: "Rehema Juma",
        phone: "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 611",
        sold: "18 Sep 2026",
        prescription: "RX-1912",
        bottles: 2,
        status: "Called 02 Oct 10:15",
        call_label: "Call again",
    },
    Bought {
        patient: "Asha P.",
        phone: "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 204",
        sold: "21 Sep 2026",
        prescription: "RX-1955",
        bottles: 1,
        status: "Returned 1 bottle",
        call_label: "Collect the rest",
    },
    Bought {
        patient: "Khamis B.",
        phone: "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 738",
        sold: "28 Sep 2026",
        prescription: "RX-2033",
        bottles: 1,
        status: "Not reached",
        call_label: "Call",
    },
    Bought {
        patient: "Mariam S.",
        phone: "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 977",
        sold: "30 Sep 2026",
        prescription: "RX-2044",
        bottles: 1,
        status: "Not reached",
        call_label: "Call",
    },
    Bought {
        patient: "Upendo L.",
        phone: "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 480",
        sold: "01 Oct 2026",
        prescription: "RX-2061",
        bottles: 1,
        status: "Text sent 02 Oct",
        call_label: "Call",
    },
];

/// The steps a pharmacist works through, in the order they block the rest.
const CHECKLIST: [Step; 5] = [
    Step {
        label: "Pull every batch from both shelves",
        detail: "AMS-2404 and AMS-2311, 20 bottles in all",
        checked: true,
    },
    Step {
        label: "Block the batch at the till",
        detail: "done automatically when the notice arrived",
        checked: true,
    },
    Step {
        label: "Find everyone who bought it",
        detail: "from the dispensing records, not from memory",
        checked: true,
    },
    Step {
        label: "Call every patient on the list",
        detail: "3 of 5 reached",
        checked: false,
    },
    Step {
        label: "Book the return and claim the credit",
        detail: "with Uzima Pharmaceuticals, form started",
        checked: false,
    },
];

/// The recalls before this one.
const PAST: [PastRecall; 2] = [
    PastRecall {
        code: "RC-0031",
        product: "Paracetamol 500mg tablets",
        closed: "18 Jun 2026",
    },
    PastRecall {
        code: "RC-0012",
        product: "ORS sachet for 1 litre",
        closed: "04 Feb 2026",
    },
];

impl Recall {
    /// The board's own recall, RC-0047.
    #[must_use]
    pub fn story() -> Self {
        Self {
            code: "RC-0047",
            product: "Amoxicillin 250mg/5ml powder for oral suspension, 100 ml",
            batch: "AMS-2404",
            from: "Uzima Pharmaceuticals, 02 Oct 2026",
            stage: Stage::Contacting,
            held: HELD.to_vec(),
            automatic: AUTOMATIC.to_vec(),
            bought: BOUGHT.to_vec(),
            checklist: CHECKLIST.to_vec(),
            returns: "20 bottles, 11 returned, 9 outstanding \u{b7} credit TZS 61,250",
            past: PAST.to_vec(),
        }
    }

    /// How many bottles the pharmacy holds of the recalled batch.
    #[must_use]
    pub fn held_bottles(&self) -> u32 {
        self.held.iter().map(|held| held.bottles).sum()
    }

    /// How many bottles went out to patients.
    #[must_use]
    pub fn sold_bottles(&self) -> u32 {
        self.bought.iter().map(|patient| patient.bottles).sum()
    }

    /// Everyone who has still not heard about it.
    #[must_use]
    pub fn outstanding(&self) -> Vec<&Bought> {
        self.bought
            .iter()
            .filter(|patient| patient.outstanding())
            .collect()
    }

    /// What the checklist has left to do.
    #[must_use]
    pub fn unchecked(&self) -> Vec<&Step> {
        self.checklist.iter().filter(|step| !step.checked).collect()
    }
}

/// The batch the notice covers, as the board titles the page with it.
fn headline(recall: &Recall) -> Div {
    div()
        .sx(board::root())
        .child(board::card_head(
            recall.product,
            board::dotted(&[recall.code, recall.from]),
        ))
        .child(board::chip_line(vec![
            board::chip(recall.stage.label(), recall.stage.tone()).into_any_element(),
            board::chip(format!("Batch {}", recall.batch), Tone::Danger).into_any_element(),
        ]))
}

/// What rok did without being asked, which the board lists as a timeline.
fn automatic(recall: &Recall) -> Div {
    board::card(1.)
        .child(board::card_head(
            "What rok did automatically",
            "No one had to ask",
        ))
        .children(
            recall
                .automatic
                .iter()
                .map(|step| {
                    board::line(vec![
                        board::cell_fixed(step.time),
                        board::cell_stack(step.title, step.detail),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The batches the pharmacy holds, which have to be off the shelf already.
fn held(recall: &Recall, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Batches held",
            format!("{} bottles across both branches", recall.held_bottles()),
        ))
        .child(board::head(vec![
            board::cell("Batch"),
            board::cell("Where"),
            board::cell_fixed("Quantity"),
        ]))
        .children(
            recall
                .held
                .iter()
                .map(|batch| {
                    board::toned_row(
                        Tone::Danger,
                        mode,
                        vec![
                            board::cell_fixed(batch.batch),
                            board::cell(batch.shelf),
                            board::cell_fixed(format!("{} bottles", batch.bottles)),
                        ],
                    )
                })
                .collect::<Vec<_>>(),
        )
}

/// Everyone who bought the batch, and how many are still to reach.
fn patients(recall: &Recall) -> Div {
    let sold = recall.sold_bottles();
    let outstanding = recall.outstanding().len();
    board::card(1.4)
        .child(board::card_head(
            board::dotted(&[
                "Patients to contact",
                &format!("{sold} bottles sold to {} patients", recall.bought.len()),
            ]),
            format!("{outstanding} of {} still to reach", recall.bought.len()),
        ))
        .child(board::head(vec![
            board::cell("Patient"),
            board::cell("Sold"),
            board::cell("Prescription"),
            board::cell_fixed("Bottles"),
            board::cell("Contact"),
            board::cell_fixed(""),
        ]))
        .children(
            recall
                .bought
                .iter()
                .enumerate()
                .map(|(index, patient)| {
                    let tone = if patient.outstanding() {
                        Tone::Warning
                    } else {
                        Tone::Success
                    };
                    board::line(vec![
                        board::cell_stack(patient.patient, patient.phone),
                        board::cell(patient.sold),
                        board::cell(patient.prescription),
                        board::cell_fixed(patient.bottles.to_string()),
                        board::chip(patient.status, tone).into_any_element(),
                        board::link(("recall-patient", index), "/recalls", patient.call_label)
                            .into_any_element(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The checklist and what the pharmacy gets back.
fn checklist(recall: &Recall, mode: ThemeMode) -> Div {
    let left = recall.unchecked().len();
    board::card(0.8)
        .child(board::card_head("Recall checklist", format!("{left} steps left")))
        .children(
            recall
                .checklist
                .iter()
                .map(|step| {
                    let mark = if step.checked { "Done" } else { "To do" };
                    board::line(vec![
                        board::chip(mark, if step.checked { Tone::Success } else { Tone::Warning })
                            .into_any_element(),
                        board::cell_stack(step.label, step.detail),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::key_value("Return to supplier", recall.returns))
        .child(board::actions(vec![
            Button::new("recall-export")
                .label("Export return form")
                .into_any_element(),
            Button::new("recall-close")
                .label("Close recall")
                .into_any_element(),
        ]))
        .child(board::alert(
            "A closed recall keeps its list",
            "The regulator asks who was told and when, so closing the recall does not clear the record.",
            Tone::Info,
            mode,
        ))
}

/// The recalls before this one, which is how a pharmacist recognises a pattern.
fn past(recall: &Recall) -> Div {
    board::card(0.7)
        .child(board::card_head("Past recalls", "Closed at this pharmacy"))
        .children(
            recall
                .past
                .iter()
                .map(|item| {
                    board::line(vec![
                        board::cell_fixed(item.code),
                        board::cell_stack(item.product, item.closed),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The recall board for `recall`.
#[component]
pub fn Recalls(recall: Recall, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let outstanding = recall.outstanding().len();
    div()
        .sx((board::root(), &sx))
        .child(headline(&recall))
        .child(board::stat_row(vec![
            board::stat(
                "Bottles held",
                recall.held_bottles().to_string(),
                "quarantined from both branches",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Bottles sold",
                recall.sold_bottles().to_string(),
                format!("to {} patients", recall.bought.len()),
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Patients to reach",
                outstanding.to_string(),
                "of everyone who bought the batch",
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Steps left",
                recall.unchecked().len().to_string(),
                "the checklist has to finish first",
                None,
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(held(&recall, mode))
                .child(automatic(&recall)),
        )
        .child(patients(&recall))
        .child(
            div()
                .sx(board::row())
                .child(checklist(&recall, mode))
                .child(past(&recall)),
        )
        .child(board::footnote(
            "A recall is not closed until every patient has been reached and every bottle is back or written off.",
        ))
}

/// The recall board with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    Recalls::new(Recall::story())
}

// ---------------------------------------------------------------------------
// The list `/recalls` opens, so a recall is found before it is worked on.

/// One row of the recall list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Summary {
    /// Its reference.
    pub code: &'static str,
    /// What is recalled.
    pub product: &'static str,
    /// Which batch.
    pub batch: &'static str,
    /// Who issued the notice.
    pub from: &'static str,
    /// How far along it is.
    pub stage: Stage,
    /// What is left to do, as the board counts it.
    pub outstanding: u32,
    /// Its path.
    pub href: &'static str,
}

/// Every recall the board lists, the open ones first.
const SUMMARIES: [Summary; 4] = [
    Summary {
        code: "RC-0047",
        product: "Amoxicillin 250mg/5ml syrup",
        batch: "AMS-2404",
        from: "Uzima Pharmaceuticals",
        stage: Stage::Contacting,
        outstanding: 3,
        href: "/recalls/RC-0047",
    },
    Summary {
        code: "RC-0046",
        product: "Human insulin 100 IU/ml vial",
        batch: "INS-2506",
        from: "Uzima Pharmaceuticals",
        stage: Stage::Quarantined,
        outstanding: 2,
        href: "/recalls/RC-0046",
    },
    Summary {
        code: "RC-0044",
        product: "Cetirizine 10mg tablets",
        batch: "CTZ-2410",
        from: "Medline Africa",
        stage: Stage::Open,
        outstanding: 1,
        href: "/recalls/RC-0044",
    },
    Summary {
        code: "RC-0031",
        product: "Paracetamol 500mg tablets",
        batch: "PCM-2405",
        from: "Tanzania Medics",
        stage: Stage::Closed,
        outstanding: 0,
        href: "/recalls/RC-0031",
    },
];

/// The recall list: what is open, what is being worked on, what is done.
#[component]
pub fn List(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let open = SUMMARIES
        .iter()
        .filter(|summary| summary.stage != Stage::Closed)
        .count();
    let to_reach: u32 = SUMMARIES
        .iter()
        .filter(|summary| summary.stage == Stage::Contacting)
        .map(|summary| summary.outstanding)
        .sum();
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Recalls open",
                open.to_string(),
                "one per supplier notice",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "Patients to reach",
                to_reach.to_string(),
                "across the recalls being worked on",
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Batches quarantined",
                "20".to_string(),
                "off the shelf, blocked at the till",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Closed this year",
                "2".to_string(),
                "kept for the regulator",
                None,
                mode,
            ),
        ]))
        .child(
            board::card(1.)
                .child(board::card_head("Recalls", "Open first"))
                .child(board::head(vec![
                    board::cell("Recall"),
                    board::cell("Batch"),
                    board::cell("Notified by"),
                    board::cell("Stage"),
                    board::cell_fixed("To reach"),
                    board::cell_fixed(""),
                ]))
                .children(
                    SUMMARIES
                        .iter()
                        .enumerate()
                        .map(|(index, summary)| {
                            board::line(vec![
                                board::cell_stack(summary.code, summary.product),
                                board::cell_fixed(summary.batch),
                                board::cell(summary.from),
                                board::chip(summary.stage.label(), summary.stage.tone())
                                    .into_any_element(),
                                board::cell_fixed(
                                    if summary.outstanding > 0 {
                                        summary.outstanding.to_string()
                                    } else {
                                        "\u{2014}".to_string()
                                    },
                                ),
                                board::link(
                                    ("recall-row", index),
                                    summary.href,
                                    "Open recall",
                                )
                                .into_any_element(),
                            ])
                        })
                        .collect::<Vec<_>>(),
                ),
        )
        .child(board::footnote(
            "A recall blocks its batch from the moment the notice arrives; everything after that is paperwork and telephone calls.",
        ))
}

/// The recall list with the board's figures.
#[must_use]
pub fn list() -> impl IntoElement {
    List::new()
}

/// One recall, `recall_id` a code like `RC-0047`. Phase 3 reads the recall from
/// the database; until then every id opens the board's recall.
#[must_use]
pub fn recall_view(_recall_id: &str) -> impl IntoElement {
    Recalls::new(Recall::story())
}

#[cfg(test)]
mod tests {
    use super::{Recall, Stage, view};
    use rok_ui::prelude::*;

    #[test]
    fn a_recall_blocks_the_batch_before_it_asks_for_anything() {
        let recall = Recall::story();
        assert_eq!(recall.stage, Stage::Contacting);
        assert!(
            recall
                .automatic
                .iter()
                .any(|step| step.title.contains("blocked at the till")),
            "the till is blocked by the notice arriving, not by a person acting"
        );
        assert_eq!(recall.held_bottles(), 20);
    }

    #[test]
    fn the_patient_list_agrees_with_the_bottle_count() {
        let recall = Recall::story();
        assert_eq!(recall.bought.len(), 5);
        assert_eq!(recall.sold_bottles(), 6);
        assert_eq!(
            recall.outstanding().len(),
            3,
            "two were reached, three still have to hear about it"
        );
    }

    #[test]
    fn the_recall_cannot_close_until_the_checklist_is_done() {
        let recall = Recall::story();
        let left = recall.unchecked();
        assert_eq!(left.len(), 2);
        assert!(
            left.iter()
                .any(|step| step.label.contains("Call every patient")),
            "the calls are the step that has to finish"
        );
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
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
