//! The figures one branch's day is made of.

use std::sync::Arc;

use rok_pos_domain::Money;

use super::enums::{Destination, TaskKind, TaskSeverity};

/// A word the dashboard draws on every frame.
///
/// The words are shared rather than rebuilt, so a re-render of an unchanged
/// board copies pointers rather than text.
pub type Text = Arc<str>;

/// A word the dashboard draws, from anything a source holds.
#[must_use]
pub fn text(value: &str) -> Text {
    Arc::from(value)
}

/// What one hour of the day sold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HourSales {
    /// The hour it starts, as the board writes it: `"08"`.
    pub hour: Text,
    /// What it sold.
    pub sales: Money,
}

/// How much came in one way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Payment {
    /// The way it was paid.
    pub label: Text,
    /// How much.
    pub amount: Money,
}

/// One medicine's day, at the top of the best sellers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MedicineSold {
    /// The medicine.
    pub name: Text,
    /// How much of it, with its unit.
    pub quantity: Text,
    /// What it sold for.
    pub sales: Money,
}

/// One line of "Needs you now".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DashboardTask {
    /// What it is about.
    pub kind: TaskKind,
    /// How serious it is.
    pub severity: TaskSeverity,
    /// What it is about, in words.
    pub title: Text,
    /// What to do about it.
    pub detail: Text,
    /// Where following it goes.
    pub destination: Destination,
    /// The words on the right, as the board writes them.
    pub action: Text,
}

impl DashboardTask {
    /// Whether the day's work on this row is finished.
    #[must_use]
    pub const fn is_done(&self) -> bool {
        self.kind.is_done()
    }
}

/// What the other branch did today, for the comparison table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OtherBranch {
    /// The branch's short name.
    pub name: Text,
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

/// Everything the dashboard knows about one branch's day.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dashboard {
    /// This branch's short name.
    pub branch: Text,
    /// Sales by hour, oldest first.
    pub hours: Vec<HourSales>,
    /// Sales by way of payment.
    pub payments: Vec<Payment>,
    /// Prescriptions dispensed today.
    pub prescriptions_dispensed: u32,
    /// Prescriptions waiting to be checked or dispensed.
    pub prescriptions_waiting: u32,
    /// How long the oldest of them has waited, in minutes.
    pub oldest_waiting_minutes: u32,
    /// What expires within 30 days, at cost.
    pub expiring_within_30_days: Money,
    /// How many batches that is.
    pub expiring_batches: u32,
    /// Expired batches blocked at the till.
    pub expired_batches_blocked: u32,
    /// Claims the insurer queried.
    pub claims_queried: u32,
    /// Which insurer and batch those claims are in.
    pub claims_queried_in: Text,
    /// The average basket here.
    pub average_basket: Money,
    /// What needs someone now.
    pub tasks: Vec<DashboardTask>,
    /// The best sellers today.
    pub top_medicines: Vec<MedicineSold>,
    /// The other branch, for comparison.
    pub other_branch: OtherBranch,
}
