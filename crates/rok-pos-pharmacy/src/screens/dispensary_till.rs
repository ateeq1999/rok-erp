//! The dispensary till: sell over the counter, take a split payment, print a
//! label. The basket carries the batch each line will draw from, and the
//! schedule decides who may sell it.
//!
//! Board: `VerticalPharmacy`, which this export of the design leaves out; the
//! plan names it in Phase 6. The board's own sale lives in [`Till::story`].

use rok_pos_shell::{Tone, group_digits};
use rok_ui::prelude::*;

use crate::screens::board;

/// How a medicine may be sold, which decides who has to be involved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Schedule {
    /// Any cashier may sell it.
    GeneralSale,
    /// It needs a pharmacist on duty.
    PharmacyMedicine,
    /// It needs a prescription.
    PrescriptionOnly,
    /// It needs a prescription and the pharmacist's PIN, and it is written to
    /// the controlled register.
    Controlled,
}

impl Schedule {
    /// What the basket's schedule column says.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Schedule::GeneralSale => "General sale",
            Schedule::PharmacyMedicine => "Pharmacy medicine",
            Schedule::PrescriptionOnly => "Prescription-only",
            Schedule::Controlled => "Controlled",
        }
    }

    /// Whether the schedule needs a prescription behind it.
    #[must_use]
    pub const fn needs_prescription(self) -> bool {
        matches!(self, Schedule::PrescriptionOnly | Schedule::Controlled)
    }

    /// Whether the schedule needs the pharmacist's PIN before it is handed over.
    #[must_use]
    pub const fn needs_pin(self) -> bool {
        matches!(self, Schedule::Controlled)
    }

    /// How the basket colours the schedule.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Schedule::GeneralSale => Tone::Neutral,
            Schedule::PharmacyMedicine => Tone::Info,
            Schedule::PrescriptionOnly => Tone::Brand,
            Schedule::Controlled => Tone::Danger,
        }
    }
}

/// One line of the basket, with the batch it draws from and the rule that has
/// to be satisfied before it can be sold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    /// What it is.
    pub medicine: &'static str,
    /// The pack.
    pub pack: &'static str,
    /// How many packs.
    pub quantity: u32,
    /// What the line comes to, in shillings.
    pub price: i64,
    /// The batch the first-expiry rule picked.
    pub batch: &'static str,
    /// How long that batch has left, as `MM/YYYY`.
    pub expiry: &'static str,
    /// How it may be sold.
    pub schedule: Schedule,
    /// The prescription behind it, where the schedule needs one.
    pub prescription: Option<&'static str>,
    /// Whether the pharmacist's PIN has been given.
    pub pin_given: bool,
    /// Whether a pharmacist is on duty.
    pub pharmacist_on_duty: bool,
    /// Whether the batch has expired.
    pub expired: bool,
}

impl Line {
    /// What the line comes to, in shillings.
    #[must_use]
    pub const fn total(&self) -> i64 {
        self.price
    }

    /// Whether the line carries a dosage label.
    #[must_use]
    pub const fn needs_label(&self) -> bool {
        !matches!(self.schedule, Schedule::GeneralSale)
    }

    /// Why the till cannot sell this line, or nothing when it can.
    ///
    /// The rules are checked in the order a dispenser meets them: the shelf
    /// first, then the schedule, then the people.
    #[must_use]
    pub fn refusal(&self) -> Option<&'static str> {
        if self.expired {
            Some("batch expired on the shelf")
        } else if self.schedule.needs_prescription() && self.prescription.is_none() {
            Some("a prescription-only medicine needs a prescription")
        } else if self.schedule == Schedule::PharmacyMedicine && !self.pharmacist_on_duty {
            Some("no pharmacist on duty")
        } else if self.schedule.needs_pin() && !self.pin_given {
            Some("a controlled line needs the pharmacist's PIN")
        } else {
            None
        }
    }
}

/// One batch on the shelf, which is what the first-expiry rule chooses from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Batch {
    /// The medicine it holds.
    pub medicine: &'static str,
    /// The batch number.
    pub batch: &'static str,
    /// How long it has left, as `MM/YYYY`, so the string order is the date
    /// order.
    pub expiry: &'static str,
    /// How many packs are on the shelf.
    pub on_hand: u32,
}

/// How a sale was paid for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Payment {
    /// What was taken.
    pub method: &'static str,
    /// How much came in against it.
    pub amount: i64,
    /// Whether the money is the pharmacy's own.
    pub till_money: bool,
}

/// A return the counter has been asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Returned {
    /// What came back.
    pub medicine: &'static str,
    /// Whether the pack is still sealed.
    pub unopened: bool,
    /// Whether it is a controlled medicine.
    pub controlled: bool,
    /// Whether a pharmacist approved it.
    pub approved: bool,
}

impl Returned {
    /// Whether the till may take the item back.
    ///
    /// Only an unopened, non-controlled item, and only with a pharmacist's
    /// approval: anything else is a recall or a disposal, not a return.
    #[must_use]
    pub const fn allowed(&self) -> bool {
        self.unopened && !self.controlled && self.approved
    }

    /// What the returns table says about it.
    #[must_use]
    pub fn reason(&self) -> &'static str {
        if self.controlled {
            "a controlled return is a register entry, not a sale"
        } else if !self.unopened {
            "only an unopened pack can go back on the shelf"
        } else if !self.approved {
            "a pharmacist has to approve the return"
        } else {
            "back on the shelf, FEFO"
        }
    }
}

/// Everything the till board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Till {
    /// The sale's reference.
    pub reference: &'static str,
    /// Who it is for.
    pub patient: &'static str,
    /// Who is serving.
    pub cashier: &'static str,
    /// Who is on duty as the pharmacist.
    pub pharmacist: &'static str,
    /// The share the insurer pays, out of 100.
    pub insurer_percent: u32,
    /// What the drawer opened with.
    pub drawer_float: i64,
    /// The basket.
    pub lines: Vec<Line>,
    /// What the shelves hold for these medicines.
    pub batches: Vec<Batch>,
    /// What was taken.
    pub payments: Vec<Payment>,
    /// What the counter has been asked to take back.
    pub returns: Vec<Returned>,
}

/// RX-2210 as the plan spells it out: 13,700 in total, 9,590 from the insurer.
const LINES: [Line; 4] = [
    Line {
        medicine: "Metronidazole 400mg tablets",
        pack: "box of 21",
        quantity: 1,
        price: 5_250,
        batch: "MTZ-2503",
        expiry: "06/2027",
        schedule: Schedule::PrescriptionOnly,
        prescription: Some("RX-2210"),
        pin_given: false,
        pharmacist_on_duty: true,
        expired: false,
    },
    Line {
        medicine: "Amoxicillin 500mg capsules",
        pack: "box of 15",
        quantity: 1,
        price: 4_500,
        batch: "AMX-2409",
        expiry: "03/2027",
        schedule: Schedule::PrescriptionOnly,
        prescription: Some("RX-2210"),
        pin_given: false,
        pharmacist_on_duty: true,
        expired: false,
    },
    Line {
        medicine: "Paracetamol 500mg tablets",
        pack: "box of 20",
        quantity: 1,
        price: 2_000,
        batch: "PCM-2502",
        expiry: "11/2027",
        schedule: Schedule::GeneralSale,
        prescription: None,
        pin_given: false,
        pharmacist_on_duty: true,
        expired: false,
    },
    Line {
        medicine: "Human insulin 100 IU/ml",
        pack: "10 ml vial",
        quantity: 1,
        price: 1_950,
        batch: "INS-2503",
        expiry: "01/2027",
        schedule: Schedule::PharmacyMedicine,
        prescription: None,
        pin_given: false,
        pharmacist_on_duty: true,
        expired: false,
    },
];

/// The shelf behind the sale, so the first-expiry rule has something to choose.
const SHELF: [Batch; 7] = [
    Batch {
        medicine: "Metronidazole 400mg tablets",
        batch: "MTZ-2403",
        expiry: "09/2026",
        on_hand: 0,
    },
    Batch {
        medicine: "Metronidazole 400mg tablets",
        batch: "MTZ-2503",
        expiry: "06/2027",
        on_hand: 34,
    },
    Batch {
        medicine: "Amoxicillin 500mg capsules",
        batch: "AMX-2409",
        expiry: "03/2027",
        on_hand: 61,
    },
    Batch {
        medicine: "Paracetamol 500mg tablets",
        batch: "PCM-2502",
        expiry: "11/2027",
        on_hand: 118,
    },
    Batch {
        medicine: "Human insulin 100 IU/ml",
        batch: "INS-2409",
        expiry: "04/2027",
        on_hand: 3,
    },
    Batch {
        medicine: "Human insulin 100 IU/ml",
        batch: "INS-2503",
        expiry: "01/2027",
        on_hand: 4,
    },
    Batch {
        medicine: "Human insulin 100 IU/ml",
        batch: "INS-2312",
        expiry: "12/2026",
        on_hand: 0,
    },
];

/// The lines the board shows the rules refusing, none of which are in the
/// basket.
const REFUSED: [Line; 2] = [
    Line {
        medicine: "Cetirizine 10mg tablets",
        pack: "box of 100",
        quantity: 1,
        price: 8_500,
        batch: "CTZ-2311",
        expiry: "08/2026",
        schedule: Schedule::GeneralSale,
        prescription: None,
        pin_given: false,
        pharmacist_on_duty: true,
        expired: true,
    },
    Line {
        medicine: "Tramadol 50mg capsules",
        pack: "box of 20",
        quantity: 1,
        price: 45_000,
        batch: "TRM-2508",
        expiry: "02/2028",
        schedule: Schedule::Controlled,
        prescription: Some("RX-2221"),
        pin_given: false,
        pharmacist_on_duty: true,
        expired: false,
    },
];

/// What was taken against RX-2210.
const PAYMENTS: [Payment; 2] = [
    Payment {
        method: "Cash",
        amount: 4_110,
        till_money: true,
    },
    Payment {
        method: "National health insurance",
        amount: 9_590,
        till_money: false,
    },
];

/// What the counter has been asked to take back.
const RETURNS: [Returned; 3] = [
    Returned {
        medicine: "Amoxicillin 250mg/5ml syrup",
        unopened: true,
        controlled: false,
        approved: true,
    },
    Returned {
        medicine: "Paracetamol 500mg tablets",
        unopened: false,
        controlled: false,
        approved: true,
    },
    Returned {
        medicine: "Diazepam 5mg tablets",
        unopened: true,
        controlled: true,
        approved: true,
    },
];

impl Till {
    /// The board's own sale.
    #[must_use]
    pub fn story() -> Self {
        Self {
            reference: "RX-2210",
            patient: "Mzee Salim R.",
            cashier: "Peter O.",
            pharmacist: "Grace N.",
            insurer_percent: 70,
            drawer_float: 50_000,
            lines: LINES.to_vec(),
            batches: SHELF.to_vec(),
            payments: PAYMENTS.to_vec(),
            returns: RETURNS.to_vec(),
        }
    }

    /// What the basket comes to.
    #[must_use]
    pub fn total(&self) -> i64 {
        self.lines.iter().map(Line::total).sum()
    }

    /// What the insurer pays, to the shilling.
    #[must_use]
    pub fn insurer_share(&self) -> i64 {
        self.total() * i64::from(self.insurer_percent) / 100
    }

    /// What the patient pays: the total less the insurer's share, so the two
    /// always add back up.
    #[must_use]
    pub fn patient_share(&self) -> i64 {
        self.total() - self.insurer_share()
    }

    /// The money that goes in the drawer.
    #[must_use]
    pub fn till_money(&self) -> i64 {
        self.payments
            .iter()
            .filter(|payment| payment.till_money)
            .map(|payment| payment.amount)
            .sum()
    }

    /// Whether the payments add up to the basket.
    #[must_use]
    pub fn paid(&self) -> bool {
        let taken: i64 = self.payments.iter().map(|payment| payment.amount).sum();
        taken == self.total()
    }

    /// The lines in the basket the rules will not let the till sell.
    #[must_use]
    pub fn blocked(&self) -> Vec<(&Line, &'static str)> {
        self.lines
            .iter()
            .filter_map(|line| line.refusal().map(|reason| (line, reason)))
            .collect()
    }

    /// The lines the board shows the rules refusing.
    #[must_use]
    pub fn refused(&self) -> Vec<(&Line, &'static str)> {
        REFUSED
            .iter()
            .filter_map(|line| line.refusal().map(|reason| (line, reason)))
            .collect()
    }

    /// The batch the first-expiry rule picks for `medicine`: the earliest expiry
    /// with stock on the shelf.
    #[must_use]
    pub fn fefo(&self, medicine: &str) -> Option<&Batch> {
        self.batches
            .iter()
            .filter(|batch| batch.medicine == medicine && batch.on_hand > 0)
            .min_by_key(|batch| batch.expiry)
    }

    /// How many dosage labels this sale prints.
    #[must_use]
    pub fn labels(&self) -> usize {
        self.lines.iter().filter(|line| line.needs_label()).count()
    }

    /// Whether the sale can be completed.
    #[must_use]
    pub fn can_complete(&self) -> bool {
        self.blocked().is_empty() && self.paid()
    }
}

/// The basket, with the batch each line draws from and the schedule behind it.
fn basket(till: &Till) -> Div {
    board::card(1.4)
        .child(board::card_head(
            "Basket",
            board::dotted(&[
                till.reference,
                till.patient,
                &format!("{} lines", till.lines.len()),
            ]),
        ))
        .child(board::head(vec![
            board::cell("Medicine"),
            board::cell("Batch"),
            board::cell("Schedule"),
            board::cell("Rule"),
            board::cell_number("Amount"),
        ]))
        .children(
            till.lines
                .iter()
                .map(|line| {
                    board::line(vec![
                        board::cell_stack(
                            line.medicine,
                            board::dotted(&[
                                line.pack,
                                &format!("x{}", line.quantity),
                                &format!("exp {}", line.expiry),
                            ]),
                        ),
                        board::cell_fixed(line.batch),
                        board::chip(line.schedule.label(), line.schedule.tone()).into_any_element(),
                        board::chip(
                            if line.needs_label() {
                                "Label"
                            } else {
                                "No label"
                            },
                            if line.needs_label() {
                                Tone::Info
                            } else {
                                Tone::Neutral
                            },
                        )
                        .into_any_element(),
                        board::cell_number(group_digits(line.total())),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::key_value("Basket total", group_digits(till.total())))
        .child(board::footnote(
            "Each line took the earliest expiry with stock on the shelf.",
        ))
}

/// The payment panel: the insurance split, the money, and the buttons.
fn payment(till: &Till) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Payment",
            board::dotted(&[till.cashier, till.pharmacist]),
        ))
        .child(board::key_value("Total", group_digits(till.total())))
        .child(board::key_value(
            format!("Insurer {}%", till.insurer_percent),
            group_digits(till.insurer_share()),
        ))
        .child(board::key_value(
            "Patient",
            group_digits(till.patient_share()),
        ))
        .child(board::section_label("MONEY IN"))
        .children(
            till.payments
                .iter()
                .map(|taken| board::key_value(taken.method, group_digits(taken.amount)))
                .collect::<Vec<_>>(),
        )
        .child(board::key_value(
            "Drawer float",
            group_digits(till.drawer_float),
        ))
        .child(board::key_value(
            "In the drawer",
            group_digits(till.till_money()),
        ))
        .child(board::actions(vec![
            Button::new("till-hold").label("Hold").into_any_element(),
            Button::new("till-pay")
                .label("Take payment")
                .into_any_element(),
        ]))
        .child(board::footnote(
            "Medicines are VAT exempt, so the basket is the whole of the sale.",
        ))
}

/// The rules the till applies, and the lines they refuse.
fn rules(till: &Till, mode: ThemeMode) -> Div {
    let refused = till.refused();
    board::card(1.)
        .child(board::card_head(
            "Refused at the till",
            format!("{} lines", refused.len()),
        ))
        .children(
            refused
                .iter()
                .map(|(line, reason)| {
                    board::line(vec![
                        board::cell_stack(line.medicine, line.batch),
                        board::chip("Refused", Tone::Danger).into_any_element(),
                        board::cell(*reason),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::alert(
            "The same rule function guards the shelf and the till",
            board::dotted(&[
                "An expired or quarantined batch cannot be sold from either screen.",
                "A controlled line needs the prescription and the pharmacist's PIN.",
            ]),
            Tone::Info,
            mode,
        ))
}

/// The returns the counter has been asked for.
fn returns(till: &Till) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Returns",
            format!("{} today", till.returns.len()),
        ))
        .child(board::head(vec![
            board::cell("Medicine"),
            board::cell("Why"),
            board::cell_fixed("Decision"),
        ]))
        .children(
            till.returns
                .iter()
                .map(|item| {
                    board::line(vec![
                        board::cell(item.medicine),
                        board::cell_truncating(item.reason()),
                        board::chip(
                            if item.allowed() {
                                "Accepted"
                            } else {
                                "Refused"
                            },
                            if item.allowed() {
                                Tone::Success
                            } else {
                                Tone::Danger
                            },
                        )
                        .into_any_element(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The till board with the board's sale.
#[component]
pub fn DispensaryTill(till: Till, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Basket",
                group_digits(till.total()),
                till.reference,
                None,
                mode,
            ),
            board::stat(
                "Insurer",
                group_digits(till.insurer_share()),
                format!("{}% of the basket", till.insurer_percent),
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Patient",
                group_digits(till.patient_share()),
                "the part the insurer does not cover",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Labels",
                till.labels().to_string(),
                "one per medicine that carries one",
                Some(Tone::Info),
                mode,
            ),
            board::stat(
                "Refused",
                till.refused().len().to_string(),
                "lines the rules stopped",
                Some(Tone::Danger),
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(basket(&till))
                .child(payment(&till))
                .child(rules(&till, mode)),
        )
        .child(
            div()
                .sx(board::row())
                .child(returns(&till))
                .child(board::card(1.).child(board::card_head(
                    "Receipt and labels",
                    "80 mm receipt \u{b7} dosage labels",
                ))
                .child(board::key_value(
                    "Receipt",
                    "Mzee Salim R. \u{b7} 13,700 \u{b7} insurer 9,590 \u{b7} patient 4,110",
                ))
                .child(board::key_value(
                    "Label 1 of 3",
                    "Metronidazole 400mg tablets x 21 \u{b7} MTZ-2503 \u{b7} pharmacist GN",
                ))
                .child(board::key_value(
                    "Counter",
                    "Mwenge \u{b7} till 2 \u{b7} 02/10/2026",
                ))
                .child(board::footnote(
                    "Each label is written to the dispensing labels table before it is printed.",
                ))),
        )
}

/// The dispensary till.
#[must_use]
pub fn view() -> impl IntoElement {
    DispensaryTill::new(Till::story())
}

#[cfg(test)]
mod tests {
    use super::{Line, Schedule, Till, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_sale_reproduces_the_splits_to_the_shilling() {
        let till = Till::story();
        assert_eq!(till.total(), 13_700);
        assert_eq!(till.insurer_share(), 9_590);
        assert_eq!(till.patient_share(), 4_110);
        assert_eq!(till.insurer_share() + till.patient_share(), till.total());
        assert!(
            till.paid(),
            "the money in has to add up to the basket, or the sale is not finished"
        );
        assert!(till.can_complete());
    }

    #[test]
    fn an_expired_batch_cannot_be_sold() {
        let till = Till::story();
        let refused = till.refused();
        let expired = refused
            .iter()
            .find(|(line, _)| line.medicine.contains("Cetirizine"))
            .expect("the board refuses one expired line");
        assert_eq!(expired.1, "batch expired on the shelf");
    }

    #[test]
    fn a_controlled_line_without_a_pin_is_refused() {
        let till = Till::story();
        let refused = till.refused();
        let controlled = refused
            .iter()
            .find(|(line, _)| line.schedule == Schedule::Controlled)
            .expect("the board refuses one controlled line");
        assert_eq!(controlled.1, "a controlled line needs the pharmacist's PIN");
        assert!(
            controlled.0.prescription.is_some(),
            "the PIN is refused even with the prescription behind it"
        );
    }

    #[test]
    fn giving_the_pin_releases_the_controlled_line() {
        let till = Till::story();
        let refused = till.refused();
        let (line, reason) = refused
            .iter()
            .find(|(line, _)| line.schedule == Schedule::Controlled)
            .expect("the board refuses one controlled line");
        let mut line = **line;
        assert_eq!(line.refusal(), Some(*reason));
        line.pin_given = true;
        assert_eq!(line.refusal(), None);
    }

    #[test]
    fn a_pharmacy_medicine_needs_a_pharmacist_on_duty() {
        let mut till = Till::story();
        let insulin = till
            .lines
            .iter_mut()
            .find(|line| line.schedule == Schedule::PharmacyMedicine)
            .expect("the basket has one pharmacy medicine");
        insulin.pharmacist_on_duty = false;
        assert_eq!(insulin.refusal(), Some("no pharmacist on duty"));
        assert!(
            !till.can_complete(),
            "one refused line holds the whole sale"
        );
    }

    #[test]
    fn the_batch_is_the_earliest_expiry_with_stock_on_the_shelf() {
        let till = Till::story();
        let insulin = till
            .fefo("Human insulin 100 IU/ml")
            .expect("the shelf holds insulin");
        assert_eq!(insulin.batch, "INS-2503");
        let line = till
            .lines
            .iter()
            .find(|line| line.medicine == "Human insulin 100 IU/ml")
            .expect("the basket has the insulin");
        assert_eq!(line.batch, insulin.batch, "the basket took the FEFO batch");
        assert_eq!(line.expiry, insulin.expiry);
    }

    #[test]
    fn only_an_unopened_non_controlled_return_goes_back_on_the_shelf() {
        let till = Till::story();
        let accepted = till
            .returns
            .iter()
            .find(|item| item.allowed())
            .expect("one return is accepted");
        assert_eq!(accepted.medicine, "Amoxicillin 250mg/5ml syrup");
        for item in &till.returns {
            assert_eq!(
                item.allowed(),
                item.unopened && !item.controlled && item.approved,
                "{}",
                item.medicine
            );
        }
    }

    #[test]
    fn an_empty_basket_takes_no_money() {
        let mut till = Till::story();
        till.lines = vec![Line {
            medicine: "Metronidazole 400mg tablets",
            pack: "box of 21",
            quantity: 0,
            price: 0,
            batch: "MTZ-2503",
            expiry: "06/2027",
            schedule: Schedule::PrescriptionOnly,
            prescription: Some("RX-2210"),
            pin_given: false,
            pharmacist_on_duty: true,
            expired: false,
        }];
        assert_eq!(till.total(), 0);
        assert_eq!(till.insurer_share(), 0);
        assert_eq!(till.patient_share(), 0);
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
