//! The records a source hands over, and the entities they become.
//!
//! A record is the source's shape: owned `String`s, amounts in whole shillings,
//! and a tone written as its own word. [`From`] turns it into the domain, which
//! shares its words and keeps its money typed.

use rok_pos_domain::Money;

use crate::features::dashboard::domain::entities::{
    Dashboard, DashboardTask, HourSales, MedicineSold, OtherBranch, Payment, text,
};
use crate::features::dashboard::domain::enums::{Destination, TaskKind, TaskSeverity};

/// What one hour of the day sold, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HourRecord {
    /// The hour it starts.
    pub hour: String,
    /// What it sold, in whole shillings.
    pub sales: i64,
}

/// How much came in one way, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentRecord {
    /// The way it was paid.
    pub label: String,
    /// How much, in whole shillings.
    pub amount: i64,
}

/// One medicine's day, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MedicineSoldRecord {
    /// The medicine.
    pub name: String,
    /// How much of it, with its unit.
    pub quantity: String,
    /// What it sold for, in whole shillings.
    pub sales: i64,
}

/// One line of "Needs you now", as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskRecord {
    /// The kind of work, as the source names it.
    pub kind: String,
    /// How serious it is, as the source names it.
    pub severity: String,
    /// What it is about.
    pub title: String,
    /// What to do about it.
    pub detail: String,
    /// Which screen follows it, as the source names the screen.
    pub screen: String,
    /// The words on the right.
    pub action: String,
}

/// The other branch's day, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OtherBranchRecord {
    /// The branch's short name.
    pub name: String,
    /// What it sold, in whole shillings.
    pub sales: i64,
    /// Prescriptions it dispensed.
    pub prescriptions_dispensed: u32,
    /// Its insurance share, in percent.
    pub insurance_share_percent: i64,
    /// Its average basket, in whole shillings.
    pub average_basket: i64,
    /// Prescriptions still waiting there.
    pub prescriptions_waiting: u32,
}

/// A whole day of one branch, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DashboardRecord {
    /// The branch this day belongs to.
    pub branch: String,
    /// Sales by hour.
    pub hours: Vec<HourRecord>,
    /// Sales by way of payment.
    pub payments: Vec<PaymentRecord>,
    /// Prescriptions dispensed today.
    pub prescriptions_dispensed: u32,
    /// Prescriptions waiting.
    pub prescriptions_waiting: u32,
    /// How long the oldest of them has waited, in minutes.
    pub oldest_waiting_minutes: u32,
    /// What expires within 30 days, at cost, in whole shillings.
    pub expiring_within_30_days: i64,
    /// How many batches that is.
    pub expiring_batches: u32,
    /// Expired batches blocked at the till.
    pub expired_batches_blocked: u32,
    /// Claims the insurer queried.
    pub claims_queried: u32,
    /// Which insurer and batch those claims are in.
    pub claims_queried_in: String,
    /// The average basket, in whole shillings.
    pub average_basket: i64,
    /// What needs someone now.
    pub tasks: Vec<TaskRecord>,
    /// The best sellers today.
    pub top_medicines: Vec<MedicineSoldRecord>,
    /// The other branch, for comparison.
    pub other_branch: OtherBranchRecord,
}

/// What a source calls a kind of work.
fn task_kind(kind: &str) -> TaskKind {
    match kind {
        "Recall" => TaskKind::Recall,
        "Check" => TaskKind::Check,
        "Delivery" => TaskKind::Delivery,
        "Licence" => TaskKind::Licence,
        _ => TaskKind::Done,
    }
}

/// What a source calls a severity.
fn task_severity(severity: &str) -> TaskSeverity {
    match severity {
        "Danger" => TaskSeverity::Danger,
        "Warning" => TaskSeverity::Warning,
        "Info" => TaskSeverity::Info,
        _ => TaskSeverity::Success,
    }
}

/// Which screen a task's row follows.
fn task_destination(screen: &str) -> Destination {
    match screen {
        "recalls" => Destination::Recall,
        "prescriptions" => Destination::PrescriptionCheck,
        "receive" => Destination::Receive,
        "licences" => Destination::Licences,
        _ => Destination::ControlledRegister,
    }
}

impl From<HourRecord> for HourSales {
    fn from(record: HourRecord) -> Self {
        Self {
            hour: text(&record.hour),
            sales: Money::from_shillings(record.sales),
        }
    }
}

impl From<PaymentRecord> for Payment {
    fn from(record: PaymentRecord) -> Self {
        Self {
            label: text(&record.label),
            amount: Money::from_shillings(record.amount),
        }
    }
}

impl From<MedicineSoldRecord> for MedicineSold {
    fn from(record: MedicineSoldRecord) -> Self {
        Self {
            name: text(&record.name),
            quantity: text(&record.quantity),
            sales: Money::from_shillings(record.sales),
        }
    }
}

impl From<TaskRecord> for DashboardTask {
    fn from(record: TaskRecord) -> Self {
        Self {
            kind: task_kind(&record.kind),
            severity: task_severity(&record.severity),
            title: text(&record.title),
            detail: text(&record.detail),
            destination: task_destination(&record.screen),
            action: text(&record.action),
        }
    }
}

impl From<OtherBranchRecord> for OtherBranch {
    fn from(record: OtherBranchRecord) -> Self {
        Self {
            name: text(&record.name),
            sales: Money::from_shillings(record.sales),
            prescriptions_dispensed: record.prescriptions_dispensed,
            insurance_share_percent: record.insurance_share_percent,
            average_basket: Money::from_shillings(record.average_basket),
            prescriptions_waiting: record.prescriptions_waiting,
        }
    }
}

impl From<DashboardRecord> for Dashboard {
    fn from(record: DashboardRecord) -> Self {
        Self {
            branch: text(&record.branch),
            hours: record.hours.into_iter().map(HourSales::from).collect(),
            payments: record.payments.into_iter().map(Payment::from).collect(),
            prescriptions_dispensed: record.prescriptions_dispensed,
            prescriptions_waiting: record.prescriptions_waiting,
            oldest_waiting_minutes: record.oldest_waiting_minutes,
            expiring_within_30_days: Money::from_shillings(record.expiring_within_30_days),
            expiring_batches: record.expiring_batches,
            expired_batches_blocked: record.expired_batches_blocked,
            claims_queried: record.claims_queried,
            claims_queried_in: text(&record.claims_queried_in),
            average_basket: Money::from_shillings(record.average_basket),
            tasks: record.tasks.into_iter().map(DashboardTask::from).collect(),
            top_medicines: record
                .top_medicines
                .into_iter()
                .map(MedicineSold::from)
                .collect(),
            other_branch: OtherBranch::from(record.other_branch),
        }
    }
}
