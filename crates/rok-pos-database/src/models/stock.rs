//! Stock the pharmacy has to watch: the fridge temperatures, the recalls and
//! what was done about them, and the checks on each delivery line.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

use crate::models::{Celsius, CheckResult, Quantity, all_values};

/// How serious a recall is, as the regulator classes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum RecallClass {
    /// Could cause serious harm or death.
    #[rok(rename = "class_i")]
    ClassOne,
    /// Could cause temporary harm.
    #[rok(rename = "class_ii")]
    ClassTwo,
    /// Unlikely to cause harm.
    #[rok(rename = "class_iii")]
    ClassThree,
}
all_values!(RecallClass: ClassOne, ClassTwo, ClassThree);

/// Where a recall has got to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum RecallStatus {
    /// Received; nothing done yet.
    Open,
    /// The stock on hand is set aside.
    Quarantined,
    /// The stock went back to the supplier.
    Returned,
    /// Every step is done and signed off.
    Closed,
}
all_values!(RecallStatus: Open, Quarantined, Returned, Closed);

/// One thing done about a recall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum RecallActionType {
    /// Stock set aside.
    Quarantined,
    /// A patient who bought the batch was reached.
    PatientContacted,
    /// Stock sent back.
    ReturnedToSupplier,
    /// The supplier credited it.
    CreditReceived,
}
all_values!(RecallActionType: Quarantined, PatientContacted, ReturnedToSupplier, CreditReceived);

/// What a delivery line was checked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum ReceiptCheckKey {
    /// Enough shelf life left.
    Expiry,
    /// Packs intact.
    Packaging,
    /// Kept between 2 and 8 degrees on the way.
    ColdChain,
    /// The count matches.
    Quantity,
}
all_values!(ReceiptCheckKey: Expiry, Packaging, ColdChain, Quantity);

/// One reading from a fridge or cold box: `pharmacy.temperature_logs`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.temperature_logs", timestamps, soft_delete)]
pub struct TemperatureLog {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The branch the fridge is in.
    pub branch_id: Uuid,
    /// "Fridge 1".
    pub storage_unit_name: String,
    /// When it was read.
    pub recorded_at: DateTime<Utc>,
    /// What it read.
    pub temperature_celsius: Celsius,
    /// Whether that is between 2 and 8 degrees.
    pub is_in_range: bool,
    /// Who read it.
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

/// A batch the supplier or regulator recalled: `pharmacy.recall_notices`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.recall_notices", timestamps, soft_delete)]
pub struct RecallNotice {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// Who issued it.
    pub supplier_name: String,
    /// The marketplace's reference, once Phase 15 receives recalls.
    pub marketplace_recall_reference: Option<String>,
    /// The product recalled.
    pub product_id: Uuid,
    /// The batch recalled.
    pub batch_number: String,
    /// Why.
    pub reason_text: String,
    /// How serious it is.
    pub recall_class: Option<RecallClass>,
    /// When it was issued.
    pub issued_at: Option<NaiveDate>,
    /// What the supplier says to do.
    pub instructions_text: Option<String>,
    /// Where it has got to.
    pub status: RecallStatus,
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

/// One thing done about a recall: `pharmacy.recall_actions`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.recall_actions", timestamps, soft_delete)]
pub struct RecallAction {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The recall it was about.
    pub recall_notice_id: Uuid,
    /// What was done.
    pub action_type: RecallActionType,
    /// The patient reached, for a contact.
    pub customer_id: Option<Uuid>,
    /// How much stock moved, for a quarantine or a return.
    pub quantity: Option<Quantity>,
    /// When it was done.
    pub done_at: DateTime<Utc>,
    /// Who did it.
    pub done_by_user_id: Option<Uuid>,
    /// Anything else worth keeping.
    pub note_text: Option<String>,
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

/// One check on one delivery line: `pharmacy.receipt_quality_checks`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.receipt_quality_checks", timestamps, soft_delete)]
pub struct ReceiptQualityCheck {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The delivery line checked.
    pub goods_receipt_line_id: Uuid,
    /// What was checked.
    pub check_key: ReceiptCheckKey,
    /// How it came out.
    pub result: CheckResult,
    /// The coldest the logger saw, for a cold-chain check.
    pub minimum_temperature_celsius: Option<Celsius>,
    /// The warmest it saw.
    pub maximum_temperature_celsius: Option<Celsius>,
    /// Anything else worth keeping.
    pub note_text: Option<String>,
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
