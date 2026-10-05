//! Batches and expiry: what is on the shelf, what may not be sold, what is
//! about to run out, and which cold-chain unit is holding.
//!
//! Board: `Pharmacy_batches_expiry_FEFO_cold_chain.html`. The board's own
//! batches live in [`Batches::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// What may be done with a batch, which is the board's status column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchStatus {
    /// Sellable, and the till will reach for it.
    InStock,
    /// Sellable, but something older should go first.
    SellFirst,
    /// Moved to the quarantine box.
    Quarantined,
    /// Past its date and waiting to be destroyed.
    Expired,
    /// Not yet expired, but going back to the supplier.
    ReturnToSupplier,
    /// Tracked in the controlled register.
    Controlled,
}

impl BatchStatus {
    /// The words on the status chip.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            BatchStatus::InStock => "In stock",
            BatchStatus::SellFirst => "Sell first",
            BatchStatus::Quarantined => "Quarantined",
            BatchStatus::Expired => "Expired \u{b7} disposal",
            BatchStatus::ReturnToSupplier => "Return to supplier",
            BatchStatus::Controlled => "Controlled \u{b7} in register",
        }
    }

    /// How the board colours it.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            BatchStatus::InStock => Tone::Success,
            BatchStatus::SellFirst | BatchStatus::ReturnToSupplier => Tone::Warning,
            BatchStatus::Quarantined | BatchStatus::Expired => Tone::Danger,
            BatchStatus::Controlled => Tone::Neutral,
        }
    }
}

/// One batch, as the board's table shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Batch {
    /// The medicine it is.
    pub medicine: &'static str,
    /// Where it is kept.
    pub location: &'static str,
    /// Its batch code.
    pub code: &'static str,
    /// Which branch holds it.
    pub branch: &'static str,
    /// When it expires.
    pub expires: &'static str,
    /// How much is left, with its unit.
    pub on_hand: &'static str,
    /// What it is worth at cost.
    pub value_at_cost: &'static str,
    /// What may be done with it.
    pub status: BatchStatus,
}

/// One of the five figures across the top.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Figure {
    /// What it counts.
    pub label: &'static str,
    /// How many batches.
    pub count: u32,
    /// The line under it, or the value at cost where the board gives one.
    pub note: &'static str,
    /// How the board colours it.
    pub tone: Tone,
}

/// Everything the batches board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batches {
    /// The figures across the top.
    pub figures: Vec<Figure>,
    /// The batches, soonest expiry first.
    pub rows: Vec<Batch>,
    /// How many batches the board is showing out of all of them.
    pub showing: (usize, u32),
    /// The recall the quarantined batches belong to.
    pub recall: &'static str,
}

/// The board's four figures across the top.
const FIGURES: [Figure; 4] = [
    Figure {
        label: "Expired, still on shelf",
        count: 2,
        note: "Blocked at the till \u{b7} in the quarantine box",
        tone: Tone::Danger,
    },
    Figure {
        label: "Expiring in 30 days",
        count: 5,
        note: "Value 186,400 at cost",
        tone: Tone::Warning,
    },
    Figure {
        label: "Expiring in 90 days",
        count: 14,
        note: "Value 612,900 at cost",
        tone: Tone::Warning,
    },
    Figure {
        label: "Quarantined",
        count: 3,
        note: "Includes AMS-2404 under recall RC-0047",
        tone: Tone::Danger,
    },
];

/// The batches the table shows, first expiry first out.
const ROWS: [Batch; 12] = [
    batch(
        "Cough syrup 100ml",
        "Quarantine box",
        "CSY-2310",
        "Mwenge",
        "28 Sep 2026",
        "6 bottles",
        "15,600",
        BatchStatus::Expired,
    ),
    batch(
        "Vitamin C 500mg",
        "Quarantine box",
        "VTC-2401",
        "Mwenge",
        "1 Oct 2026",
        "3 packs",
        "8,100",
        BatchStatus::Expired,
    ),
    batch(
        "Amoxicillin 250mg/5ml syrup",
        "Blocked at the till since 09:31",
        "AMS-2404",
        "Mwenge",
        "19 Oct 2026",
        "14 bottles",
        "58,800",
        BatchStatus::Quarantined,
    ),
    batch(
        "Metformin 500mg tablets",
        "Near expiry",
        "MTF-2405",
        "Mwenge",
        "30 Oct 2026",
        "9 packs",
        "40,500",
        BatchStatus::ReturnToSupplier,
    ),
    batch(
        "Cetirizine 10mg tablets",
        "Pinned on the till",
        "CTZ-2502",
        "Mwenge",
        "12 Dec 2026",
        "22 packs",
        "30,800",
        BatchStatus::SellFirst,
    ),
    batch(
        "Tramadol 50mg capsules",
        "Locked cabinet",
        "TRM-2411",
        "Mwenge",
        "01/2027",
        "80 caps",
        "36,000",
        BatchStatus::Controlled,
    ),
    batch(
        "Amoxicillin 500mg capsules",
        "Older batch goes first",
        "AMX-2409",
        "Mwenge",
        "03/2027",
        "1,231 caps",
        "141,600",
        BatchStatus::SellFirst,
    ),
    batch(
        "Paracetamol 500mg tablets",
        "Shelf B2",
        "PCM-2502",
        "Tegeta",
        "11/2027",
        "2,600 tabs",
        "41,600",
        BatchStatus::InStock,
    ),
    batch(
        "Paracetamol 500mg tablets",
        "Shelf B2",
        "PCM-2502",
        "Mwenge",
        "11/2027",
        "4,380 tabs",
        "70,100",
        BatchStatus::InStock,
    ),
    batch(
        "Human insulin 100 IU/ml vial",
        "Fridge 1 \u{b7} 2\u{2013}8 \u{b0}C",
        "INS-2507",
        "Mwenge",
        "01/2028",
        "4 vials",
        "84,000",
        BatchStatus::InStock,
    ),
    batch(
        "Oral rehydration salts",
        "Shelf A1",
        "ORS-2507",
        "Tegeta",
        "07/2028",
        "180 sachets",
        "45,000",
        BatchStatus::InStock,
    ),
    batch(
        "Oral rehydration salts",
        "Shelf A1",
        "ORS-2507",
        "Mwenge",
        "07/2028",
        "245 sachets",
        "61,300",
        BatchStatus::InStock,
    ),
];

impl Batches {
    /// The board's batches at Mwenge and Tegeta.
    #[must_use]
    pub fn story() -> Self {
        Self {
            figures: FIGURES.to_vec(),
            rows: ROWS.to_vec(),
            showing: (ROWS.len(), 214),
            recall: "RC-0047",
        }
    }

    /// How many batches may not be sold, which is the number that stops work.
    #[must_use]
    pub fn blocked(&self) -> usize {
        self.rows
            .iter()
            .filter(|batch| {
                matches!(
                    batch.status,
                    BatchStatus::Expired | BatchStatus::Quarantined
                )
            })
            .count()
    }
    /// The first sellable batch for a medicine at `branch`, which is what that
    /// branch's till picks: first expiry first out.
    ///
    /// The branch matters as much as the expiry. Both branches may hold the
    /// same batch code, and a till at Mwenge cannot hand a patient a pack from
    /// Tegeta, so searching across branches would name a batch the counter
    /// cannot reach.
    #[must_use]
    pub fn fefo(&self, branch: &str, medicine: &str) -> Option<&Batch> {
        self.rows
            .iter()
            .filter(|batch| {
                batch.medicine == medicine
                    && batch.branch == branch
                    && matches!(
                        batch.status,
                        BatchStatus::InStock | BatchStatus::SellFirst | BatchStatus::Controlled
                    )
            })
            .min_by_key(|batch| batch.expires)
    }
}

/// Build one row; the board is a long list of the same shape.
#[allow(clippy::too_many_arguments)]
const fn batch(
    medicine: &'static str,
    location: &'static str,
    code: &'static str,
    branch: &'static str,
    expires: &'static str,
    on_hand: &'static str,
    value_at_cost: &'static str,
    status: BatchStatus,
) -> Batch {
    Batch {
        medicine,
        location,
        code,
        branch,
        expires,
        on_hand,
        value_at_cost,
        status,
    }
}

/// The cold-chain unit's last day, as the board charts it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColdChain {
    /// Which fridge and which branch.
    pub unit: &'static str,
    /// The range it has to stay inside.
    pub range: &'static str,
    /// The lowest reading in the last day.
    pub minimum: &'static str,
    /// The highest reading in the last day.
    pub maximum: &'static str,
    /// What the fridge is holding, and what is waiting for it.
    pub holding: &'static str,
}

impl ColdChain {
    /// The board's Fridge 1.
    #[must_use]
    pub const fn story() -> Self {
        Self {
            unit: "Fridge 1 \u{b7} Mwenge",
            range: "In range 2\u{2013}8 \u{b0}C",
            minimum: "3.4 \u{b0}C",
            maximum: "6.8 \u{b0}C",
            holding: "Holds 4 insulin vials (INS-2507), plus 10 from UZ-7781 waiting to be accepted. Logged every 4 hours, checked by John M.",
        }
    }
}

/// The disposal record the board keeps for expired stock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Disposal {
    /// What is being destroyed.
    pub item: &'static str,
    /// How much, and what it was worth.
    pub quantity: &'static str,
    /// Who witnessed it.
    pub witnessed_by: &'static str,
    /// How the regulator wants it done.
    pub method: &'static str,
    /// The certificate number.
    pub certificate: &'static str,
    /// Where it sits until it is collected.
    pub held: &'static str,
}

impl Disposal {
    /// The board's disposal record for the cough syrup.
    #[must_use]
    pub const fn story() -> Self {
        Self {
            item: "Cough syrup 100ml \u{b7} CSY-2310",
            quantity: "6 bottles \u{b7} expired 28 Sep \u{b7} 15,600 at cost",
            witnessed_by: "Grace N., pharmacist in charge",
            method: "[Method per regulator guidance]",
            certificate: "[Disposal certificate no.]",
            held: "Pending \u{b7} kept in the quarantine box until collected",
        }
    }
}

/// The batches table.
fn table(batches: &Batches) -> Div {
    let rows = batches.rows.iter().enumerate().map(|(index, batch)| {
        board::link_to(("batch-row", index), "/batches")
            .sx(board::line_style())
            .child(board::cell_fixed_stack(batch.medicine, batch.location))
            .child(board::cell_fixed(batch.code))
            .child(board::cell_fixed(batch.branch))
            .child(board::cell_fixed(batch.expires))
            .child(board::cell_fixed(batch.on_hand))
            .child(board::cell_number(batch.value_at_cost))
            .child(board::chip(batch.status.label(), batch.status.tone()))
    });
    board::card(2.4)
        .child(board::card_head(
            "Batches, soonest expiry first",
            "The till always picks the first-to-expire batch that is allowed to sell.",
        ))
        .child(board::head(vec![
            board::cell_fixed("Medicine"),
            board::cell_fixed("Batch"),
            board::cell_fixed("Branch"),
            board::cell_fixed("Expires"),
            board::cell_fixed("On hand"),
            board::cell_number("Value at cost"),
            board::cell_fixed("Status"),
        ]))
        .children(rows.collect::<Vec<_>>())
        .child(board::footnote(format!(
            "Showing {} of {} batches \u{b7} controlled items are tracked in the controlled register",
            batches.showing.0, batches.showing.1
        )))
}

/// "Returns to supplier": what the pharmacy expects credit for.
fn returns(
    credit: u32,
    batches: &[(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    )],
) -> Div {
    board::card(1.)
        .child(board::card_title("Returns to supplier"))
        .child(board::meta("Uzima Pharmaceuticals"))
        .children(
            batches
                .iter()
                .enumerate()
                .map(|(index, (medicine, code, value, quantity, reason))| {
                    board::line(vec![
                        board::cell_fixed_stack(*medicine, *code),
                        board::cell_number(*value),
                        board::cell_fixed(*quantity),
                        board::cell_fixed(*reason),
                    ])
                    .id(("return-row", index))
                    .into_any_element()
                })
                .collect::<Vec<_>>(),
        )
        .child(board::key_value("Credit expected", credit.to_string()))
        .child(board::footnote(
            "Credit for expired or near-expiry stock depends on the supplier's return terms.",
        ))
        .child(board::link(
            "print-return-note",
            "/batches",
            "Print return note for the next van",
        ))
}

/// The batches board.
#[component]
pub fn BatchesAndExpiry(
    batches: Batches,
    cold: ColdChain,
    disposal: Disposal,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(board::filters_in(
            mode,
            &[
                ("Both branches", true),
                ("Mwenge", false),
                ("Tegeta", false),
            ],
        ))
        .child(board::stat_row(
            batches
                .figures
                .iter()
                .map(|figure| {
                    board::stat(
                        figure.label,
                        figure.count.to_string(),
                        figure.note,
                        Some(figure.tone),
                        mode,
                    )
                })
                .collect::<Vec<_>>(),
        ))
        .child(
            div().sx(board::row()).child(table(&batches)).child(
                div()
                    .sx(board::row())
                    .child(returns(
                        107_400,
                        &[
                            (
                                "Vitamin C 500mg",
                                "VTC-2401",
                                "8,100",
                                "3 packs",
                                "Expired 1 Oct",
                            ),
                            (
                                "Metformin 500mg",
                                "MTF-2405",
                                "40,500",
                                "9 packs",
                                "Near expiry, 30 Oct",
                            ),
                            (
                                "Amoxicillin 250mg/5ml",
                                "AMS-2404",
                                "58,800",
                                "14 bottles",
                                "Recall RC-0047",
                            ),
                        ],
                    ))
                    .child(disposal_card(&disposal)),
            ),
        )
        .child(
            div().sx(board::row()).child(
                board::card(1.)
                    .child(board::card_head(cold.unit, cold.range))
                    .child(board::stat_row(vec![
                        board::stat("Last 24 h min", cold.minimum, "below 8 \u{b0}C", None, mode),
                        board::stat("Last 24 h max", cold.maximum, "above 2 \u{b0}C", None, mode),
                    ]))
                    .child(board::meta(cold.holding)),
            ),
        )
}

/// The disposal record, as the board shows it beside the returns.
fn disposal_card(disposal: &Disposal) -> Div {
    board::card(1.)
        .child(board::card_title("Disposal record"))
        .child(board::panel(vec![
            board::key_value("Item:", disposal.item).into_any_element(),
            board::key_value("Quantity:", disposal.quantity).into_any_element(),
            board::key_value("Witnessed by:", disposal.witnessed_by).into_any_element(),
            board::key_value("Method:", disposal.method).into_any_element(),
            board::key_value("Certificate:", disposal.certificate).into_any_element(),
        ]))
        .child(board::meta(disposal.held))
}

/// The batches with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    BatchesAndExpiry::new(Batches::story(), ColdChain::story(), Disposal::story())
}

#[cfg(test)]
mod tests {
    use super::{BatchStatus, Batches, ColdChain, Disposal, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_board_says_how_many_batches_are_blocked() {
        let batches = Batches::story();
        assert_eq!(batches.rows.len(), 12);
        assert_eq!(batches.blocked(), 3);
        assert_eq!(batches.showing, (12, 214));
        assert_eq!(
            batches.showing.0,
            batches.rows.len(),
            "the board says how many of its own rows it is showing"
        );
    }

    #[test]
    fn the_till_takes_the_first_to_expire_batch_it_may_sell() {
        let batches = Batches::story();
        assert_eq!(
            batches
                .fefo("Mwenge", "Paracetamol 500mg tablets")
                .map(|batch| batch.on_hand),
            Some("4,380 tabs"),
            "Mwenge's till reaches Mwenge's shelf, not Tegeta's"
        );
        assert_eq!(
            batches
                .fefo("Tegeta", "Paracetamol 500mg tablets")
                .map(|batch| batch.on_hand),
            Some("2,600 tabs"),
            "the other till reaches the other shelf"
        );
        assert_eq!(
            batches.fefo("Mwenge", "Amoxicillin 250mg/5ml syrup"),
            None,
            "AMS-2404 is quarantined, so there is nothing the till may sell"
        );
    }

    #[test]
    fn a_quarantined_batch_says_so() {
        let batches = Batches::story();
        let recalled = batches
            .rows
            .iter()
            .find(|batch| batch.code == "AMS-2404")
            .expect("the board has the recalled batch");
        assert_eq!(recalled.status, BatchStatus::Quarantined);
        assert_eq!(batches.recall, "RC-0047");
    }

    #[test]
    fn the_cold_chain_readings_are_inside_the_range() {
        let cold = ColdChain::story();
        assert_eq!(cold.range, "In range 2\u{2013}8 \u{b0}C");
        assert_eq!(cold.minimum, "3.4 \u{b0}C");
    }

    #[test]
    fn the_disposal_record_names_a_witness() {
        assert_eq!(
            Disposal::story().witnessed_by,
            "Grace N., pharmacist in charge"
        );
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
        }
    }

    #[gpui::test]
    fn draws_the_batches_board(cx: &mut gpui::TestAppContext) {
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
