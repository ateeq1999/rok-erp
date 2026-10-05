//! The arithmetic over a day's figures.
//!
//! Every rule here is a free function over the entities: no window, no context,
//! no formatting. A screen that needs a figure calls one of these, so the
//! number on the board and the number in the tests are the same number.

use rok_pos_domain::Money;

use super::entities::{Dashboard, DashboardTask, MedicineSold, OtherBranch, Text, text};
use super::enums::{Destination, TaskSeverity};
use super::measures::MeasureValue;

/// The label the board writes the insurer's money under.
const INSURANCE_LABEL: &str = "Insurance";

/// Everything sold today, however it was paid.
#[must_use]
pub fn sales_total(dashboard: &Dashboard) -> Money {
    Money::total(dashboard.payments.iter().map(|payment| payment.amount)).unwrap_or(Money::zero())
}

/// How much of today's sales the insurer owes rather than the patient paid.
#[must_use]
pub fn insurance(dashboard: &Dashboard) -> Money {
    dashboard
        .payments
        .iter()
        .find(|payment| &*payment.label == INSURANCE_LABEL)
        .map_or(Money::zero(), |payment| payment.amount)
}

/// `part` as a whole percentage of `whole`, rounded half up; zero of nothing is
/// zero rather than a division by zero.
#[must_use]
pub fn percentage(part: i64, whole: i64) -> i64 {
    if whole == 0 {
        0
    } else {
        (part * 200 + whole) / (2 * whole)
    }
}

/// `amount` as a whole percentage of today's sales.
#[must_use]
pub fn payment_share(dashboard: &Dashboard, amount: Money) -> i64 {
    percentage(
        amount.round_to_shillings(),
        sales_total(dashboard).round_to_shillings(),
    )
}

/// Each hour's bar height in pixels, with the busiest hour at `tallest`.
///
/// A day with no hours draws no bars, and a day whose hours sold nothing draws
/// them flat rather than dividing by zero.
#[must_use]
pub fn bar_heights(dashboard: &Dashboard, tallest: f32) -> Vec<f32> {
    let busiest = dashboard
        .hours
        .iter()
        .map(|hour| hour.sales.round_to_shillings())
        .max()
        .unwrap_or(0);
    dashboard
        .hours
        .iter()
        .map(|hour| {
            if busiest == 0 {
                0.
            } else {
                // Shillings in one day fit an f64 exactly; the bar is a picture
                // of a figure, not the figure itself.
                #[allow(clippy::cast_precision_loss)]
                let ratio = hour.sales.round_to_shillings() as f64 / busiest as f64;
                #[allow(clippy::cast_possible_truncation)]
                {
                    (ratio * f64::from(tallest)).round() as f32
                }
            }
        })
        .collect()
}

/// The work still to do today.
#[must_use]
pub fn open_tasks(dashboard: &Dashboard) -> Vec<&DashboardTask> {
    dashboard
        .tasks
        .iter()
        .filter(|task| !task.is_done())
        .collect()
}

/// The work already finished today.
#[must_use]
pub fn done_tasks(dashboard: &Dashboard) -> usize {
    dashboard.tasks.len() - open_tasks(dashboard).len()
}

/// One tile across the top of the board: what it counts, as a figure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    /// What the tile counts.
    pub label: Text,
    /// The figure itself, which the screen knows how to draw.
    pub measure: MeasureValue,
    /// The line under the figure, in words.
    pub note: Text,
    /// A second figure on that line, where the board shows one.
    pub note_figure: Option<MeasureValue>,
    /// How serious the figure is, or nothing for the theme's own text.
    pub severity: Option<TaskSeverity>,
    /// Where the tile goes.
    pub destination: Destination,
}

/// The four tiles, with their notes, as the board draws them.
#[must_use]
pub fn tiles(dashboard: &Dashboard) -> [Tile; 4] {
    let total = sales_total(dashboard);
    let insurer = insurance(dashboard);
    [
        Tile {
            label: text("Sales today"),
            measure: MeasureValue::Money(total),
            note: Text::from(format!(
                "Insurance share {}%",
                payment_share(dashboard, insurer)
            )),
            note_figure: Some(MeasureValue::Money(insurer)),
            severity: None,
            destination: Destination::Reports,
        },
        Tile {
            label: text("Prescriptions"),
            measure: MeasureValue::Count(dashboard.prescriptions_dispensed),
            note: Text::from(format!(
                "{} dispensed \u{b7} {} waiting \u{b7} oldest {} min",
                dashboard.prescriptions_dispensed,
                dashboard.prescriptions_waiting,
                dashboard.oldest_waiting_minutes
            )),
            note_figure: None,
            severity: None,
            destination: Destination::Prescriptions,
        },
        Tile {
            label: text("Expiring in 30 days"),
            measure: MeasureValue::Money(dashboard.expiring_within_30_days),
            note: Text::from(format!(
                "{} batches at cost \u{b7} {} expired batches blocked",
                dashboard.expiring_batches, dashboard.expired_batches_blocked
            )),
            note_figure: None,
            severity: Some(TaskSeverity::Warning),
            destination: Destination::Batches,
        },
        Tile {
            label: text("Claims queried"),
            measure: MeasureValue::Count(dashboard.claims_queried),
            note: dashboard.claims_queried_in.clone(),
            note_figure: None,
            severity: Some(TaskSeverity::Danger),
            destination: Destination::Claims,
        },
    ]
}

/// One measure of the branch comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BranchMeasure {
    /// What is being compared.
    pub label: Text,
    /// This branch's figure.
    pub here: MeasureValue,
    /// The other branch's figure.
    pub there: MeasureValue,
}

/// The five measures the comparison table draws.
#[must_use]
pub fn branch_measures(dashboard: &Dashboard) -> [BranchMeasure; 5] {
    let other = &dashboard.other_branch;
    [
        BranchMeasure {
            label: text("Sales"),
            here: MeasureValue::Money(sales_total(dashboard)),
            there: MeasureValue::Money(other.sales),
        },
        BranchMeasure {
            label: text("Prescriptions dispensed"),
            here: MeasureValue::Count(dashboard.prescriptions_dispensed),
            there: MeasureValue::Count(other.prescriptions_dispensed),
        },
        BranchMeasure {
            label: text("Insurance share"),
            here: MeasureValue::Percent(payment_share(dashboard, insurance(dashboard))),
            there: MeasureValue::Percent(other.insurance_share_percent),
        },
        BranchMeasure {
            label: text("Average basket"),
            here: MeasureValue::Money(dashboard.average_basket),
            there: MeasureValue::Money(other.average_basket),
        },
        BranchMeasure {
            label: text("Prescriptions waiting"),
            here: MeasureValue::Count(dashboard.prescriptions_waiting),
            there: MeasureValue::Count(other.prescriptions_waiting),
        },
    ]
}

/// The best sellers, most money first, which is the order the board lists.
#[must_use]
pub fn best_sellers(dashboard: &Dashboard) -> Vec<&MedicineSold> {
    let mut sold: Vec<&MedicineSold> = dashboard.top_medicines.iter().collect();
    sold.sort_by_key(|medicine| std::cmp::Reverse(medicine.sales.round_to_shillings()));
    sold
}

/// The branch this dashboard compares itself against.
#[must_use]
pub fn compared_branch(dashboard: &Dashboard) -> &OtherBranch {
    &dashboard.other_branch
}
