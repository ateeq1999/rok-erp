//! What to order from the licensed suppliers, and what is already on its way.
//!
//! Board: `Pharmacy_order_from_medicine_suppliers.html`. The board's own
//! suppliers and order live in [`OrderBoard::story`].

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// A supplier of a medicine, as the board's listing lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Listing {
    /// The generic name.
    pub name: &'static str,
    /// The pack the supplier sells.
    pub pack: &'static str,
    /// How many packs of it the pharmacy has or wants.
    pub count: u32,
    /// Who supplies it.
    pub supplier: &'static str,
    /// When they can deliver.
    pub delivery: &'static str,
    /// How long the earliest batch they hold has left.
    pub expiry: &'static str,
    /// Whether it has to be kept cold.
    pub cold_chain: bool,
    /// Whether a prescription is needed for it.
    pub prescription_only: bool,
    /// The supplier's price for the pack.
    pub price: &'static str,
    /// The button on the listing.
    pub action: &'static str,
}

impl Listing {
    /// The chips the board puts under a listing's name.
    #[must_use]
    pub fn chips(&self) -> Vec<AnyElement> {
        let mut chips = Vec::new();
        if self.cold_chain {
            chips.push(board::chip("Cold chain", Tone::Info));
        }
        if self.prescription_only {
            chips.push(board::chip("Prescription only", Tone::Neutral));
        }
        if !chips.is_empty() {
            chips.push(board::cell_fixed(""));
        }
        chips
    }

    /// How urgent the listing looks, which is what the board colours it by.
    ///
    /// A medicine with nothing on the shelf and nothing on its way is the one
    /// that stops the dispensary, so it is the only thing that reads as danger.
    #[must_use]
    pub fn tone(&self) -> Tone {
        match (self.count, self.cold_chain) {
            (0, _) => Tone::Danger,
            (1..=3, true) => Tone::Warning,
            (1..=3, false) => Tone::Info,
            _ => Tone::Neutral,
        }
    }
}

/// One medicine's schedule, as the board's `SCHEDULE` filter bar groups them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Schedule {
    /// What the bar calls it.
    pub label: &'static str,
    /// How many listings are in it.
    pub count: u32,
    /// Whether the board has it selected.
    pub selected: bool,
}

/// One line of an order placed with a supplier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    /// What was ordered.
    pub item: &'static str,
    /// The pack.
    pub pack: &'static str,
    /// How many were ordered.
    pub quantity: u32,
    /// The unit price the supplier quoted.
    pub unit_price: &'static str,
    /// The batch that will arrive, when it is known.
    pub batch: &'static str,
}

/// An order already placed and on its way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Order {
    /// Its reference, as the board titles the rail with it.
    pub reference: &'static str,
    /// Who it was placed with.
    pub supplier: &'static str,
    /// When it was placed and when it lands.
    pub when: &'static str,
    /// Its lines.
    pub lines: &'static [Line],
}

/// Everything the order board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderBoard {
    /// The schedule filters across the top.
    pub schedules: Vec<Schedule>,
    /// The listings the table shows.
    pub listings: Vec<Listing>,
    /// The order the rail shows.
    pub order: Order,
    /// What the board says under the table.
    pub summary: &'static str,
}

/// The board's schedule filters, `SCHEDULE` heading and all.
const SCHEDULES: [Schedule; 4] = [
    Schedule {
        label: "All \u{b7} 412",
        count: 412,
        selected: true,
    },
    Schedule {
        label: "Reorder now \u{b7} 7",
        count: 7,
        selected: false,
    },
    Schedule {
        label: "On order \u{b7} 12",
        count: 12,
        selected: false,
    },
    Schedule {
        label: "Cold chain \u{b7} 9",
        count: 9,
        selected: false,
    },
];

/// The listings the board shows, the ones that stop work first.
const LISTINGS: [Listing; 7] = [
    Listing {
        name: "Amoxicillin",
        pack: "500mg capsules \u{b7} box of 100",
        count: 0,
        supplier: "Uzima Pharmaceuticals",
        delivery: "Tomorrow, by 10:00",
        expiry: "09/2027",
        cold_chain: false,
        prescription_only: true,
        price: "20,000",
        action: "Order",
    },
    Listing {
        name: "Human insulin",
        pack: "100 IU/ml, 10 ml vial",
        count: 4,
        supplier: "Uzima Pharmaceuticals",
        delivery: "Today, in the cold box",
        expiry: "01/2028",
        cold_chain: true,
        prescription_only: true,
        price: "260,000",
        action: "Order",
    },
    Listing {
        name: "Metformin",
        pack: "500mg tablets \u{b7} box of 100",
        count: 0,
        supplier: "Tanzania Medics",
        delivery: "Mon 6 Oct",
        expiry: "05/2028",
        cold_chain: false,
        prescription_only: true,
        price: "12,000",
        action: "Order",
    },
    Listing {
        name: "Amoxicillin",
        pack: "250mg/5ml syrup \u{b7} bottle of 100 ml",
        count: 0,
        supplier: "Uzima Pharmaceuticals",
        delivery: "Tomorrow, by 10:00",
        expiry: "19 Oct 2026",
        cold_chain: false,
        prescription_only: true,
        price: "5,500",
        action: "Order another batch",
    },
    Listing {
        name: "Salbutamol",
        pack: "100mcg inhaler",
        count: 2,
        supplier: "Medline Africa",
        delivery: "Wed 8 Oct",
        expiry: "02/2027",
        cold_chain: false,
        prescription_only: true,
        price: "38,000",
        action: "Order",
    },
    Listing {
        name: "Oral rehydration salts",
        pack: "sachet for 1 litre \u{b7} box of 50",
        count: 245,
        supplier: "Tanzania Medics",
        delivery: "Mon 6 Oct",
        expiry: "07/2028",
        cold_chain: false,
        prescription_only: false,
        price: "500",
        action: "Order more",
    },
    Listing {
        name: "Paracetamol",
        pack: "500mg tablets \u{b7} tin of 1000",
        count: 4_380,
        supplier: "Medline Africa",
        delivery: "Wed 8 Oct",
        expiry: "11/2027",
        cold_chain: false,
        prescription_only: false,
        price: "50,000",
        action: "Order more",
    },
];

/// The order the rail shows, placed with the supplier that delivers today.
const ORDER: Order = Order {
    reference: "UZ-7781",
    supplier: "Uzima Pharmaceuticals",
    when: "Placed Thu 1 Oct 16:20 \u{b7} arriving today",
    lines: &[
        Line {
            item: "Amoxicillin 500mg capsules",
            pack: "box of 100",
            quantity: 10,
            unit_price: "20,000",
            batch: "AMX-2409",
        },
        Line {
            item: "Human insulin 100 IU/ml",
            pack: "10 ml vial",
            quantity: 10,
            unit_price: "26,000",
            batch: "INS-2507 \u{b7} cold box",
        },
        Line {
            item: "Amoxicillin 250mg/5ml syrup",
            pack: "100 ml bottle",
            quantity: 20,
            unit_price: "5,500",
            batch: "not yet allocated",
        },
    ],
};

impl OrderBoard {
    /// The board's own suppliers and order.
    #[must_use]
    pub fn story() -> Self {
        Self {
            schedules: SCHEDULES.to_vec(),
            listings: LISTINGS.to_vec(),
            order: ORDER,
            summary: "Only licensed suppliers are listed. A medicine with nothing on the shelf and nothing on its way is the one that stops the dispensary, so it sits at the top.",
        }
    }

    /// What the order comes to, as the rail totals it.
    ///
    /// The unit prices are the board's own figures, so the sum is a picture of
    /// the order rather than a figure to bill from.
    #[must_use]
    pub fn order_total(&self) -> String {
        let shillings: u64 = self
            .order
            .lines
            .iter()
            .map(|line| {
                line.unit_price
                    .replace(',', "")
                    .parse::<u64>()
                    .unwrap_or_default()
                    * u64::from(line.quantity)
            })
            .sum();
        let (whole, part) = (shillings / 1000, shillings % 1000);
        if part == 0 {
            format!("{whole},000")
        } else {
            format!("{whole},{part:03}")
        }
    }

    /// How many listings have nothing to sell, which is what "reorder now" means.
    #[must_use]
    pub fn out_of_stock(&self) -> usize {
        self.listings
            .iter()
            .filter(|listing| listing.count == 0)
            .count()
    }
}

/// One listing row, as the board's table shows it.
fn listing_row(index: usize, listing: &Listing) -> Div {
    board::line(vec![
        board::cell_stack(listing.name, listing.pack),
        board::cell_number(listing.count.to_string()),
        board::cell(listing.supplier),
        board::cell(listing.delivery),
        board::chip(listing.expiry, listing.tone()).into_any_element(),
        board::cell_fixed(listing.price),
        board::link(("order-listing", index), "/order", listing.action).into_any_element(),
    ])
}

/// The rail the board keeps beside the table: the order already placed.
fn order_rail(order: &Order, total: &str, mode: ThemeMode) -> Div {
    board::card(0.85)
        .child(board::card_head(order.reference, order.when))
        .child(board::key_value("Supplier", order.supplier))
        .child(board::section_label("LINES"))
        .children(
            order
                .lines
                .iter()
                .map(|line| {
                    div()
                        .sx(board::line_style())
                        .child(board::cell_stack(line.item, line.pack))
                        .child(board::cell_fixed(format!(
                            "{} \u{d7} {}",
                            line.quantity, line.unit_price
                        )))
                        .child(board::cell_fixed(line.batch))
                })
                .collect::<Vec<_>>(),
        )
        .child(board::key_value("Total", format!("TZS {total}")))
        .child(board::actions(vec![
            Button::new("order-cancel")
                .label("Cancel line")
                .into_any_element(),
            Button::new("order-receive")
                .label("Receive on arrival")
                .into_any_element(),
        ]))
        .child(board::alert(
            "Cold chain line",
            "The insulin travels in a loggered cold box. Receiving it books the temperature log against batch INS-2507.",
            Tone::Info,
            mode,
        ))
        .child(board::footnote(
            "An order is not a promise of supply: the batches are only fixed when the supplier allocates them.",
        ))
}

/// The order board.
#[component]
pub fn OrderMedicines(order: OrderBoard, #[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let mode = board::mode(cx);
    let total = order.order_total();
    div()
        .sx((board::root(), &sx))
        .child(board::stat_row(vec![
            board::stat(
                "Nothing to sell",
                order.out_of_stock().to_string(),
                "and nothing on its way",
                Some(Tone::Danger),
                mode,
            ),
            board::stat(
                "On order",
                "12 packs".to_string(),
                "across 4 suppliers",
                Some(Tone::Info),
                mode,
            ),
            board::stat(
                "Arriving today",
                "1 delivery".to_string(),
                "Uzima Pharmaceuticals \u{b7} 6 lines",
                Some(Tone::Brand),
                mode,
            ),
            board::stat(
                "Suppliers licensed",
                "4".to_string(),
                "only licensed suppliers are listed",
                None,
                mode,
            ),
        ]))
        .child(board::section_label("SCHEDULE"))
        .child(board::filters_in(
            mode,
            &order
                .schedules
                .iter()
                .map(|schedule| (schedule.label, schedule.selected))
                .collect::<Vec<_>>(),
        ))
        .child(
            div()
                .sx(board::row())
                .child(
                    board::card(1.)
                        .child(board::card_head("Medicines", "Lowest stock first"))
                        .child(board::head(vec![
                            board::cell("Medicine"),
                            board::cell("On hand"),
                            board::cell("Supplier"),
                            board::cell("Delivery"),
                            board::cell("Earliest expiry"),
                            board::cell_fixed("Price"),
                            board::cell_fixed(""),
                        ]))
                        .children(
                            order
                                .listings
                                .iter()
                                .enumerate()
                                .map(|(index, listing)| listing_row(index, listing)),
                        ),
                )
                .child(order_rail(&order.order, &total, mode)),
        )
        .child(board::footnote(order.summary))
}

/// The order board with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    OrderMedicines::new(OrderBoard::story())
}

#[cfg(test)]
mod tests {
    use super::{OrderBoard, view};
    use rok_pos_shell::Tone;
    use rok_ui::prelude::*;

    #[test]
    fn the_order_totals_what_its_lines_cost() {
        let order = OrderBoard::story();
        // 10 x 20,000 + 10 x 26,000 + 20 x 5,500 = 570,000.
        assert_eq!(order.order_total(), "570,000");
        assert_eq!(order.order.lines.len(), 3);
    }

    #[test]
    fn a_medicine_with_nothing_left_reads_as_danger() {
        let order = OrderBoard::story();
        assert_eq!(order.out_of_stock(), 3);
        let amoxicillin = order
            .listings
            .iter()
            .find(|listing| listing.name == "Amoxicillin" && listing.count == 0)
            .expect("the board's first listing");
        assert_eq!(amoxicillin.tone(), Tone::Danger);
        assert_eq!(
            amoxicillin.action, "Order",
            "a stocked medicine asks for more, an empty one asks to order"
        );
    }

    #[test]
    fn the_cold_chain_line_is_the_one_that_needs_the_box() {
        let order = OrderBoard::story();
        let insulin = order
            .listings
            .iter()
            .find(|listing| listing.cold_chain)
            .expect("the board lists insulin");
        assert_eq!(insulin.chips().len(), 3, "chip, chip and a spacer");
        assert!(
            order
                .order
                .lines
                .iter()
                .any(|line| line.batch.contains("cold box")),
            "the order's insulin line says how it travels"
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
