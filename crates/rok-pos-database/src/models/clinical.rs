//! The pharmacist's check on a prescription: the interaction rules it runs
//! against, the checks recorded, the calls to the prescriber, and the labels
//! printed.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

use crate::models::all_values;

/// How much an interaction matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum InteractionSeverity {
    /// Worth knowing.
    Information,
    /// Check with the prescriber or counsel the patient.
    Caution,
    /// Do not dispense without the prescriber.
    Serious,
}
all_values!(InteractionSeverity: Information, Caution, Serious);

/// What a prescription check looked at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum PrescriptionCheckKey {
    /// The prescription is for this patient.
    Identity,
    /// Nothing on it is something they are allergic to.
    Allergy,
    /// No two medicines interact.
    Interaction,
    /// No two medicines do the same job.
    DuplicateTherapy,
    /// Every dose is in range.
    DoseRange,
    /// The insurer covers it.
    InsuranceCover,
}
all_values!(
    PrescriptionCheckKey: Identity,
    Allergy,
    Interaction,
    DuplicateTherapy,
    DoseRange,
    InsuranceCover,
);

/// How a check came out, on a prescription or a delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum CheckResult {
    /// Nothing to act on.
    Passed,
    /// The pharmacist has to decide.
    Warning,
    /// Stop.
    Failed,
}
all_values!(CheckResult: Passed, Warning, Failed);

/// How the prescriber was reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum ContactMethod {
    /// A phone call.
    Phone,
    /// A text message.
    TextMessage,
    /// An email.
    Email,
    /// In person.
    InPerson,
}
all_values!(ContactMethod: Phone, TextMessage, Email, InPerson);

/// What the prescriber said.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum ContactOutcome {
    /// Dispense as written.
    PrescriberAgreed,
    /// The prescription was changed.
    PrescriptionChanged,
    /// Nobody answered.
    NoAnswer,
}
all_values!(ContactOutcome: PrescriberAgreed, PrescriptionChanged, NoAnswer);

/// Two generic medicines that interact, from a reference source a pharmacist
/// reviewed: `pharmacy.interaction_rules`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.interaction_rules", timestamps, soft_delete)]
pub struct InteractionRule {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// One generic name.
    pub first_generic_name: String,
    /// The other.
    pub second_generic_name: String,
    /// How much it matters.
    pub severity: InteractionSeverity,
    /// What the alert says.
    pub message_text: String,
    /// Where the rule comes from; every alert names it.
    pub source_reference: Option<String>,
    /// The pharmacist who reviewed it.
    pub reviewed_by_user_id: Option<Uuid>,
    /// When they did.
    pub reviewed_on: Option<NaiveDate>,
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

/// One check recorded against a prescription: `pharmacy.prescription_checks`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.prescription_checks", timestamps, soft_delete)]
pub struct PrescriptionCheck {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The prescription checked.
    pub prescription_id: Uuid,
    /// What was checked.
    pub check_key: PrescriptionCheckKey,
    /// How it came out.
    pub result: CheckResult,
    /// What was found.
    pub details: Option<String>,
    /// Who checked.
    pub checked_by_user_id: Option<Uuid>,
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

/// A call to the prescriber about a prescription:
/// `pharmacy.prescriber_contacts`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.prescriber_contacts", timestamps, soft_delete)]
pub struct PrescriberContact {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The prescription it was about.
    pub prescription_id: Uuid,
    /// When the prescriber was reached.
    pub contacted_at: DateTime<Utc>,
    /// How.
    pub contact_method: ContactMethod,
    /// What they said.
    pub outcome: ContactOutcome,
    /// Anything else worth keeping.
    pub note_text: Option<String>,
    /// Who made the call.
    pub recorded_by_user_id: Option<Uuid>,
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

/// A dosage label printed for one prescription item:
/// `pharmacy.dispensing_labels`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.dispensing_labels", timestamps, soft_delete)]
pub struct DispensingLabel {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The prescription item it labels.
    pub prescription_item_id: Uuid,
    /// The words on the label.
    pub label_text: String,
    /// When it was first printed.
    pub printed_at: DateTime<Utc>,
    /// Who printed it.
    pub printed_by_user_id: Option<Uuid>,
    /// How many times it was printed again.
    pub reprint_count: i32,
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
