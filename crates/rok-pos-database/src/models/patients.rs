//! What the pharmacist knows about a patient: their clinical profile, the
//! notes written about them, and the refills they are due.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

use crate::models::all_values;

/// A patient's sex, as a prescription records it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum Sex {
    /// Female.
    Female,
    /// Male.
    Male,
    /// Recorded, and neither of the above.
    Other,
    /// Not recorded.
    Unknown,
}
all_values!(Sex: Female, Male, Other, Unknown);

/// Where a refill reminder has got to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum ReminderStatus {
    /// Not due yet.
    NotDue,
    /// A reminder is queued.
    Scheduled,
    /// The reminder went out.
    Sent,
    /// Due, but the patient has not agreed to reminders.
    NoConsent,
    /// The patient collected the refill.
    Collected,
}
all_values!(ReminderStatus: NotDue, Scheduled, Sent, NoConsent, Collected);

/// A patient's clinical facts, one per customer:
/// `pharmacy.patient_clinical_profiles`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.patient_clinical_profiles", timestamps, soft_delete)]
pub struct PatientClinicalProfile {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The customer this profile is about.
    pub customer_id: Uuid,
    /// Their date of birth, for dose checks.
    pub date_of_birth: Option<NaiveDate>,
    /// Their sex.
    pub sex: Option<Sex>,
    /// What they are allergic to, by generic name.
    pub allergies: Vec<String>,
    /// Conditions their prescriptions show.
    pub conditions_from_prescriptions: Vec<String>,
    /// Their insurer.
    pub insurance_provider_id: Option<Uuid>,
    /// Their membership number with that insurer.
    pub insurance_member_number: Option<String>,
    /// Whether they agreed to refill reminders.
    pub reminder_consent: bool,
    /// When they agreed.
    pub reminder_consent_given_at: Option<DateTime<Utc>>,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When it last changed.
    pub updated_at: DateTime<Utc>,
    /// When it was deleted, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// Bumped on every change.
    #[rok(version)]
    pub row_version: i64,
}

/// A note a pharmacist wrote about a patient or a prescription:
/// `pharmacy.clinical_notes`. Readable only with
/// `pharmacy.clinical_notes.view`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.clinical_notes", timestamps, soft_delete)]
pub struct ClinicalNote {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The patient it is about.
    pub customer_id: Option<Uuid>,
    /// The prescription it is about.
    pub prescription_id: Option<Uuid>,
    /// What it says.
    pub note_text: String,
    /// Who wrote it.
    pub written_by_user_id: Option<Uuid>,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When it last changed.
    pub updated_at: DateTime<Utc>,
    /// When it was deleted, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// Bumped on every change.
    #[rok(version)]
    pub row_version: i64,
}

/// When a chronic patient's next supply is due: `pharmacy.refill_schedules`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.refill_schedules", timestamps, soft_delete)]
pub struct RefillSchedule {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The patient.
    pub customer_id: Uuid,
    /// The medicine they take.
    pub product_id: Uuid,
    /// How many days one supply lasts.
    pub days_of_supply: i32,
    /// When they last collected it.
    pub last_filled_on: Option<NaiveDate>,
    /// When the next supply is due.
    pub next_due_on: Option<NaiveDate>,
    /// Where the reminder has got to.
    pub reminder_status: ReminderStatus,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When it last changed.
    pub updated_at: DateTime<Utc>,
    /// When it was deleted, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// Bumped on every change.
    #[rok(version)]
    pub row_version: i64,
}
