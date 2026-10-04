//! What an inspection and a recall ask the pharmacy to keep: licences, the
//! quality checks done on a delivery, and the recall itself.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

/// The document an inspection asks for, matching the `licence_type` check
/// constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum LicenceType {
    /// The premises licence for the address itself.
    Premises,
    /// The pharmacist in charge's registration.
    PharmacistRegistration,
    /// The business registration.
    Business,
    /// The permit to hold controlled medicines.
    ControlledPermit,
    /// The fire certificate.
    FireCertificate,
    /// The fridge's calibration certificate, which is what makes the
    /// temperature log defensible.
    FridgeCalibration,
}

/// A recall notice from a supplier or the regulator.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.recall_notices", timestamps, soft_delete)]
pub struct RecallNotice {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this recall belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The pharmacy's own reference, the one on the board: RC-0047.
    pub recall_reference: String,
    /// Who sent it, as the board shows it: "Uzima Pharmaceuticals".
    pub supplier_name: String,
    /// The marketplace's reference when it arrived from there.
    pub marketplace_recall_reference: Option<String>,
    /// The medicine being recalled.
    pub product_id: Uuid,
    /// The batch to stop, which is the first thing entered.
    pub batch_number: String,
    /// Why it was recalled, in the sender's words.
    pub reason_text: String,
    /// How serious it is, matching the `recall_class` check constraint.
    pub recall_class: RecallClass,
    /// When it was issued, which the board shows as "issued Fri 2 Oct 09:30".
    pub issued_at: DateTime<Utc>,
    /// What to do: "Stop dispensing, quarantine, return to supplier".
    pub instructions_text: Option<String>,
    /// Where the recall has got to, matching the `status` check constraint.
    pub status: RecallStatus,
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

/// How serious a recall is, matching the `recall_class` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum RecallClass {
    /// A dangerous defect: act now, tell everyone.
    ClassI,
    /// A temporary or medically reversible problem.
    ClassIi,
    /// Little or no risk.
    ClassIii,
}

/// Where a recall has got to, matching the `status` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum RecallStatus {
    /// Entered and blocking the till, nothing quarantined yet.
    Open,
    /// The stock on hand is in the quarantine box.
    Quarantined,
    /// The stock has gone back to the supplier.
    Returned,
    /// Every step is done and the recall is closed.
    Closed,
}

/// What the pharmacy did about a recall, matching the `action_type` check
/// constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum RecallActionType {
    /// Moved the stock on hand into quarantine.
    Quarantined,
    /// Reached a patient who bought it, with the outcome recorded.
    PatientContacted,
    /// Sent the stock back, with the quantity.
    ReturnedToSupplier,
    /// The supplier's credit arrived, with the amount.
    CreditReceived,
}

/// What the pharmacy did about a recall: quarantine, contact, return, credit.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.recall_actions", timestamps, soft_delete)]
pub struct RecallAction {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this action belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The recall it belongs to.
    pub recall_notice_id: Uuid,
    /// Which step of the recall this row records.
    pub action_type: RecallActionType,
    /// The patient reached, when the action is a contact.
    pub customer_id: Option<Uuid>,
    /// How much the action covered: 14 bottles quarantined, say.
    pub quantity: Option<crate::models::numeric::Quantity>,
    /// When it was done.
    pub done_at: DateTime<Utc>,
    /// Who did it.
    pub done_by_user_id: Uuid,
    /// What happened, which is where "3 reached by text, 2 pending" comes
    /// from.
    pub note_text: Option<String>,
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

/// The licences an inspection asks for, kept with their expiry dates so the
/// readiness checklist can be computed rather than remembered.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.licence_documents", timestamps, soft_delete)]
pub struct LicenceDocument {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this licence belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The branch it covers; some licences are per premises.
    pub branch_id: Uuid,
    /// Which of the six documents it is.
    pub licence_type: LicenceType,
    /// Whose it is: the pharmacy's name or the pharmacist's.
    pub holder_name: String,
    /// The number as the document prints it.
    pub licence_number: Option<String>,
    /// When it was issued.
    pub issued_on: Option<NaiveDate>,
    /// When it expires, which drives the "due in 57 days" warning.
    pub expires_on: Option<NaiveDate>,
    /// The scanned document in core.attachments.
    pub attachment_id: Option<Uuid>,
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

/// The quality checks done on a delivery, line by line: expiry, packaging,
/// cold chain and quantity, unique per receipt line and check.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.receipt_quality_checks", timestamps, soft_delete)]
pub struct ReceiptQualityCheck {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this check belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The receipt line being checked.
    pub goods_receipt_line_id: Uuid,
    /// Which of the four checks this row records.
    pub check_key: ReceiptCheckKey,
    /// How it came out.
    pub result: crate::models::clinical::CheckResult,
    /// The bottom of the range the logger must have stayed inside.
    pub minimum_temperature_celsius: Option<crate::models::numeric::Temperature>,
    /// The top of that range.
    pub maximum_temperature_celsius: Option<crate::models::numeric::Temperature>,
    /// What was found: "shelf life under 12 months", say.
    pub note_text: Option<String>,
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

/// Which quality check on a delivery this row records, matching the
/// `check_key` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum ReceiptCheckKey {
    /// Does the batch leave enough shelf life to sell?
    Expiry,
    /// Is the pack intact?
    Packaging,
    /// Did the cold chain hold between the recorded limits?
    ColdChain,
    /// Did the count match the delivery note?
    Quantity,
}
