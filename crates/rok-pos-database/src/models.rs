//! The models the pharmacy's clinical, stock and compliance screens read and
//! write: the fifteen tables migration `0002_add_clinical_stock_and_compliance`
//! adds to the `pharmacy` schema.
//!
//! One file per table group, mirroring the way the boards group the work. The
//! closed sets of values a column allows are [`rok_db::DbEnum`]s stored as
//! text, spelled exactly as the `check` constraints spell them, so a status the
//! database would reject is a compile error here instead of a runtime surprise.
//!
//! Every table follows the module conventions: a UUID version 7 primary key, an
//! `organization_id` marked [`rok(tenant)`](rok_db::Model) so a session sees its
//! own business's rows, `created_at`/`updated_at`, a soft `deleted_at` and a
//! `row_version` for optimistic locking.

pub mod clinical;
pub mod compliance;
pub mod dispensing;
pub mod insurance;
pub mod medicine;
pub mod numeric;

pub use clinical::{
    CheckKey, CheckResult, ClinicalNote, ContactMethod, ContactOutcome, InteractionRule,
    InteractionSeverity, PatientClinicalProfile, PrescriberContact, PrescriptionCheck, Sex,
};
pub use compliance::{
    LicenceDocument, LicenceType, RecallAction, RecallActionType, RecallClass, RecallNotice,
    RecallStatus, ReceiptCheckKey, ReceiptQualityCheck,
};
pub use dispensing::{DispensingLabel, RefillReminderStatus, RefillSchedule, TemperatureLog};
pub use insurance::{ClaimBatchStatus, InsuranceClaimBatch};
pub use medicine::{
    MedicineDetails, MedicineSchedule, MedicineSubstitute, StorageCondition, SubstitutionKind,
};
pub use numeric::{Quantity, Temperature};
