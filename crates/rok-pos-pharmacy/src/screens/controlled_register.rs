//! The controlled medicines register: a running balance per substance, written
//! one signed entry at a time, with the daily count and the variance that a
//! pharmacist has to explain before the substance can be sold again.
//!
//! Board: `VerticalPharmacyRegister`, which this export of the design leaves
//! out; the plan names it in Phase 8. The board's own substances live in
//! [`Register::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// What kind of movement an entry records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Movement {
    /// The balance carried over from yesterday.
    Opening,
    /// Stock booked in on a delivery.
    Received,
    /// Stock handed to a patient against a prescription.
    Dispensed,
    /// Stock destroyed with a pharmacist watching.
    Destroyed,
    /// Stock handed back by a patient.
    Returned,
}

impl Movement {
    /// What the register's movement column says.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Movement::Opening => "Opening balance",
            Movement::Received => "Received",
            Movement::Dispensed => "Dispensed",
            Movement::Destroyed => "Destroyed",
            Movement::Returned => "Returned",
        }
    }

    /// Whether the movement adds to the balance.
    #[must_use]
    pub const fn adds(self) -> bool {
        matches!(
            self,
            Movement::Opening | Movement::Received | Movement::Returned
        )
    }

    /// How the register colours the movement.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Movement::Opening | Movement::Received | Movement::Returned => Tone::Info,
            Movement::Dispensed | Movement::Destroyed => Tone::Warning,
        }
    }
}

/// One signed line of the register. Nothing here is ever edited: a correction
/// is another entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    /// When it was written.
    pub time: &'static str,
    /// What it records.
    pub movement: Movement,
    /// How much moved, positive for a movement that adds to the balance.
    pub quantity: i64,
    /// The balance after this entry.
    pub balance: i64,
    /// Who wrote it.
    pub by: &'static str,
    /// What it was for.
    pub note: &'static str,
    /// Whether the pharmacist's PIN is against it.
    pub signed: bool,
}

impl Entry {
    /// How the register's quantity column reads, so a dispensing is never
    /// confused with a receipt.
    #[must_use]
    pub fn quantity_label(&self) -> String {
        if self.movement.adds() {
            format!("+{}", self.quantity)
        } else {
            format!("-{}", self.quantity)
        }
    }
}

/// One controlled substance and where its balance stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Substance {
    /// What it is called.
    pub name: &'static str,
    /// The schedule it sits on.
    pub schedule: &'static str,
    /// What a single unit is.
    pub unit: &'static str,
    /// The balance the register ends the day on.
    pub balance: i64,
    /// What the morning count found, where a count has been taken.
    pub counted: Option<i64>,
    /// Whether a pharmacist has explained the variance.
    pub explained: bool,
}

impl Substance {
    /// The variance between the expected balance and what was counted.
    #[must_use]
    pub fn variance(&self) -> i64 {
        self.counted.map_or(0, |counted| counted - self.balance)
    }

    /// Whether controlled sales of this substance are blocked.
    ///
    /// A count that does not agree is the one thing the register cannot carry:
    /// it blocks until a pharmacist records why the shelf and the book differ.
    #[must_use]
    pub fn blocked(&self) -> bool {
        self.variance() != 0 && !self.explained
    }

    /// What the register's state chip says.
    #[must_use]
    pub fn state(&self) -> &'static str {
        if self.blocked() {
            "Blocked"
        } else if self.variance() != 0 {
            "Explained"
        } else {
            "Balanced"
        }
    }

    /// How the register colours that state.
    #[must_use]
    pub fn tone(&self) -> Tone {
        if self.blocked() {
            Tone::Danger
        } else if self.variance() != 0 {
            Tone::Warning
        } else {
            Tone::Success
        }
    }
}

/// The daily count, which is the same number the inspector asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Count {
    /// When the count was taken.
    pub time: &'static str,
    /// Which substance it was.
    pub substance: &'static str,
    /// What the balance says it should be.
    pub expected: i64,
    /// What was actually on the shelf.
    pub counted: i64,
    /// Who counted.
    pub by: &'static str,
}

impl Count {
    /// Counted minus expected, which is the variance.
    #[must_use]
    pub fn variance(&self) -> i64 {
        self.counted - self.expected
    }
}

/// Everything the register board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    /// The substances the pharmacy holds.
    pub substances: Vec<Substance>,
    /// The entries of the substance the board opens on.
    pub entries: Vec<Entry>,
    /// The morning count of every substance.
    pub counts: Vec<Count>,
    /// Which substance the board opens on.
    pub focus: &'static str,
}

/// The four substances the pharmacy holds, in the register's order.
const SUBSTANCES: [Substance; 4] = [
    Substance {
        name: "Tramadol 50mg capsules",
        schedule: "Part III",
        unit: "capsules",
        balance: 80,
        counted: Some(80),
        explained: false,
    },
    Substance {
        name: "Morphine 10mg/ml injection",
        schedule: "Part II",
        unit: "ampoules",
        balance: 26,
        counted: Some(26),
        explained: false,
    },
    Substance {
        name: "Diazepam 5mg tablets",
        schedule: "Part IV",
        unit: "tablets",
        balance: 140,
        counted: Some(138),
        explained: false,
    },
    Substance {
        name: "Methylphenidate 10mg tablets",
        schedule: "Part II",
        unit: "tablets",
        balance: 30,
        counted: Some(30),
        explained: false,
    },
];

/// Tramadol's register for the day, as the plan spells it out: received 100,
/// dispensed 20 and 10, balance 80.
const TRAMADOL: [Entry; 5] = [
    Entry {
        time: "08:00",
        movement: Movement::Opening,
        quantity: 10,
        balance: 10,
        by: "John M.",
        note: "Balance carried from yesterday, sealed cupboard",
        signed: true,
    },
    Entry {
        time: "08:15",
        movement: Movement::Opening,
        quantity: 0,
        balance: 10,
        by: "John M.",
        note: "Count before the delivery was opened",
        signed: true,
    },
    Entry {
        time: "09:20",
        movement: Movement::Received,
        quantity: 100,
        balance: 110,
        by: "John M.",
        note: "Delivery UZ-7781 \u{b7} batch TRM-2508 \u{b7} register written with the receipt",
        signed: true,
    },
    Entry {
        time: "10:05",
        movement: Movement::Dispensed,
        quantity: 20,
        balance: 90,
        by: "Grace N.",
        note: "Mzee Salim R. \u{b7} RX-2216 \u{b7} batch TRM-2508 \u{b7} PIN 1234",
        signed: true,
    },
    Entry {
        time: "14:40",
        movement: Movement::Dispensed,
        quantity: 10,
        balance: 80,
        by: "Grace N.",
        note: "J. Mwakyembe \u{b7} RX-2221 \u{b7} batch TRM-2508 \u{b7} PIN 1234",
        signed: true,
    },
];

/// The morning count, one line per substance.
const COUNTS: [Count; 4] = [
    Count {
        time: "08:15",
        substance: "Tramadol 50mg capsules",
        expected: 10,
        counted: 10,
        by: "John M.",
    },
    Count {
        time: "08:15",
        substance: "Morphine 10mg/ml injection",
        expected: 26,
        counted: 26,
        by: "John M.",
    },
    Count {
        time: "08:20",
        substance: "Diazepam 5mg tablets",
        expected: 140,
        counted: 138,
        by: "Peter O.",
    },
    Count {
        time: "08:25",
        substance: "Methylphenidate 10mg tablets",
        expected: 30,
        counted: 30,
        by: "Peter O.",
    },
];

impl Register {
    /// The board's own register.
    #[must_use]
    pub fn story() -> Self {
        Self {
            substances: SUBSTANCES.to_vec(),
            entries: TRAMADOL.to_vec(),
            counts: COUNTS.to_vec(),
            focus: "Tramadol 50mg capsules",
        }
    }

    /// A substance by name, which the board's tables look up.
    #[must_use]
    pub fn substance(&self, name: &str) -> Option<&Substance> {
        self.substances.iter().find(|item| item.name == name)
    }

    /// The substances that cannot be sold until a pharmacist explains the
    /// count.
    #[must_use]
    pub fn blocked(&self) -> Vec<&Substance> {
        self.substances
            .iter()
            .filter(|substance| substance.blocked())
            .collect()
    }

    /// The substance the board opens on.
    ///
    /// # Panics
    ///
    /// Panics when the board names a substance it does not hold, which cannot
    /// happen on a board that opens on a substance it holds.
    #[must_use]
    pub fn focus(&self) -> &Substance {
        self.substance(self.focus)
            .expect("the board opens on a substance it holds")
    }

    /// The balance the entries add up to, which has to be the one on file.
    #[must_use]
    pub fn ledger_balance(&self) -> i64 {
        self.entries
            .iter()
            .map(|entry| {
                if entry.movement.adds() {
                    entry.quantity
                } else {
                    -entry.quantity
                }
            })
            .sum()
    }

    /// Whether every entry in the ledger carries the pharmacist's signature.
    #[must_use]
    pub fn all_signed(&self) -> bool {
        self.entries.iter().all(|entry| entry.signed)
    }

    /// What the register dispensed in the day, which is the figure the count
    /// has to reconcile with.
    #[must_use]
    pub fn dispensed(&self) -> i64 {
        self.entries
            .iter()
            .filter(|entry| entry.movement == Movement::Dispensed)
            .map(|entry| entry.quantity)
            .sum()
    }

    /// What the register took in.
    #[must_use]
    pub fn received(&self) -> i64 {
        self.entries
            .iter()
            .filter(|entry| entry.movement == Movement::Received)
            .map(|entry| entry.quantity)
            .sum()
    }
}

/// The substances and where each balance stands.
fn balances(register: &Register) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Balances by substance",
            format!("{} substances", register.substances.len()),
        ))
        .child(board::head(vec![
            board::cell("Substance"),
            board::cell_fixed("Schedule"),
            board::cell_number("Balance"),
            board::cell_fixed("Count"),
            board::cell_fixed("State"),
        ]))
        .children(
            register
                .substances
                .iter()
                .map(|substance| {
                    board::line(vec![
                        board::cell_stack(substance.name, substance.unit),
                        board::cell_fixed(substance.schedule),
                        board::cell_number(substance.balance.to_string()),
                        board::cell_fixed(
                            substance
                                .counted
                                .map_or("\u{2014}".to_string(), |counted| counted.to_string()),
                        ),
                        board::chip(substance.state(), substance.tone()).into_any_element(),
                    ])
                })
                .collect::<Vec<_>>(),
        )
}

/// The running balance, one signed entry at a time.
fn entries(register: &Register) -> Div {
    board::card(1.2)
        .child(board::card_head(
            format!("{} register", register.focus),
            "append only \u{b7} a correction is another entry",
        ))
        .child(board::head(vec![
            board::cell_fixed("Time"),
            board::cell("Movement"),
            board::cell_number("Qty"),
            board::cell_number("Balance"),
            board::cell("Note"),
            board::cell_fixed("Signed"),
        ]))
        .children(
            register
                .entries
                .iter()
                .map(|entry| {
                    let cells = vec![
                        board::cell_fixed(entry.time),
                        board::chip(entry.movement.label(), entry.movement.tone())
                            .into_any_element(),
                        board::cell_number(entry.quantity_label()),
                        board::cell_number(entry.balance.to_string()),
                        board::cell_truncating(entry.note),
                        board::chip(
                            if entry.signed { "PIN" } else { "Unsigned" },
                            if entry.signed {
                                Tone::Success
                            } else {
                                Tone::Danger
                            },
                        )
                        .into_any_element(),
                    ];
                    board::line(cells)
                })
                .collect::<Vec<_>>(),
        )
        .child(board::key_value(
            "Received today",
            register.received().to_string(),
        ))
        .child(board::key_value(
            "Dispensed today",
            register.dispensed().to_string(),
        ))
        .child(board::key_value(
            "Balance from the ledger",
            register.ledger_balance().to_string(),
        ))
}

/// The morning count, which is the same figure an inspector asks for.
fn counts(register: &Register) -> Div {
    board::card(1.)
        .child(board::card_head(
            "Daily count",
            "one count per substance, before the shop opens",
        ))
        .child(board::head(vec![
            board::cell_fixed("Time"),
            board::cell("Substance"),
            board::cell_number("Expected"),
            board::cell_number("Counted"),
            board::cell_number("Variance"),
            board::cell("By"),
        ]))
        .children(
            register
                .counts
                .iter()
                .map(|count| {
                    let variance = count.variance();
                    board::line(vec![
                        board::cell_fixed(count.time),
                        board::cell(count.substance),
                        board::cell_number(count.expected.to_string()),
                        board::cell_number(count.counted.to_string()),
                        board::cell_number(format!("{variance:+}")),
                        board::cell(count.by),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .child(board::actions(vec![
            Button::new("register-count")
                .label("Record a count")
                .into_any_element(),
            Button::new("register-explain")
                .label("Explain the variance")
                .into_any_element(),
        ]))
}

/// The controlled register board.
#[component]
pub fn ControlledRegister(register: Register, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let blocked = register.blocked();
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Substances",
                register.substances.len().to_string(),
                "on the controlled schedule",
                None,
                mode,
            ),
            board::stat(
                "Received",
                register.received().to_string(),
                "written with the goods receipt",
                Some(Tone::Info),
                mode,
            ),
            board::stat(
                "Dispensed",
                register.dispensed().to_string(),
                "each against a prescription and a PIN",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Balance",
                register.ledger_balance().to_string(),
                format!("{} on the shelf", register.focus().name),
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Blocked",
                blocked.len().to_string(),
                "count does not agree",
                Some(Tone::Danger),
                mode,
            ),
        ]))
        .child(board::alert(
            "A mismatch blocks controlled sales",
            board::dotted(&[
                "The register is append only: a correction is a new signed entry, never an edit.",
                "Until a pharmacist explains the variance, the substance cannot be sold.",
            ]),
            if blocked.is_empty() {
                Tone::Success
            } else {
                Tone::Danger
            },
            mode,
        ))
        .child(balances(&register))
        .child(
            div()
                .sx(board::row())
                .child(entries(&register))
                .child(counts(&register)),
        )
        .child(board::footnote(
            "Export the register as PDF and CSV for inspection.",
        ))
        .child(board::actions(vec![
            Button::new("register-pdf")
                .label("Export PDF")
                .into_any_element(),
            Button::new("register-csv")
                .label("Export CSV")
                .into_any_element(),
        ]))
}

/// The controlled register.
#[must_use]
pub fn view() -> impl IntoElement {
    ControlledRegister::new(Register::story())
}

#[cfg(test)]
mod tests {
    use super::{Entry, Movement, Register, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_tramadol_balance_is_the_arithmetic_of_the_ledger() {
        let register = Register::story();
        assert_eq!(register.received(), 100);
        assert_eq!(register.dispensed(), 30);
        assert_eq!(
            register.ledger_balance(),
            80,
            "10 carried in, 100 received, 30 dispensed"
        );
        assert_eq!(register.focus().balance, register.ledger_balance());
    }

    #[test]
    fn every_entry_is_signed_because_the_register_is_append_only() {
        let register = Register::story();
        assert!(register.all_signed(), "an unsigned entry is not a register");
        assert!(register.entries.iter().all(|entry| !entry.by.is_empty()));
    }

    #[test]
    fn a_count_that_does_not_agree_blocks_the_substance() {
        let register = Register::story();
        let blocked = register.blocked();
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].name, "Diazepam 5mg tablets");
        assert_eq!(blocked[0].variance(), -2);
        assert!(
            !register
                .substance("Morphine 10mg/ml injection")
                .expect("held")
                .blocked(),
            "a substance whose count agrees is never blocked"
        );
    }

    #[test]
    fn explaining_the_variance_unblocks_the_substance() {
        let mut register = Register::story();
        let diazepam = register
            .substances
            .iter_mut()
            .find(|substance| substance.name == "Diazepam 5mg tablets")
            .expect("held");
        assert!(diazepam.blocked());
        diazepam.explained = true;
        assert_eq!(diazepam.state(), "Explained");
        assert_eq!(register.blocked().len(), 0);
    }

    #[test]
    fn a_dispensing_never_reads_as_a_receipt() {
        let register = Register::story();
        let dispensed: Vec<&Entry> = register
            .entries
            .iter()
            .filter(|entry| entry.movement == Movement::Dispensed)
            .collect();
        assert_eq!(dispensed.len(), 2);
        assert_eq!(dispensed[0].quantity_label(), "-20");
        assert_eq!(dispensed[1].quantity_label(), "-10");
        assert_eq!(
            register.entries[2].quantity_label(),
            "+100",
            "the receipt is positive"
        );
    }

    #[test]
    fn the_morning_count_is_the_number_the_register_carries() {
        let register = Register::story();
        let tramadol = register
            .counts
            .iter()
            .find(|count| count.substance == "Tramadol 50mg capsules")
            .expect("every substance is counted");
        assert_eq!(tramadol.time, "08:15");
        assert_eq!(tramadol.variance(), 0);
        assert_eq!(tramadol.counted, 10, "the count is before the delivery");
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
