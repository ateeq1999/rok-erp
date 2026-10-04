//! What the pharmacy hands to the patient and keeps watch on afterwards:
//! labels, refills and the temperature log for the fridge.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

/// Whether a chronic medicine's refill is due yet, matching the
/// `reminder_status` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum RefillReminderStatus {
    /// The next fill is still weeks away.
    NotDue,
    /// Due, and queued to be sent.
    Scheduled,
    /// The reminder has gone out.
    Sent,
    /// The patient has not consented to reminders, so nothing is sent.
    NoConsent,
    /// The patient came and collected; the schedule restarts.
    Collected,
}

/// One row per label printed for a prescription item; reprints are counted
/// because a label that is printed twice is worth asking about.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.dispensing_labels", timestamps, soft_delete)]
pub struct DispensingLabel {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this label belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The prescription item the label was printed for.
    pub prescription_item_id: Uuid,
    /// The text as it goes to the printer.
    pub label_text: String,
    /// When this copy was printed.
    pub printed_at: DateTime<Utc>,
    /// Who printed it.
    pub printed_by_user_id: Uuid,
    /// How many times the item's label has been printed, counting this one.
    pub reprint_count: i32,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
    /// When the row was removed, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// The optimistic lock.
    #[rok(version)]
    pub row_version: i64,
}

/// When a chronic medicine runs out, when the last one was filled and when
/// the next one is due, unique per patient and product.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.refill_schedules", timestamps, soft_delete)]
pub struct RefillSchedule {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this schedule belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The patient the medicine is for.
    pub customer_id: Uuid,
    /// The medicine on repeat.
    pub product_id: Uuid,
    /// How many days one fill covers, which sets the interval.
    pub days_of_supply: i32,
    /// When it was last filled.
    pub last_filled_on: Option<NaiveDate>,
    /// When the next fill is due; the board's "9 refills due this week" is
    /// this column.
    pub next_due_on: NaiveDate,
    /// Where the reminder has got to.
    pub reminder_status: RefillReminderStatus,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
    /// When the row was removed, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// The optimistic lock.
    #[rok(version)]
    pub row_version: i64,
}

/// Fridge and cold cabinet readings, one row per recording.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.temperature_logs", timestamps, soft_delete)]
pub struct TemperatureLog {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this reading belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The branch the unit stands in.
    pub branch_id: Uuid,
    /// What was measured: "Fridge 1".
    pub storage_unit_name: String,
    /// When the reading was taken.
    pub recorded_at: DateTime<Utc>,
    /// What the thermometer said.
    pub temperature_celsius: crate::models::numeric::Temperature,
    /// Whether it was inside the unit's range, decided when it was taken.
    pub is_in_range: bool,
    /// Who took the reading.
    pub recorded_by_user_id: Uuid,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
    /// When the row was removed, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// The optimistic lock.
    #[rok(version)]
    pub row_version: i64,
}
