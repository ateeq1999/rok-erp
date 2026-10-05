//! Insurance claims: the split between insurer and patient, the batches sent,
//! and the queries to answer before the next submission.
//!
//! Board: `Pharmacy_insurance_claims.html`. The board's own batches live in
//! [`Claims::story`].

use rok_pos_shell::{Tone, group_digits};
use rok_ui::prelude::*;

use crate::screens::board;

/// How a claim batch stands with its insurer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchStatus {
    /// Sent, and the insurer has not answered.
    Submitted,
    /// The insurer has queried claims inside it.
    Queried,
    /// Paid.
    Paid,
    /// The insurer rejected the whole batch.
    Rejected,
}

impl BatchStatus {
    /// What the board's chip says.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            BatchStatus::Submitted => "Submitted",
            BatchStatus::Queried => "Queried",
            BatchStatus::Paid => "Paid",
            BatchStatus::Rejected => "Rejected",
        }
    }

    /// How urgent the chip looks.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            BatchStatus::Submitted => Tone::Info,
            BatchStatus::Queried => Tone::Warning,
            BatchStatus::Paid => Tone::Success,
            BatchStatus::Rejected => Tone::Danger,
        }
    }
}

/// One claim batch sent to one insurer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Batch {
    /// Who it went to.
    pub insurer: &'static str,
    /// Which month it covers.
    pub month: &'static str,
    /// How many claims are in it.
    pub claims: u32,
    /// What it is worth.
    pub value: &'static str,
    /// How many of its claims the insurer queried.
    pub queried: u32,
    /// How it stands.
    pub status: BatchStatus,
    /// What the board says about it.
    pub note: &'static str,
}

/// One claim the insurer has queried.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Query {
    /// Which prescription it was against.
    pub prescription: &'static str,
    /// Who the patient is.
    pub patient: &'static str,
    /// What the insurer is owed.
    pub amount: &'static str,
    /// Why they queried it.
    pub reason: &'static str,
    /// What fixes it.
    pub fix: &'static str,
    /// Whether it has been answered.
    pub answered: bool,
}

/// Everything the claims board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claims {
    /// The batches the board lists.
    pub batches: Vec<Batch>,
    /// The queries inside the batch that needs attention.
    pub queries: Vec<Query>,
    /// What the insurer owes across every open batch.
    pub owed: &'static str,
    /// What the patients paid that the insurer has not covered.
    pub patients_paid: &'static str,
}

/// The batches the board lists, the one needing work first.
const BATCHES: [Batch; 4] = [
    Batch {
        insurer: "National health insurance",
        month: "September",
        claims: 214,
        value: "8,412,600",
        queried: 11,
        status: BatchStatus::Queried,
        note: "answer the queries by 08 Oct or the batch is late",
    },
    Batch {
        insurer: "National health insurance",
        month: "October",
        claims: 96,
        value: "3,884,200",
        queried: 0,
        status: BatchStatus::Submitted,
        note: "closes on 31 Oct",
    },
    Batch {
        insurer: "[Private insurer A]",
        month: "August",
        claims: 41,
        value: "1,502,400",
        queried: 2,
        status: BatchStatus::Paid,
        note: "paid 26 Sep",
    },
    Batch {
        insurer: "[Private insurer B]",
        month: "September",
        claims: 18,
        value: "640,900",
        queried: 18,
        status: BatchStatus::Rejected,
        note: "whole batch rejected, resubmit with the prescriber's codes",
    },
];

/// The queries inside the September batch.
const QUERIES: [Query; 5] = [
    Query {
        prescription: "RX-2214",
        patient: "Mzee Salim R.",
        amount: "48,200",
        reason: "Prescriber code missing on the claim",
        fix: "Add the dental clinic's code",
        answered: false,
    },
    Query {
        prescription: "RX-2208",
        patient: "Rehema Juma",
        amount: "12,500",
        reason: "Medicine not on the approved list for the diagnosis",
        fix: "Attach the specialist's letter",
        answered: false,
    },
    Query {
        prescription: "RX-2197",
        patient: "Khamis B.",
        amount: "31,800",
        reason: "Two claims for the same day",
        fix: "Cancel the duplicate claim",
        answered: false,
    },
    Query {
        prescription: "RX-2188",
        patient: "Mariam S.",
        amount: "9,400",
        reason: "Patient's membership had lapsed that day",
        fix: "Claim from the patient instead",
        answered: true,
    },
    Query {
        prescription: "RX-2176",
        patient: "Upendo L.",
        amount: "22,000",
        reason: "Batch number not recorded on the claim",
        fix: "Scan the dispensing label",
        answered: true,
    },
];

impl Claims {
    /// The board's own claim batches.
    #[must_use]
    pub fn story() -> Self {
        Self {
            batches: BATCHES.to_vec(),
            queries: QUERIES.to_vec(),
            owed: "13,940,100",
            patients_paid: "486,300",
        }
    }

    /// How many queries are still to answer, which is what the sidebar counts.
    #[must_use]
    pub fn unanswered(&self) -> usize {
        self.queries.iter().filter(|query| !query.answered).count()
    }

    /// What the unanswered queries are worth, as the board totals them.
    #[must_use]
    pub fn queried_value(&self) -> String {
        let shillings: i64 = self
            .queries
            .iter()
            .filter(|query| !query.answered)
            .map(|query| {
                query
                    .amount
                    .replace(',', "")
                    .parse::<i64>()
                    .unwrap_or_default()
            })
            .sum();
        group_digits(shillings)
    }

    /// The batch the board opens on: the one with queries still to answer.
    #[must_use]
    pub fn urgent(&self) -> Option<&Batch> {
        self.batches
            .iter()
            .find(|batch| batch.status == BatchStatus::Queried)
    }
}

/// One batch row, as the board's list shows them.
fn batch_row(index: usize, batch: &Batch) -> Div {
    board::line(vec![
        board::cell_stack(batch.insurer, batch.month),
        board::cell_fixed(batch.claims.to_string()),
        board::cell_fixed(batch.value),
        queried_cell(batch),
        board::chip(batch.status.label(), batch.status.tone()).into_any_element(),
        board::link(("claims-batch", index), "/claims", "Open batch").into_any_element(),
    ])
}

/// The queried count as its own cell, so an unqueried batch draws nothing.
fn queried_cell(batch: &Batch) -> AnyElement {
    match batch.queried {
        0 => board::cell_fixed("\u{2014}"),
        queried => board::cell_fixed(format!("{queried} queried")),
    }
}

/// The board's batches, one card each, because each carries its own note.
fn batches(claims: &Claims) -> Div {
    board::card(1.)
        .child(board::card_head("Claim batches", "Open first"))
        .child(board::head(vec![
            board::cell("Insurer"),
            board::cell_fixed("Claims"),
            board::cell_fixed("Value"),
            board::cell_fixed("Queries"),
            board::cell("Status"),
            board::cell_fixed(""),
        ]))
        .children(
            claims
                .batches
                .iter()
                .enumerate()
                .map(|(index, batch)| batch_row(index, batch)),
        )
        .child(board::footnote(
            "A queried batch is still the pharmacy's money until the insurer pays it, so it stays on this list.",
        ))
}

/// The queries inside the batch that needs work, which is the whole job.
fn queries(claims: &Claims, mode: ThemeMode) -> Div {
    let unanswered = claims.unanswered();
    board::card(1.3)
        .child(board::card_head(
            "Queries to answer",
            format!(
                "{unanswered} of {} still to answer \u{b7} TZS {}",
                claims.queries.len(),
                claims.queried_value()
            ),
        ))
        .child(board::head(vec![
            board::cell("Prescription"),
            board::cell("Patient"),
            board::cell_fixed("Amount"),
            board::cell("Query reason"),
            board::cell("Fix"),
            board::cell_fixed(""),
        ]))
        .children(
            claims
                .queries
                .iter()
                .enumerate()
                .map(|(index, query)| {
                    let tone = if query.answered {
                        Tone::Success
                    } else {
                        Tone::Warning
                    };
                    board::line(vec![
                        board::cell_fixed(query.prescription),
                        board::cell(query.patient),
                        board::cell_number(query.amount),
                        board::cell(query.reason),
                        board::cell(query.fix),
                        board::chip(
                            if query.answered { "Answered" } else { "To answer" },
                            tone,
                        )
                        .into_any_element(),
                        board::link(("claim-query", index), "/claims", "Answer").into_any_element(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::alert(
            "An unanswered query costs the claim, not the patient",
            "A claim the insurer rejects is written back to the patient, so it is worth answering before it is resubmitted.",
            Tone::Info,
            mode,
        ))
}

/// The claims board.
#[component]
pub fn InsuranceClaims(claims: Claims, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let urgent = claims.urgent();
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Owed by insurers",
                claims.owed.to_string(),
                "across every open batch",
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Queries to answer",
                claims.unanswered().to_string(),
                "TZS 92,500 at risk",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Claim value this month",
                "3,884,200".to_string(),
                "October batch closes 31 Oct",
                None,
                mode,
            ),
            board::stat(
                "Paid by patients",
                claims.patients_paid.to_string(),
                "the part no insurer covers",
                None,
                mode,
            ),
        ]))
        .child(urgent.map_or_else(
            || board::card(1.).child(board::card_head("Claim batches", "All submitted")),
            |batch| {
                board::alert(
                    board::dotted(&[batch.insurer, batch.month, "batch queried"]),
                    board::dotted(&[
                        &format!("{} claims \u{b7} TZS {}", batch.claims, batch.value),
                        &format!("{} queried", batch.queried),
                        batch.note,
                    ]),
                    Tone::Warning,
                    mode,
                )
            },
        ))
        .child(batches(&claims))
        .child(queries(&claims, mode))
        .child(board::footnote(
            "A claim is only the pharmacy's money once the insurer has paid it, so an open batch is not a sale.",
        ))
}

/// The claims board with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    InsuranceClaims::new(Claims::story())
}

#[cfg(test)]
mod tests {
    use super::{BatchStatus, Claims, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_sidebar_count_is_the_number_still_to_answer() {
        let claims = Claims::story();
        assert_eq!(claims.unanswered(), 3);
        assert_eq!(claims.queried_value(), "92,500");
    }

    #[test]
    fn the_board_opens_on_the_batch_with_queries() {
        let claims = Claims::story();
        let urgent = claims.urgent().expect("a queried batch");
        assert_eq!(urgent.status, BatchStatus::Queried);
        assert_eq!(urgent.month, "September");
        assert!(
            claims
                .batches
                .iter()
                .any(|batch| batch.status == BatchStatus::Rejected),
            "a rejected batch stays on the list until it is resubmitted"
        );
    }

    #[test]
    fn every_query_names_the_fix_and_not_only_the_complaint() {
        let claims = Claims::story();
        for query in &claims.queries {
            assert_ne!(query.reason, "");
            assert!(
                !query.fix.trim().is_empty(),
                "{} is queried but nothing says how to clear it",
                query.prescription
            );
        }
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
