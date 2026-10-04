//! The patient's record and the checks a pharmacist works through before a
//! prescription is approved: profiles, notes, interaction rules, the recorded
//! checks and the calls to the prescriber.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

/// The patient's sex as recorded, matching the `sex` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum Sex {
    /// Female.
    Female,
    /// Male.
    Male,
    /// Another value the patient gave.
    Other,
    /// Not known, which is its own answer on a clinical record.
    Unknown,
}

/// How badly two medicines clash, matching the `severity` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum InteractionSeverity {
    /// Worth knowing; nothing is stopped.
    Information,
    /// The pharmacist should think before dispensing.
    Caution,
    /// The pair is dangerous and must not leave the counter together.
    Serious,
}

/// Which check on a prescription this row records, matching the `check_key`
/// check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum CheckKey {
    /// Is this the person the prescription was written for?
    Identity,
    /// Does the patient have an allergy to any line?
    Allergy,
    /// Do the lines clash with each other or with the patient's record?
    Interaction,
    /// Is the same therapy already prescribed elsewhere?
    DuplicateTherapy,
    /// Is each dose inside the safe range for the patient?
    DoseRange,
    /// Does the insurer cover the lines?
    InsuranceCover,
}

/// How a check came out, matching the `result` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum CheckResult {
    /// Nothing to report.
    Passed,
    /// Something to look at, but dispensing may go ahead.
    Warning,
    /// Something that must be resolved first.
    Failed,
}

/// How the prescriber was reached, matching the `contact_method` check
/// constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum ContactMethod {
    /// A call.
    Phone,
    /// A text message.
    Sms,
    /// A `WhatsApp` message, which is how most prescribers in the story reply.
    Whatsapp,
    /// The prescriber walked in.
    InPerson,
}

/// What came of the call, matching the `outcome` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum ContactOutcome {
    /// The prescriber confirmed the prescription as written.
    PrescriberAgreed,
    /// The prescriber changed it, which starts the checks again.
    PrescriptionChanged,
    /// Nobody answered; the pharmacy tries again.
    NoAnswer,
}

/// What a pharmacist needs to know about a patient before dispensing.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.patient_clinical_profiles", timestamps, soft_delete)]
pub struct PatientClinicalProfile {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this record belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The customer this profile describes, one per customer.
    pub customer_id: Uuid,
    /// The date of birth, for the age behind a dose range.
    pub date_of_birth: Option<NaiveDate>,
    /// The sex recorded on the file.
    pub sex: Option<Sex>,
    /// What the patient reacts to, checked against every line.
    pub allergies: Vec<String>,
    /// Conditions as earlier prescriptions spelled them, for the duplicate
    /// therapy check.
    pub conditions_from_prescriptions: Vec<String>,
    /// Who insures the patient, if anyone.
    pub insurance_provider_id: Option<Uuid>,
    /// The member number as the card prints it.
    pub insurance_member_number: Option<String>,
    /// Whether the pharmacy may send refill reminders.
    pub reminder_consent: bool,
    /// When that consent was given; required by a check once consent is on.
    pub reminder_consent_given_at: Option<DateTime<Utc>>,
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

/// A note about a patient's care. Readable only with
/// `pharmacy.clinical_notes.view`, because a clinical note is not stock
/// control.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.clinical_notes", timestamps, soft_delete)]
pub struct ClinicalNote {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this note belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The patient the note is about.
    pub customer_id: Uuid,
    /// The prescription that prompted it, when there was one.
    pub prescription_id: Option<Uuid>,
    /// what the note says.
    pub note_text: String,
    /// The pharmacist who wrote it.
    pub written_by_user_id: Uuid,
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

/// The interaction rule table. Rows are loaded from a reference source the
/// pharmacy has the right to use and reviewed by a pharmacist; the app never
/// invents a rule.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.interaction_rules", timestamps, soft_delete)]
pub struct InteractionRule {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this rule belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The first generic, in whichever order the pair is read.
    pub first_generic_name: String,
    /// The second generic.
    pub second_generic_name: String,
    /// How badly the pair clashes.
    pub severity: InteractionSeverity,
    /// What the pharmacist is told, and why.
    pub message_text: String,
    /// Where the rule came from, which the board shows beside it.
    pub source_reference: String,
    /// The pharmacist who signed the rule off.
    pub reviewed_by_user_id: Option<Uuid>,
    /// When they signed it off.
    pub reviewed_on: Option<NaiveDate>,
    /// Whether the rule is in force; a withdrawn rule is kept, not deleted.
    pub is_active: bool,
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

/// Every check a pharmacist works through before approving a prescription,
/// one row per check, unique per prescription.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.prescription_checks", timestamps, soft_delete)]
pub struct PrescriptionCheck {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this check belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The prescription being checked.
    pub prescription_id: Uuid,
    /// Which of the six checks this row records.
    pub check_key: CheckKey,
    /// How it came out.
    pub result: CheckResult,
    /// What was found, in the words the pharmacist reads at the counter.
    pub details: Option<String>,
    /// The pharmacist who worked through it.
    pub checked_by_user_id: Uuid,
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

/// The call to the prescriber when something on a prescription needs
/// confirming.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.prescriber_contacts", timestamps, soft_delete)]
pub struct PrescriberContact {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this contact belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The prescription that needed confirming.
    pub prescription_id: Uuid,
    /// When the pharmacy reached out.
    pub contacted_at: DateTime<Utc>,
    /// How they reached the prescriber.
    pub contact_method: ContactMethod,
    /// What came of it.
    pub outcome: ContactOutcome,
    /// What was agreed, which the record keeps after the call.
    pub note_text: Option<String>,
    /// The staff member who made the call.
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
