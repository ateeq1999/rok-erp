//! The pharmacy dashboard: today's sales, prescriptions, expiring stock and
//! queried claims, what needs the pharmacist now, and how the day is going.
//!
//! Board: `PharmacyDashboard`. Phase 13 computes [`DashboardFigures`] from the
//! database; until then [`DashboardFigures::story`] holds the board's own
//! figures, and the totals the board derives from them are derived here too.

use rok_pos_domain::Money;
use rok_pos_shell::theme::{chart_in_progress, chart_series};
use rok_pos_shell::{Tone, format_money, tone};
use rok_ui::prelude::*;
use rok_ui::router::navigate;

/// One of the four tiles across the top.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    /// What the tile counts.
    pub label: &'static str,
    /// The figure, as drawn.
    pub value: String,
    /// The line under the figure.
    pub note: String,
    /// The figure's colour: none for the theme's text.
    pub tone: Option<Tone>,
    /// The screen the tile opens.
    pub href: &'static str,
}

/// One row of "Needs you now".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Task {
    /// The chip on the left: "Recall", "Check".
    pub kind: &'static str,
    /// How urgent the chip looks.
    pub tone: Tone,
    /// What it is about.
    pub title: &'static str,
    /// What to do about it.
    pub detail: &'static str,
    /// Where the row goes.
    pub href: &'static str,
    /// The words on the right: "Open recall".
    pub action: &'static str,
}

/// What one hour sold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HourSales {
    /// The hour it starts, as a board writes it: `"08"`.
    pub hour: &'static str,
    /// What it sold.
    pub sales: Money,
}

/// One row of "Top medicines today".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MedicineSold {
    /// The medicine.
    pub name: &'static str,
    /// How much of it, with its unit.
    pub quantity: &'static str,
    /// What it sold for.
    pub sales: Money,
}

/// How much was paid one way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Payment {
    /// The way it was paid.
    pub label: &'static str,
    /// How much.
    pub amount: Money,
}

/// What the other branch did today, for the comparison table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OtherBranch {
    /// The branch's short name.
    pub name: &'static str,
    /// What it sold.
    pub sales: Money,
    /// Prescriptions it dispensed.
    pub prescriptions_dispensed: u32,
    /// Its insurance share, in percent.
    pub insurance_share_percent: i64,
    /// Its average basket.
    pub average_basket: Money,
    /// Prescriptions still waiting there.
    pub prescriptions_waiting: u32,
}

/// Everything the dashboard draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DashboardFigures {
    /// This branch's short name.
    pub branch: &'static str,
    /// Sales by hour, oldest first; the last hour is still in progress.
    pub hours: Vec<HourSales>,
    /// Sales by way of payment, insurance first.
    pub payments: Vec<Payment>,
    /// Prescriptions dispensed today.
    pub prescriptions_dispensed: u32,
    /// Prescriptions waiting.
    pub prescriptions_waiting: u32,
    /// How long the oldest one has waited, in minutes.
    pub oldest_waiting_minutes: u32,
    /// What expires within 30 days, at cost.
    pub expiring_within_30_days: Money,
    /// How many batches that is.
    pub expiring_batches: u32,
    /// Expired batches blocked at the till.
    pub expired_batches_blocked: u32,
    /// Claims the insurer queried.
    pub claims_queried: u32,
    /// The insurer and batch those claims are in.
    pub claims_queried_in: &'static str,
    /// The average basket here.
    pub average_basket: Money,
    /// What needs someone now, most urgent first.
    pub tasks: Vec<Task>,
    /// The best sellers today.
    pub top_medicines: Vec<MedicineSold>,
    /// The other branch, for comparison.
    pub other_branch: OtherBranch,
}

impl DashboardFigures {
    /// The board's figures for Mwenge branch at 15:30.
    #[must_use]
    pub fn story() -> Self {
        let tzs = Money::from_shillings;
        Self {
            branch: "Mwenge",
            hours: [
                ("08", 96_500),
                ("09", 184_200),
                ("10", 312_400),
                ("11", 268_900),
                ("12", 221_300),
                ("13", 247_600),
                ("14", 289_800),
                ("15", 225_500),
            ]
            .into_iter()
            .map(|(hour, sales)| HourSales {
                hour,
                sales: tzs(sales),
            })
            .collect(),
            payments: vec![
                Payment {
                    label: "Insurance",
                    amount: tzs(701_600),
                },
                Payment {
                    label: "Mobile money",
                    amount: tzs(632_200),
                },
                Payment {
                    label: "Cash",
                    amount: tzs(512_400),
                },
            ],
            prescriptions_dispensed: 23,
            prescriptions_waiting: 6,
            oldest_waiting_minutes: 34,
            expiring_within_30_days: tzs(186_400),
            expiring_batches: 5,
            expired_batches_blocked: 2,
            claims_queried: 11,
            claims_queried_in: "National health insurance \u{b7} September batch",
            average_basket: tzs(18_600),
            tasks: vec![
                Task {
                    kind: "Recall",
                    tone: Tone::Danger,
                    title: "Recall RC-0047 \u{b7} Amoxicillin 250mg/5ml, batch AMS-2404",
                    detail: "14 bottles quarantined \u{b7} 2 of 5 patients still to reach",
                    href: "/recalls/RC-0047",
                    action: "Open recall",
                },
                Task {
                    kind: "Check",
                    tone: Tone::Warning,
                    title: "RX-2214 \u{b7} Mzee Salim R. \u{b7} interaction check",
                    detail: "Metronidazole with warfarin may increase bleeding risk. Contact the prescriber.",
                    href: "/prescriptions/RX-2214/check",
                    action: "Review",
                },
                Task {
                    kind: "Delivery",
                    tone: Tone::Info,
                    title: "Delivery UZ-7781 arrived from Uzima Pharmaceuticals",
                    detail: "Arrived 11:20 \u{b7} 6 lines \u{b7} insulin in cold box \u{b7} count and check batches",
                    href: "/receive/UZ-7781",
                    action: "Receive",
                },
                Task {
                    kind: "Licence",
                    tone: Tone::Warning,
                    title: "Premises licence renewal due in 57 days",
                    detail: "Pharmacy Council \u{b7} expires 30 Nov 2026 \u{b7} submit by 28 Nov",
                    href: "/licences",
                    action: "Start renewal",
                },
                Task {
                    kind: "Done",
                    tone: Tone::Success,
                    title: "Controlled count done 08:15",
                    detail: "Tramadol 50mg and 5 other lines counted by Grace N. \u{b7} no difference",
                    href: "/controlled-register",
                    action: "View register",
                },
            ],
            top_medicines: [
                ("Paracetamol 500mg tablets", "640 tabs", 32_000),
                ("Amoxicillin 500mg capsules", "231 caps", 46_200),
                ("Metformin 500mg tablets", "420 tabs", 50_400),
                ("Amlodipine 5mg tablets", "300 tabs", 45_000),
                ("Oral rehydration salts", "62 sachets", 31_000),
            ]
            .into_iter()
            .map(|(name, quantity, sales)| MedicineSold {
                name,
                quantity,
                sales: tzs(sales),
            })
            .collect(),
            other_branch: OtherBranch {
                name: "Tegeta",
                sales: tzs(1_212_800),
                prescriptions_dispensed: 15,
                insurance_share_percent: 44,
                average_basket: tzs(16_900),
                prescriptions_waiting: 2,
            },
        }
    }

    /// Everything sold today, however it was paid.
    #[must_use]
    pub fn sales_total(&self) -> Money {
        Money::total(self.payments.iter().map(|payment| payment.amount)).unwrap_or(Money::zero())
    }

    /// `amount` as a whole percentage of today's sales, rounded half up.
    #[must_use]
    pub fn share_percent(&self, amount: Money) -> i64 {
        percent(
            amount.round_to_shillings(),
            self.sales_total().round_to_shillings(),
        )
    }

    /// How much of today's sales the insurer owes, rather than the patient paid.
    #[must_use]
    pub fn insurance(&self) -> Money {
        self.payments
            .iter()
            .find(|payment| payment.label == "Insurance")
            .map_or(Money::zero(), |payment| payment.amount)
    }

    /// The four tiles, as the board draws them.
    #[must_use]
    pub fn tiles(&self) -> [Tile; 4] {
        let insurance = self.insurance();
        [
            Tile {
                label: "Sales today",
                value: format_money(self.sales_total()),
                note: format!(
                    "Insurance share {}% \u{b7} {}",
                    self.share_percent(insurance),
                    format_money(insurance)
                ),
                tone: None,
                href: "/reports",
            },
            Tile {
                label: "Prescriptions",
                value: format!("{} dispensed", self.prescriptions_dispensed),
                note: format!(
                    "{} waiting \u{b7} oldest {} min",
                    self.prescriptions_waiting, self.oldest_waiting_minutes
                ),
                tone: None,
                href: "/prescriptions",
            },
            Tile {
                label: "Expiring in 30 days",
                value: format_money(self.expiring_within_30_days),
                note: format!(
                    "{} batches at cost \u{b7} {} expired batches blocked",
                    self.expiring_batches, self.expired_batches_blocked
                ),
                tone: Some(Tone::Warning),
                href: "/batches",
            },
            Tile {
                label: "Claims queried",
                value: self.claims_queried.to_string(),
                note: self.claims_queried_in.to_string(),
                tone: Some(Tone::Danger),
                href: "/claims",
            },
        ]
    }

    /// Each hour's bar height, in pixels, with the busiest hour at `tallest`.
    #[must_use]
    pub fn bar_heights(&self, tallest: f32) -> Vec<f32> {
        let busiest = self
            .hours
            .iter()
            .map(|hour| hour.sales.round_to_shillings())
            .max()
            .unwrap_or(0);
        self.hours
            .iter()
            .map(|hour| {
                if busiest == 0 {
                    0.
                } else {
                    // Shillings in a day fit an f64 exactly; the bar is a picture.
                    #[allow(clippy::cast_precision_loss)]
                    let ratio = hour.sales.round_to_shillings() as f64 / busiest as f64;
                    #[allow(clippy::cast_possible_truncation)]
                    let height = (ratio * f64::from(tallest)).round() as f32;
                    height
                }
            })
            .collect()
    }

    /// The comparison table's rows: measure, this branch, the other branch.
    #[must_use]
    pub fn branch_rows(&self) -> [(&'static str, String, String); 5] {
        let other = &self.other_branch;
        [
            (
                "Sales",
                format_money(self.sales_total()),
                format_money(other.sales),
            ),
            (
                "Prescriptions dispensed",
                self.prescriptions_dispensed.to_string(),
                other.prescriptions_dispensed.to_string(),
            ),
            (
                "Insurance share",
                format!("{}%", self.share_percent(self.insurance())),
                format!("{}%", other.insurance_share_percent),
            ),
            (
                "Average basket",
                format_money(self.average_basket),
                format_money(other.average_basket),
            ),
            (
                "Prescriptions waiting",
                self.prescriptions_waiting.to_string(),
                other.prescriptions_waiting.to_string(),
            ),
        ]
    }
}

/// `part` as a whole percentage of `whole`, rounded half up; zero of nothing.
fn percent(part: i64, whole: i64) -> i64 {
    if whole == 0 {
        0
    } else {
        (part * 200 + whole) / (2 * whole)
    }
}

/// How tall the busiest hour's bar is, in pixels.
const TALLEST_BAR_PX: f32 = 100.;

styles! {
    DASHBOARD = {
        root: { display: flex, flex_direction: column, gap: 4 },
        row: { display: flex, flex_direction: row, gap: 4, align: stretch },
        tiles: { display: flex, flex_direction: row, gap: 3 },
        tile: {
            grow: 1,
            basis: 0,
            min_width: 0,
            display: flex,
            flex_direction: column,
            gap: 1,
            padding: 4,
            background: card,
            border: 1,
            border_color: border,
            cursor: pointer,
            hover: { border_color: input },
        },
        tile_label: { text: {13.}, color: muted_foreground },
        tile_value: { font_family: mono, text: {26.}, font: bold },
        small: { text: {12.}, color: muted_foreground },
        meta: { text: {13.}, color: muted_foreground },
        card: {
            basis: 0,
            min_width: 0,
            display: flex,
            flex_direction: column,
            gap: 2,
            padding: 4.5,
            background: card,
            border: 1,
            border_color: border,
        },
        card_head: { display: flex, flex_direction: row, justify: between, align: center },
        card_title: { text: {16.}, font: semibold },
        task: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 3,
            min_height: 13,
            padding_y: 2,
            padding_x: 2.5,
            border_top: 1,
            border_color: muted,
            cursor: pointer,
            hover: { background: background },
        },
        task_chip: {
            width: 23,
            shrink: 0,
            padding_y: {px(3.)},
            text: {12.},
            font: semibold,
            text_align: center,
        },
        task_text: { display: flex, flex_direction: column, grow: 1, min_width: 0 },
        task_title: { font: semibold },
        task_action: { shrink: 0, text: {13.}, font: semibold },
        bars: {
            display: flex,
            flex_direction: row,
            align: end,
            gap: 2.5,
            height: {px(150.)},
            padding_bottom: 1,
            border_bottom: 1,
            border_color: border,
        },
        bar_column: {
            grow: 1,
            basis: 0,
            display: flex,
            flex_direction: column,
            align: center,
            justify: end,
            gap: 1,
            height: full,
        },
        bar_figure: { font_family: mono, text: {11.}, color: muted_foreground },
        hour_labels: { display: flex, flex_direction: row, gap: 2.5 },
        hour_label: {
            grow: 1,
            basis: 0,
            text_align: center,
            font_family: mono,
            text: {12.},
            color: muted_foreground,
        },
        table_head: { display: flex, flex_direction: row, padding_y: 1, text: {12.}, font: semibold, color: muted_foreground },
        table_row: { display: flex, flex_direction: row, padding_y: {px(7.)}, border_top: 1, border_color: muted },
        first_column: { grow: 1, min_width: 0, truncate: true },
        number_column: { width: 20, shrink: 0, text_align: right, font_family: mono },
        number_head: { width: 20, shrink: 0, text_align: right },
        split: { display: flex, flex_direction: row, height: 3.5 },
        legend: { display: flex, flex_direction: row, align: center, gap: 2.5 },
        dot: { size: 2.5, shrink: 0, radius: full },
        legend_amount: { width: 21, text_align: right, font_family: mono, font: semibold },
        link: { text: {13.}, font: semibold, color: primary, cursor: pointer },
    }
}

/// A box that opens `href` on a click, Enter or Space.
fn link_to(id: impl Into<ElementId>, href: &'static str) -> gpui::Stateful<Div> {
    div()
        .id(id)
        .tab_index(0)
        .on_click(move |_, _, cx| navigate(href, cx))
        .on_key_down(move |event, _, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                navigate(href, cx);
            }
        })
}

fn card(grow: f32) -> Div {
    div().sx(sx![&DASHBOARD.card, style! { grow: {grow} }])
}

fn card_head(title: &'static str, meta: impl Into<SharedString>) -> Div {
    div()
        .sx(&DASHBOARD.card_head)
        .child(div().sx(&DASHBOARD.card_title).child(title))
        .child(div().sx(&DASHBOARD.meta).child(meta.into()))
}

fn tile(index: usize, tile: Tile, mode: ThemeMode) -> impl IntoElement {
    let color = tile.tone.map(|tone| tone::colors(tone, mode).foreground);
    link_to(("dashboard-tile", index), tile.href)
        .sx(&DASHBOARD.tile)
        .child(div().sx(&DASHBOARD.tile_label).child(tile.label))
        .child(
            div()
                .sx(&DASHBOARD.tile_value)
                .when_some(color, gpui::Styled::text_color)
                .child(tile.value),
        )
        .child(div().sx(&DASHBOARD.small).child(tile.note))
}

fn task_row(index: usize, task: Task, mode: ThemeMode, colors: &ThemeColors) -> impl IntoElement {
    let palette = tone::colors(task.tone, mode);
    let action_color = if task.tone == Tone::Success {
        colors.muted_foreground
    } else {
        colors.primary
    };
    link_to(("dashboard-task", index), task.href)
        .sx(&DASHBOARD.task)
        .child(
            div()
                .sx(sx![
                    &DASHBOARD.task_chip,
                    style! { background: {palette.background}, color: {palette.foreground} },
                ])
                .child(task.kind),
        )
        .child(
            div()
                .sx(&DASHBOARD.task_text)
                .child(div().sx(&DASHBOARD.task_title).child(task.title))
                .child(div().sx(&DASHBOARD.meta).child(task.detail)),
        )
        .child(
            div()
                .sx(&DASHBOARD.task_action)
                .text_color(action_color)
                .child(task.action),
        )
}

fn needs_you_now(figures: &DashboardFigures, mode: ThemeMode, colors: &ThemeColors) -> Div {
    let open = figures
        .tasks
        .iter()
        .filter(|task| task.tone != Tone::Success)
        .count();
    let done = figures.tasks.len() - open;
    card(1.25)
        .child(card_head(
            "Needs you now",
            format!("{open} open \u{b7} {done} done today"),
        ))
        .children(
            figures
                .tasks
                .iter()
                .enumerate()
                .map(|(index, task)| task_row(index, *task, mode, colors)),
        )
}

fn sales_by_hour(figures: &DashboardFigures, mode: ThemeMode, colors: &ThemeColors) -> Div {
    let heights = figures.bar_heights(TALLEST_BAR_PX);
    let last = figures.hours.len().saturating_sub(1);
    let bars = figures
        .hours
        .iter()
        .zip(heights)
        .enumerate()
        .map(|(index, (hour, height))| {
            let fill = if index == last {
                chart_in_progress(mode)
            } else {
                colors.primary
            };
            div()
                .sx(&DASHBOARD.bar_column)
                .child(div().sx(&DASHBOARD.bar_figure).child(format!(
                    "{}k",
                    (hour.sales.round_to_shillings() + 500) / 1000
                )))
                .child(div().w_full().h(px(height)).bg(fill))
        });
    card(1.)
        .child(card_head("Sales by hour", "TZS \u{b7} today so far"))
        .child(div().sx(&DASHBOARD.bars).children(bars))
        .child(
            div().sx(&DASHBOARD.hour_labels).children(
                figures
                    .hours
                    .iter()
                    .map(|hour| div().sx(&DASHBOARD.hour_label).child(hour.hour)),
            ),
        )
        .child(
            div()
                .sx(&DASHBOARD.small)
                .child("15:00 bar is the hour in progress (lighter)."),
        )
}

fn table_head(first: &'static str, second: &'static str, third: &'static str) -> Div {
    div()
        .sx(&DASHBOARD.table_head)
        .child(div().sx(&DASHBOARD.first_column).child(first))
        .child(div().sx(&DASHBOARD.number_head).child(second))
        .child(div().sx(&DASHBOARD.number_head).child(third))
}

/// A right-aligned figure in the numbers font.
fn number_cell(text: String) -> Div {
    div().sx(&DASHBOARD.number_column).child(text)
}

fn table_row(first: impl Into<SharedString>, second: Div, third: Div) -> Div {
    div()
        .sx(&DASHBOARD.table_row)
        .child(div().sx(&DASHBOARD.first_column).child(first.into()))
        .child(second)
        .child(third)
}

fn top_medicines(figures: &DashboardFigures, colors: &ThemeColors) -> Div {
    card(1.1)
        .child(div().sx(&DASHBOARD.card_title).child("Top medicines today"))
        .child(table_head("Medicine", "Sold", "Sales"))
        .children(figures.top_medicines.iter().map(|medicine| {
            table_row(
                medicine.name,
                number_cell(medicine.quantity.to_string()).text_color(colors.muted_foreground),
                number_cell(format_money(medicine.sales)).font_weight(FontWeight::SEMIBOLD),
            )
        }))
}

fn how_patients_paid(figures: &DashboardFigures, mode: ThemeMode) -> Div {
    let series = chart_series(mode);
    let total = figures.sales_total().round_to_shillings();
    let segments = figures.payments.iter().zip(series).map(|(payment, color)| {
        // A share of one day's sales; the segment is a picture.
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let fraction = if total == 0 {
            0.
        } else {
            (payment.amount.round_to_shillings() as f64 / total as f64) as f32
        };
        div().h_full().w(relative(fraction)).bg(color)
    });
    card(1.)
        .child(div().sx(&DASHBOARD.card_title).child("How patients paid"))
        .child(div().sx(&DASHBOARD.split).children(segments))
        .children(figures.payments.iter().zip(series).map(|(payment, color)| {
            div()
                .sx(&DASHBOARD.legend)
                .child(div().sx(&DASHBOARD.dot).bg(color))
                .child(div().flex_grow().child(payment.label))
                .child(
                    div()
                        .sx(&DASHBOARD.meta)
                        .child(format!("{}%", figures.share_percent(payment.amount))),
                )
                .child(
                    div()
                        .sx(&DASHBOARD.legend_amount)
                        .child(format_money(payment.amount)),
                )
        }))
        .child(
            div()
                .sx(&DASHBOARD.small)
                .child("Insurance part is billed in the October claim batch, not cash in hand."),
        )
}

fn branches_today(figures: &DashboardFigures) -> Div {
    card(1.)
        .child(div().sx(&DASHBOARD.card_title).child("Branches today"))
        .child(table_head(
            "Measure",
            figures.branch,
            figures.other_branch.name,
        ))
        .children(
            figures
                .branch_rows()
                .into_iter()
                .map(|(measure, here, there)| {
                    table_row(
                        measure,
                        number_cell(here).font_weight(FontWeight::SEMIBOLD),
                        number_cell(there),
                    )
                }),
        )
        .child(
            link_to("dashboard-reports", "/reports")
                .sx(&DASHBOARD.link)
                .child("Open full reports"),
        )
}

/// The dashboard for `figures`.
#[component]
pub fn Dashboard(figures: DashboardFigures, cx: &mut Cx) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let mode = cx.theme().mode;
    let tiles = figures.tiles();
    div()
        .sx(&DASHBOARD.root)
        .child(
            div().sx(&DASHBOARD.tiles).children(
                tiles
                    .into_iter()
                    .enumerate()
                    .map(|(index, figure)| tile(index, figure, mode)),
            ),
        )
        .child(
            div()
                .sx(&DASHBOARD.row)
                .child(needs_you_now(&figures, mode, &colors))
                .child(sales_by_hour(&figures, mode, &colors)),
        )
        .child(
            div()
                .sx(&DASHBOARD.row)
                .child(top_medicines(&figures, &colors))
                .child(how_patients_paid(&figures, mode))
                .child(branches_today(&figures)),
        )
}

/// The dashboard with the board's figures.
#[must_use]
pub fn view() -> impl IntoElement {
    Dashboard::new(DashboardFigures::story())
}

#[cfg(test)]
mod tests {
    use super::{Dashboard, DashboardFigures, percent};
    use rok_pos_domain::Money;
    use rok_ui::prelude::*;

    #[test]
    fn the_tiles_say_what_the_board_says() {
        let tiles = DashboardFigures::story().tiles();
        let drawn: Vec<(&str, &str, &str)> = tiles
            .iter()
            .map(|tile| (tile.label, tile.value.as_str(), tile.note.as_str()))
            .collect();
        assert_eq!(
            drawn,
            [
                (
                    "Sales today",
                    "1,846,200",
                    "Insurance share 38% \u{b7} 701,600"
                ),
                (
                    "Prescriptions",
                    "23 dispensed",
                    "6 waiting \u{b7} oldest 34 min"
                ),
                (
                    "Expiring in 30 days",
                    "186,400",
                    "5 batches at cost \u{b7} 2 expired batches blocked"
                ),
                (
                    "Claims queried",
                    "11",
                    "National health insurance \u{b7} September batch"
                ),
            ]
        );
    }

    #[test]
    fn the_payment_split_is_the_boards() {
        let figures = DashboardFigures::story();
        assert_eq!(figures.sales_total(), Money::from_shillings(1_846_200));
        let shares: Vec<i64> = figures
            .payments
            .iter()
            .map(|payment| figures.share_percent(payment.amount))
            .collect();
        assert_eq!(shares, [38, 34, 28]);
    }

    #[test]
    fn the_busiest_hour_has_the_tallest_bar() {
        let heights = DashboardFigures::story().bar_heights(100.);
        assert_eq!(heights.len(), 8);
        assert!(
            (heights[2] - 100.).abs() < f32::EPSILON,
            "10:00 was the busiest hour"
        );
        assert!(
            (heights[0] - 31.).abs() < f32::EPSILON,
            "08:00 sold 96,500 of 312,400"
        );
    }

    #[test]
    fn the_branch_table_compares_both_branches() {
        let rows = DashboardFigures::story().branch_rows();
        assert_eq!(rows[0], ("Sales", "1,846,200".into(), "1,212,800".into()));
        assert_eq!(rows[2], ("Insurance share", "38%".into(), "44%".into()));
    }

    #[test]
    fn percentages_round_half_up_and_survive_nothing() {
        assert_eq!(percent(1, 8), 13);
        assert_eq!(percent(1, 3), 33);
        assert_eq!(percent(5, 0), 0);
    }

    #[test]
    fn the_task_counts_are_the_boards() {
        let figures = DashboardFigures::story();
        assert_eq!(figures.tasks.len(), 5);
        assert_eq!(figures.tasks[0].href, "/recalls/RC-0047");
    }

    struct Screen;

    impl Render for Screen {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Dashboard::new(DashboardFigures::story())
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
