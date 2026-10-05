//! The medicine catalogue: every product the pharmacy may sell, what it is
//! regulated as, how much is on hand and what it costs.
//!
//! Board: `Pharmacy_medicine_catalogue.html`. The board's own catalogue lives
//! in [`Catalogue::story`]; the catalogue module fills it in.

use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::screens::board;

/// How a medicine may be sold, which decides the chip beside its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Schedule {
    /// Only against a prescription.
    PrescriptionOnly,
    /// A medicine the pharmacy may recommend.
    PharmacyMedicine,
    /// Sold freely over the counter.
    GeneralSale,
    /// Tracked in the controlled register.
    Controlled,
}

impl Schedule {
    /// The word the board's schedule column shows.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Schedule::PrescriptionOnly => "Prescription-only",
            Schedule::PharmacyMedicine => "Pharmacy medicine",
            Schedule::GeneralSale => "General sale",
            Schedule::Controlled => "Controlled",
        }
    }

    /// How the board colours the chip.
    #[must_use]
    pub const fn tone(self) -> Tone {
        match self {
            Schedule::PrescriptionOnly => Tone::Info,
            Schedule::PharmacyMedicine => Tone::Brand,
            Schedule::GeneralSale => Tone::Neutral,
            Schedule::Controlled => Tone::Warning,
        }
    }
}

/// How a medicine is stored, which decides whether it needs a cold chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Storage {
    /// The shelf.
    Room,
    /// A fridge, held between 2 and 8 degrees.
    ColdChain,
}

impl Storage {
    /// The word the board's storage column shows.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Storage::Room => "Room",
            Storage::ColdChain => "Cold 2\u{2013}8 \u{b0}C",
        }
    }
}

/// One medicine in the catalogue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Medicine {
    /// Its generic name, which is how the catalogue is ordered.
    pub generic: &'static str,
    /// Strength, form and the brand, as the board's second line.
    pub detail: &'static str,
    /// What a pack is.
    pub pack: &'static str,
    /// What it is regulated as.
    pub schedule: Schedule,
    /// Where it is kept.
    pub storage: Storage,
    /// How much can be sold.
    pub on_hand: &'static str,
    /// What else is true of the stock: on order, quarantined or expiring.
    pub on_hand_note: &'static str,
    /// The nearest expiry among the sellable batches.
    pub nearest_expiry: &'static str,
    /// The batch that expiry belongs to.
    pub batch: &'static str,
    /// What it sells for.
    pub price: &'static str,
    /// Whether an insurer pays for it.
    pub on_formulary: bool,
}

/// What the board's filters count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Filter {
    /// The label, count included as the board writes it.
    pub label: &'static str,
    /// How many medicines it matches.
    pub count: u32,
    /// Whether the board has it selected.
    pub selected: bool,
}

/// Everything the catalogue board draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Catalogue {
    /// How many medicines the catalogue holds in all.
    pub total: u32,
    /// The filters across the top, with the first one selected.
    pub filters: Vec<Filter>,
    /// How many medicines are kept cold.
    pub cold_chain: u32,
    /// The page of medicines, in catalogue order.
    pub rows: Vec<Medicine>,
    /// How many the board is showing out of all of them.
    pub showing: (u32, u32),
}

impl Catalogue {
    /// The board's catalogue at Mwenge.
    #[must_use]
    pub fn story() -> Self {
        Self {
            total: 412,
            filters: vec![
                ("All \u{b7} 412", 412, true),
                ("Prescription-only \u{b7} 268", 268, false),
                ("Pharmacy medicine \u{b7} 61", 61, false),
                ("General sale \u{b7} 77", 77, false),
                ("Controlled \u{b7} 6", 6, false),
            ]
            .into_iter()
            .map(|(label, count, selected)| Filter {
                label,
                count,
                selected,
            })
            .collect(),
            cold_chain: 9,
            rows: vec![
                medicine(
                    "Amoxicillin",
                    "500mg capsules \u{b7} [Brand]",
                    "box of 100",
                    Schedule::PrescriptionOnly,
                    Storage::Room,
                    "1,231 caps",
                    "+1,000 on UZ-7781",
                    "03/2027",
                    "AMX-2409",
                    "200 / cap",
                    true,
                ),
                medicine(
                    "Amoxicillin",
                    "250mg/5ml powder for oral suspension, 100 ml",
                    "bottle",
                    Schedule::PrescriptionOnly,
                    Storage::Room,
                    "0 to sell",
                    "14 quarantined \u{b7} recall",
                    "19 Oct 2026",
                    "AMS-2404",
                    "5,500",
                    true,
                ),
                medicine(
                    "Paracetamol",
                    "500mg tablets \u{b7} [Brand]",
                    "tin of 1000",
                    Schedule::GeneralSale,
                    Storage::Room,
                    "4,380 tabs",
                    "",
                    "11/2027",
                    "PCM-2502",
                    "50 / tab",
                    true,
                ),
                medicine(
                    "Oral rehydration salts",
                    "sachet for 1 litre",
                    "box of 50",
                    Schedule::GeneralSale,
                    Storage::Room,
                    "245 sachets",
                    "",
                    "07/2028",
                    "ORS-2507",
                    "500",
                    true,
                ),
                medicine(
                    "Human insulin",
                    "100 IU/ml, 10 ml vial",
                    "vial",
                    Schedule::PrescriptionOnly,
                    Storage::ColdChain,
                    "4 vials",
                    "+10 on UZ-7781",
                    "01/2028",
                    "INS-2507",
                    "26,000",
                    true,
                ),
                medicine(
                    "Metformin",
                    "500mg tablets \u{b7} [Brand]",
                    "box of 100",
                    Schedule::PrescriptionOnly,
                    Storage::Room,
                    "1,500 tabs",
                    "9 packs near expiry",
                    "30 Oct 2026",
                    "MTF-2405",
                    "120 / tab",
                    true,
                ),
                medicine(
                    "Amlodipine",
                    "5mg tablets \u{b7} [Brand]",
                    "box of 100",
                    Schedule::PrescriptionOnly,
                    Storage::Room,
                    "820 tabs",
                    "",
                    "06/2027",
                    "AML-2506",
                    "150 / tab",
                    true,
                ),
                medicine(
                    "Metronidazole",
                    "400mg tablets \u{b7} [Brand]",
                    "box of 100",
                    Schedule::PrescriptionOnly,
                    Storage::Room,
                    "600 tabs",
                    "",
                    "09/2027",
                    "MTZ-2503",
                    "100 / tab",
                    true,
                ),
                medicine(
                    "Tramadol",
                    "50mg capsules \u{b7} locked cabinet",
                    "box of 100",
                    Schedule::Controlled,
                    Storage::Room,
                    "80 caps",
                    "counted 08:15",
                    "01/2027",
                    "TRM-2411",
                    "600 / cap",
                    true,
                ),
                medicine(
                    "Cetirizine",
                    "10mg tablets \u{b7} [Brand]",
                    "box of 100",
                    Schedule::PharmacyMedicine,
                    Storage::Room,
                    "22 packs",
                    "sell first",
                    "12 Dec 2026",
                    "CTZ-2502",
                    "1,800 / pack",
                    false,
                ),
                medicine(
                    "Ascorbic acid (Vitamin C)",
                    "500mg tablets",
                    "pack of 30",
                    Schedule::GeneralSale,
                    Storage::Room,
                    "0 to sell",
                    "3 packs expired",
                    "1 Oct 2026",
                    "VTC-2401",
                    "2,700",
                    false,
                ),
                medicine(
                    "Cough syrup",
                    "100 ml \u{b7} [Brand]",
                    "bottle",
                    Schedule::PharmacyMedicine,
                    Storage::Room,
                    "0 to sell",
                    "6 expired \u{b7} disposal",
                    "28 Sep 2026",
                    "CSY-2310",
                    "3,500",
                    false,
                ),
            ],
            showing: (12, 412),
        }
    }

    /// How many medicines cannot be sold right now, which is what a pharmacist
    /// opens this screen to find out.
    #[must_use]
    pub fn blocked(&self) -> usize {
        self.rows
            .iter()
            .filter(|medicine| medicine.on_hand.starts_with("0 "))
            .count()
    }

    /// How many are kept cold, counted from the rows the board shows.
    #[must_use]
    pub fn cold_in_rows(&self) -> usize {
        self.rows
            .iter()
            .filter(|medicine| medicine.storage == Storage::ColdChain)
            .count()
    }
}

/// Build one row; the catalogue is a long list of the same shape.
#[allow(clippy::too_many_arguments)]
const fn medicine(
    generic: &'static str,
    detail: &'static str,
    pack: &'static str,
    schedule: Schedule,
    storage: Storage,
    on_hand: &'static str,
    on_hand_note: &'static str,
    nearest_expiry: &'static str,
    batch: &'static str,
    price: &'static str,
    on_formulary: bool,
) -> Medicine {
    Medicine {
        generic,
        detail,
        pack,
        schedule,
        storage,
        on_hand,
        on_hand_note,
        nearest_expiry,
        batch,
        price,
        on_formulary,
    }
}

/// The medicine the rail opens, as the board's edit panel describes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Editing {
    /// The medicine's full title.
    pub title: &'static str,
    /// Whether it is kept cold.
    pub cold_chain: bool,
    /// What receiving has to check because it is cold.
    pub cold_chain_rule: &'static str,
    /// How much shelf life a delivery must have left.
    pub minimum_shelf_life: &'static str,
    /// The minimum in months, as the board offers it.
    pub minimum_months: u32,
    /// The two brands a dispenser may substitute with.
    pub substitutes: &'static [(&'static str, &'static str)],
    /// The rule about switching between them.
    pub substitute_rule: &'static str,
    /// What it costs and what it sells for.
    pub margin: &'static str,
    /// What is on hand and what is on its way.
    pub stock: &'static str,
    /// The warning printed on its label.
    pub label_warning: &'static str,
}

impl Editing {
    /// The board's edit panel for human insulin.
    #[must_use]
    pub const fn story() -> Self {
        Self {
            title: "Human insulin 100 IU/ml, 10 ml vial",
            cold_chain: true,
            cold_chain_rule: "Receiving asks for a temperature logger reading",
            minimum_shelf_life: "12 months (pharmacy rule)",
            minimum_months: 12,
            substitutes: &[
                ("Human insulin 100 IU/ml \u{b7} [Brand B]", "same strength"),
                (
                    "Human insulin 100 IU/ml pen \u{b7} [Brand C]",
                    "different device",
                ),
            ],
            substitute_rule: "Switching insulin products needs the prescriber's agreement.",
            margin: "Cost 21,000 \u{b7} sells 26,000 per vial",
            stock: "4 on hand \u{b7} 10 on UZ-7781",
            label_warning: "Keep in a refrigerator 2\u{2013}8 \u{b0}C. Do not freeze.",
        }
    }
}

/// The catalogue's filters.
fn filters(catalogue: &Catalogue, mode: ThemeMode) -> Div {
    let labels: Vec<(&'static str, bool)> = catalogue
        .filters
        .iter()
        .map(|filter| (filter.label, filter.selected))
        .collect();
    board::filters_in(mode, &labels)
}

/// The catalogue table.
fn table(catalogue: &Catalogue) -> Div {
    let rows = catalogue.rows.iter().enumerate().map(|(index, medicine)| {
        board::link_to(("medicine-row", index), "/medicines")
            .sx(board::line_style())
            .child(board::cell_fixed_stack(
                board::dotted(&[medicine.generic, medicine.detail]),
                medicine.pack,
            ))
            .child(board::chip(
                medicine.schedule.label(),
                medicine.schedule.tone(),
            ))
            .child(board::cell_fixed(medicine.storage.label()))
            .child(board::cell_fixed_stack(
                medicine.on_hand,
                medicine.on_hand_note,
            ))
            .child(board::cell_fixed_stack(
                medicine.nearest_expiry,
                medicine.batch,
            ))
            .child(board::cell_number(medicine.price))
            .child(board::chip(
                if medicine.on_formulary { "Yes" } else { "No" },
                if medicine.on_formulary {
                    Tone::Success
                } else {
                    Tone::Neutral
                },
            ))
    });
    board::card(2.4)
        .child(board::card_head(
            format!(
                "Showing {} of {} \u{b7} prices per unit sold at the till",
                catalogue.showing.0, catalogue.showing.1
            ),
            "medicines are VAT exempt",
        ))
        .child(board::head(vec![
            board::cell_fixed("Medicine"),
            board::cell_fixed("Pack"),
            board::cell_fixed("Schedule"),
            board::cell_fixed("Storage"),
            board::cell_fixed("On hand"),
            board::cell_fixed("Nearest expiry"),
            board::cell_number("Price"),
            board::cell_fixed("Formulary"),
        ]))
        .children(rows.collect::<Vec<_>>())
        .child(board::footnote(
            "registration numbers come from the TMDA; a medicine with no batch on hand cannot be sold",
        ))
}

/// The rail: the medicine being edited.
fn rail(editing: &Editing, mode: ThemeMode) -> Div {
    board::card(1.)
        .child(board::section_label("Edit medicine"))
        .child(board::card_title(editing.title))
        .when(editing.cold_chain, |rail| {
            rail.child(board::chip("Cold chain", Tone::Info))
                .child(board::meta(editing.cold_chain_rule))
        })
        .child(board::section_label("Minimum shelf life on delivery"))
        .child(board::option(editing.minimum_shelf_life))
        .child(board::meta(format!("{} months", editing.minimum_months)))
        .child(board::section_label("Substitutes and generic equivalents"))
        .child(board::list(
            editing
                .substitutes
                .iter()
                .map(|(name, kind)| board::cell_stack(*name, *kind).into_any_element())
                .collect::<Vec<_>>(),
        ))
        .child(board::meta(editing.substitute_rule))
        .child(board::panel(vec![
            board::key_value("Margin:", editing.margin).into_any_element(),
            board::key_value("Stock:", editing.stock).into_any_element(),
        ]))
        .child(board::section_label("Label warnings"))
        .child(board::alert(
            "Keep cold",
            editing.label_warning,
            Tone::Warning,
            mode,
        ))
        .child(board::actions(vec![
            Button::new("cancel-medicine")
                .label("Cancel")
                .into_any_element(),
            Button::new("save-medicine")
                .label("Save changes")
                .variant(ButtonVariant::Primary)
                .into_any_element(),
        ]))
}

/// The catalogue board.
#[component]
pub fn MedicineCatalogue(
    catalogue: Catalogue,
    editing: Editing,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let mode = board::mode(cx);
    div()
        .sx((board::root(), &sx))
        .child(div().sx(board::row()).child(board::filters_in(
            mode,
            &[
                ("All storage", false),
                ("Room temperature", false),
                ("Cold chain 2\u{2013}8 \u{b0}C", true),
            ],
        )))
        .child(board::stat_row(vec![
            board::stat(
                "In the catalogue",
                catalogue.total.to_string(),
                "medicines the pharmacy may sell",
                None,
                mode,
            ),
            board::stat(
                "Cold chain",
                catalogue.cold_chain.to_string(),
                "kept between 2 and 8 degrees",
                Some(Tone::Info),
                mode,
            ),
            board::stat(
                "Cannot be sold",
                catalogue.blocked().to_string(),
                "quarantined, recalled or expired",
                Some(Tone::Danger),
                mode,
            ),
        ]))
        .child(filters(&catalogue, mode))
        .child(
            div()
                .sx(board::row())
                .child(table(&catalogue))
                .child(rail(&editing, mode)),
        )
}

/// The catalogue with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    MedicineCatalogue::new(Catalogue::story(), Editing::story())
}

#[cfg(test)]
mod tests {
    use super::{Catalogue, Editing, Schedule, Storage, view};
    use rok_ui::prelude::*;

    #[test]
    fn the_filters_add_up_to_the_whole_catalogue() {
        let catalogue = Catalogue::story();
        let filtered: u32 = catalogue
            .filters
            .iter()
            .skip(1)
            .map(|filter| filter.count)
            .sum();
        assert_eq!(filtered, catalogue.total);
        assert_eq!(catalogue.filters[0].label, "All \u{b7} 412");
    }

    #[test]
    fn only_the_insulin_row_is_kept_cold() {
        let catalogue = Catalogue::story();
        assert_eq!(catalogue.cold_in_rows(), 1);
        assert_eq!(
            catalogue
                .rows
                .iter()
                .filter(|medicine| medicine.storage == Storage::ColdChain)
                .map(|medicine| medicine.generic)
                .collect::<Vec<_>>(),
            ["Human insulin"]
        );
    }

    #[test]
    fn the_rows_with_nothing_to_sell_are_the_ones_the_board_blocks() {
        let catalogue = Catalogue::story();
        assert_eq!(catalogue.blocked(), 3);
        assert_eq!(catalogue.rows[1].nearest_expiry, "19 Oct 2026");
    }

    #[test]
    fn the_schedule_wording_is_the_boards() {
        assert_eq!(Schedule::PrescriptionOnly.label(), "Prescription-only");
        assert_eq!(Schedule::Controlled.label(), "Controlled");
        assert_eq!(Storage::ColdChain.label(), "Cold 2\u{2013}8 \u{b0}C");
    }

    #[test]
    fn the_edited_insulin_keeps_the_rules_the_board_states() {
        let editing = Editing::story();
        assert!(editing.cold_chain);
        assert_eq!(editing.minimum_months, 12);
        assert_eq!(editing.substitutes.len(), 2);
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            view()
        }
    }

    #[gpui::test]
    fn draws_the_catalogue(cx: &mut gpui::TestAppContext) {
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
