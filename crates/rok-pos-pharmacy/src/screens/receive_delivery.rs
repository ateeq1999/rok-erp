//! Receiving a delivery: the order that arrived, each line counted against
//! what was ordered, the batch scanned, the cold chain readings and the invoice.
//!
//! Board: `Pharmacy_receive_delivery_batch_expiry_cold_chain.html`. The board's
//! own delivery lives in [`Deliveries::story`].

use std::cmp::Ordering;

use rok_pos_shell::{Tone, group_digits};
use rok_ui::prelude::*;

use crate::screens::board;

/// How far a delivery has got.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrival {
    /// Booked in by the supplier, not here yet.
    Booked,
    /// At the door, waiting to be counted.
    Arrived,
    /// Counted, waiting for a second signature.
    Checked,
}

impl Arrival {
    /// What the board's chip says.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Arrival::Booked => "On its way",
            Arrival::Arrived => "Arrived",
            Arrival::Checked => "Checked",
        }
    }

    /// How urgent the chip looks.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Arrival::Booked => Tone::Info,
            Arrival::Arrived => Tone::Warning,
            Arrival::Checked => Tone::Brand,
        }
    }
}

/// One line of a delivery, as the board's table checks it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    /// What was ordered.
    pub item: &'static str,
    /// The pack.
    pub pack: &'static str,
    /// How many were ordered.
    pub ordered: u32,
    /// How many were counted.
    pub counted: u32,
    /// The batch whose label was scanned, or why it was not.
    pub batch: &'static str,
    /// How long that batch has left.
    pub expiry: &'static str,
    /// Whether the count and the batch both agree.
    pub clean: bool,
    /// Whether the line is accepted into stock.
    pub accepted: bool,
}

impl Line {
    /// Whether the counted number matches what was ordered.
    #[must_use]
    pub fn matches(&self) -> bool {
        self.counted == self.ordered
    }

    /// What the board's result cell says.
    #[must_use]
    pub fn result(&self) -> &'static str {
        match (self.matches(), self.batch.starts_with('[')) {
            (false, _) => "Short",
            (true, true) => "Batch unread",
            (true, false) => "OK",
        }
    }

    /// How the result cell is coloured.
    #[must_use]
    pub fn tone(&self) -> Tone {
        match (self.matches(), self.batch.starts_with('[')) {
            (false, _) => Tone::Danger,
            (true, true) => Tone::Warning,
            (true, false) => Tone::Success,
        }
    }

    /// What this line adds to the accepted value, in shillings.
    ///
    /// Only a line that was counted and whose batch was read is accepted, so a
    /// short or unread line contributes nothing rather than a part figure.
    #[must_use]
    pub fn accepted_shillings(&self) -> i64 {
        if self.accepted {
            i64::from(self.counted) * 20_000
        } else {
            0
        }
    }
}

/// One reading from a cold box logger.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reading {
    /// When it was read.
    pub time: &'static str,
    /// The temperature it recorded.
    pub celsius: f32,
    /// Whether the logger itself is signed off.
    pub signed: bool,
}

impl Reading {
    /// Whether the reading is inside the 2 to 8 degree window insulin needs.
    #[must_use]
    pub fn in_range(&self) -> bool {
        (2.0..=8.0).contains(&self.celsius)
    }
}

/// A delivery waiting to be received.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Delivery {
    /// Its reference.
    pub reference: &'static str,
    /// Who brought it.
    pub supplier: &'static str,
    /// How far along it is.
    pub arrival: Arrival,
    /// What it was booked for.
    pub note: &'static str,
    /// What it is worth.
    pub value: &'static str,
    /// Its path.
    pub href: &'static str,
}

/// Everything the receive board draws.
#[derive(Clone, Debug, PartialEq)]
pub struct Deliveries {
    /// The deliveries still to receive.
    pub arrivals: Vec<Delivery>,
    /// The delivery the board opens on.
    pub reference: &'static str,
    /// Its lines.
    pub lines: Vec<Line>,
    /// The cold chain logger's readings.
    pub readings: Vec<Reading>,
    /// What the invoice says against what was counted.
    pub invoice: &'static str,
}

/// The deliveries still to be booked in.
const ARRIVALS: [Delivery; 4] = [
    Delivery {
        reference: "UZ-7781",
        supplier: "Uzima Pharmaceuticals",
        arrival: Arrival::Arrived,
        note: "Arrived 11:20 \u{b7} 6 lines \u{b7} insulin in cold box",
        value: "1,842,000",
        href: "/receive/UZ-7781",
    },
    Delivery {
        reference: "TM-3390",
        supplier: "Tanzania Medics",
        arrival: Arrival::Checked,
        note: "Counted by John M. \u{b7} awaiting a second signature",
        value: "624,500",
        href: "/receive/TM-3390",
    },
    Delivery {
        reference: "MA-2207",
        supplier: "Medline Africa",
        arrival: Arrival::Booked,
        note: "Expected Tue 7 Oct",
        value: "908,000",
        href: "/receive/MA-2207",
    },
    Delivery {
        reference: "UZ-7742",
        supplier: "Uzima Pharmaceuticals",
        arrival: Arrival::Booked,
        note: "Expected Mon 6 Oct",
        value: "412,300",
        href: "/receive/UZ-7742",
    },
];

/// The lines of UZ-7781, the delivery the board opens on.
const LINES: [Line; 6] = [
    Line {
        item: "Amoxicillin 500mg capsules",
        pack: "box of 100",
        ordered: 10,
        counted: 10,
        batch: "AMX-2409",
        expiry: "03/2027",
        clean: true,
        accepted: true,
    },
    Line {
        item: "Amoxicillin 250mg/5ml syrup",
        pack: "100 ml bottle",
        ordered: 20,
        counted: 20,
        batch: "AMS-2511",
        expiry: "05/2028",
        clean: true,
        accepted: true,
    },
    Line {
        item: "Human insulin 100 IU/ml",
        pack: "10 ml vial",
        ordered: 10,
        counted: 8,
        batch: "INS-2507",
        expiry: "01/2028",
        clean: false,
        accepted: false,
    },
    Line {
        item: "Metronidazole 400mg tablets",
        pack: "box of 100",
        ordered: 15,
        counted: 15,
        batch: "MTZ-2503",
        expiry: "09/2027",
        clean: true,
        accepted: true,
    },
    Line {
        item: "Amlodipine 5mg tablets",
        pack: "box of 100",
        ordered: 12,
        counted: 12,
        batch: "[label not scanned]",
        expiry: "\u{2014}",
        clean: false,
        accepted: false,
    },
    Line {
        item: "Cetirizine 10mg tablets",
        pack: "box of 100",
        ordered: 8,
        counted: 8,
        batch: "CTZ-2502",
        expiry: "12 Dec 2026",
        clean: true,
        accepted: true,
    },
];

/// The cold box logger's readings.
const READINGS: [Reading; 4] = [
    Reading {
        time: "08:00 departure",
        celsius: 4.2,
        signed: true,
    },
    Reading {
        time: "10:15 on the road",
        celsius: 5.1,
        signed: true,
    },
    Reading {
        time: "11:05 at Mwenge",
        celsius: 4.8,
        signed: true,
    },
    Reading {
        time: "11:20 handover",
        celsius: 6.3,
        signed: true,
    },
];

impl Deliveries {
    /// The board's own deliveries.
    #[must_use]
    pub fn story() -> Self {
        Self {
            arrivals: ARRIVALS.to_vec(),
            reference: "UZ-7781",
            lines: LINES.to_vec(),
            readings: READINGS.to_vec(),
            invoice: "Invoice 84-2211 says 8 lines for 1,842,000; 6 lines arrived for 1,610,000",
        }
    }

    /// How many lines still need something done before they can be accepted.
    #[must_use]
    pub fn unresolved(&self) -> Vec<&Line> {
        self.lines.iter().filter(|line| !line.accepted).collect()
    }

    /// What the delivery is worth once only the accepted lines are counted.
    ///
    /// This is deliberately not the invoice figure: an unaccepted line has not
    /// been verified, so it does not become stock.
    #[must_use]
    pub fn accepted_value(&self) -> String {
        let shillings: i64 = self.lines.iter().map(Line::accepted_shillings).sum();
        group_digits(shillings)
    }

    /// Whether every reading stayed inside the cold chain window.
    #[must_use]
    pub fn cold_chain_held(&self) -> bool {
        self.readings.iter().all(Reading::in_range)
    }

    /// The reading that came closest to leaving the window.
    #[must_use]
    pub fn warmest(&self) -> Option<&Reading> {
        self.readings
            .iter()
            .max_by(|a, b| a.celsius.partial_cmp(&b.celsius).unwrap_or(Ordering::Equal))
    }
}

/// The deliveries still to be received, which is what `/receive` lists.
#[component]
pub fn List(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let waiting = ARRIVALS
        .iter()
        .filter(|delivery| delivery.arrival != Arrival::Checked)
        .count();
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "To receive",
                ARRIVALS.len().to_string(),
                "booked in but not yet in stock",
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Waiting at the door",
                waiting.to_string(),
                "counted or about to be",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Value on the dock",
                "3,786,800".to_string(),
                "across every open delivery",
                None,
                mode,
            ),
            board::stat(
                "Cold chain boxes",
                "1".to_string(),
                "loggered \u{b7} read on arrival",
                Some(Tone::Info),
                mode,
            ),
        ]))
        .child(
            board::card(1.)
                .child(board::card_head("Deliveries", "Newest arrival first"))
                .child(board::head(vec![
                    board::cell("Delivery"),
                    board::cell("Supplier"),
                    board::cell_fixed("Value"),
                    board::cell("Note"),
                    board::cell("Stage"),
                    board::cell_fixed(""),
                ]))
                .children(
                    ARRIVALS
                        .iter()
                        .enumerate()
                        .map(|(index, delivery)| {
                            board::line(vec![
                                board::cell_fixed(delivery.reference),
                                board::cell(delivery.supplier),
                                board::cell_number(delivery.value),
                                board::cell(delivery.note),
                                board::chip(delivery.arrival.label(), delivery.arrival.tone())
                                    .into_any_element(),
                                board::link(
                                    ("receive-row", index),
                                    delivery.href,
                                    "Receive",
                                )
                                .into_any_element(),
                            ])
                        })
                        .collect::<Vec<_>>(),
                ),
        )
        .child(board::footnote(
            "A delivery is not stock until it is counted and its batches are scanned: an unread batch cannot be sold or recalled.",
        ))
}

/// The lines of one delivery, checked against what was ordered.
fn lines(deliveries: &Deliveries, mode: ThemeMode) -> Div {
    let unresolved = deliveries.unresolved().len();
    board::card(1.4)
        .child(board::card_head(
            "Check each line",
            format!("{unresolved} of {} not accepted", deliveries.lines.len()),
        ))
        .child(board::head(vec![
            board::cell("Item"),
            board::cell_fixed("Ordered"),
            board::cell_fixed("Counted"),
            board::cell("Batch scanned"),
            board::cell_fixed("Expiry"),
            board::cell_fixed("Result"),
        ]))
        .children(
            deliveries
                .lines
                .iter()
                .map(|line| {
                    board::toned_row(
                        if line.accepted {
                            Tone::Neutral
                        } else {
                            Tone::Warning
                        },
                        mode,
                        vec![
                            board::cell_stack(line.item, line.pack),
                            board::cell_fixed(line.ordered.to_string()),
                            board::cell_fixed(line.counted.to_string()),
                            board::cell(line.batch),
                            board::cell_fixed(line.expiry),
                            board::chip(line.result(), line.tone()).into_any_element(),
                        ],
                    )
                })
                .collect::<Vec<_>>(),
        )
}

/// The cold box logger, which is the reason a temperature is on the board.
fn cold_chain(deliveries: &Deliveries, mode: ThemeMode) -> Div {
    let held = deliveries.cold_chain_held();
    let warmest = deliveries.warmest();
    board::card(1.)
        .child(board::card_head(
            "Insulin cold box logger",
            if held {
                "every reading inside 2 to 8 degrees"
            } else {
                "a reading left the 2 to 8 degree window"
            },
        ))
        .child(board::head(vec![
            board::cell("When"),
            board::cell_fixed("Degrees"),
            board::cell("Signed"),
        ]))
        .children(
            deliveries
                .readings
                .iter()
                .map(|reading| {
                    board::line(vec![
                        board::cell(reading.time),
                        board::cell_fixed(format!("{:.1} C", reading.celsius)),
                        board::chip(
                            if reading.in_range() { "In range" } else { "Excursion" },
                            if reading.in_range() {
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
        .child(board::footnote(format!(
            "Warmest reading {:.1} C.",
            warmest.map_or(0., |reading| reading.celsius)
        )))
        .child(board::alert(
            "An excursion is a quality event",
            "The batch is not thrown away; it is quarantined until the pharmacist decides, and the reading is booked against it.",
            Tone::Info,
            mode,
        ))
}

/// The invoice against the count, and the button that accepts what was checked.
fn invoice(deliveries: &Deliveries, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::card_head("Invoice match", deliveries.reference))
        .child(board::key_value("Counted value", format!(
            "TZS {}",
            deliveries.accepted_value()
        )))
        .child(board::key_value("Invoice says", "TZS 1,610,000"))
        .child(board::meta(deliveries.invoice))
        .child(board::actions(vec![
            Button::new("receive-short")
                .label("Note the shortage")
                .into_any_element(),
            Button::new("receive-accept")
                .label("Accept into stock")
                .into_any_element(),
        ]))
        .child(board::alert(
            "Accepting books only what was checked",
            "A short line stays with the supplier until it is credited, so it is never entered as stock the pharmacy does not have.",
            Tone::Warning,
            mode,
        ))
}

/// The receive board for one delivery.
#[component]
pub fn Receive(deliveries: Deliveries, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let unresolved = deliveries.unresolved().len();
    div()
        .sx((board::root(), &sx))
        .child(board::card_head(
            format!("Receive delivery {}", deliveries.reference),
            "Arrived 11:20 from Uzima Pharmaceuticals",
        ))
        .child(board::stat_row(vec![
            board::stat(
                "Lines arrived",
                deliveries.lines.len().to_string(),
                "against 8 on the order",
                None,
                mode,
            ),
            board::stat(
                "Accepted",
                (deliveries.lines.len() - unresolved).to_string(),
                "counted with a readable batch",
                Some(Tone::Success),
                mode,
            ),
            board::stat(
                "Not accepted",
                unresolved.to_string(),
                "short, or the batch was not scanned",
                Some(Tone::Warning),
                mode,
            ),
            board::stat(
                "Cold chain",
                if deliveries.cold_chain_held() {
                    "Held".to_string()
                } else {
                    "Excursion".to_string()
                },
                "insulin, 2 to 8 degrees",
                Some(Tone::Info),
                mode,
            ),
        ]))
        .child(
            div()
                .sx(board::row())
                .child(lines(&deliveries, mode))
                .child(
                    div()
                        .sx(board::row())
                        .child(cold_chain(&deliveries, mode))
                        .child(invoice(&deliveries, mode)),
                ),
        )
        .child(board::footnote(
            "Receiving a delivery is what makes a batch sellable, so the batch label is read before anything is accepted.",
        ))
}

/// The deliveries still to receive.
#[must_use]
pub fn list() -> impl IntoElement {
    List::new()
}

/// One delivery, `order_id` a code like `UZ-7781`. Phase 3 reads the delivery
/// from the database; until then every id opens the board's delivery.
#[must_use]
pub fn view(_order_id: &str) -> impl IntoElement {
    Receive::new(Deliveries::story())
}

#[cfg(test)]
mod tests {
    use super::{Arrival, Deliveries, view};
    use rok_ui::prelude::*;

    #[test]
    fn only_a_counted_line_with_a_read_batch_becomes_stock() {
        let deliveries = Deliveries::story();
        let unresolved = deliveries.unresolved();
        assert_eq!(unresolved.len(), 2);

        let insulin = unresolved
            .iter()
            .find(|line| line.item.contains("insulin"))
            .expect("the short line");
        assert_eq!(insulin.result(), "Short");
        assert_eq!(insulin.accepted_shillings(), 0);

        let amlodipine = unresolved
            .iter()
            .find(|line| line.item.contains("Amlodipine"))
            .expect("the unscanned line");
        assert_eq!(
            amlodipine.result(),
            "Batch unread",
            "the right count is not enough without the batch"
        );
        assert_eq!(amlodipine.accepted_shillings(), 0);
    }

    #[test]
    fn the_short_line_is_the_insulin_the_cold_box_was_for() {
        let deliveries = Deliveries::story();
        assert_eq!(deliveries.unresolved()[0].item, "Human insulin 100 IU/ml");
        assert!(
            deliveries.cold_chain_held(),
            "the box stayed cold, so the shortage is not a quality event"
        );
        assert_eq!(deliveries.warmest().map(|r| r.celsius), Some(6.3));
    }

    #[test]
    fn the_accepted_value_is_not_the_invoice_figure() {
        let deliveries = Deliveries::story();
        // Four accepted lines of 10, 20, 15 and 8 counted packs at 20,000 each.
        assert_eq!(deliveries.accepted_value(), "1,060,000");
        assert!(
            !deliveries.invoice.contains(&deliveries.accepted_value()),
            "the invoice and the accepted value are different figures on purpose"
        );
    }

    #[test]
    fn the_list_puts_the_delivery_at_the_door_first() {
        assert_eq!(Deliveries::story().arrivals[0].reference, "UZ-7781");
        assert_eq!(Deliveries::story().arrivals[0].arrival, Arrival::Arrived);
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view("UZ-7781")
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
