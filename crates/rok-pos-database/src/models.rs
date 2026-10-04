//! rok-db models for the pharmacy module's second migration, one file per
//! group of tables.
//!
//! Every model is scoped to its business with `#[rok(tenant)]`, so a query
//! made inside [`crate::in_business`] only ever sees that business's rows, and
//! row level security catches whatever the filter misses. Every status column
//! is a [`rok_db::DbEnum`] stored as text, with exactly the values the
//! migration's `check` constraint allows.
//!
//! ```
//! use rok_pos_database::models::{CheckResult, MedicineSchedule};
//!
//! assert_eq!(MedicineSchedule::PrescriptionOnly.as_str(), "prescription_only");
//! assert_eq!("warning".parse::<CheckResult>().ok(), Some(CheckResult::Warning));
//! ```

pub mod clinical;
pub mod compliance;
pub mod insurance;
pub mod medicines;
pub mod patients;
pub mod stock;

use rok_db::DbNewtype;
use rust_decimal::Decimal;

pub use clinical::{
    CheckResult, ContactMethod, ContactOutcome, DispensingLabel, InteractionRule,
    InteractionSeverity, PrescriberContact, PrescriptionCheck, PrescriptionCheckKey,
};
pub use compliance::{LicenceDocument, LicenceType};
pub use insurance::{ClaimBatchStatus, InsuranceClaimBatch};
pub use medicines::{
    MedicineDetails, MedicineSchedule, MedicineSubstitute, StorageCondition, SubstitutionKind,
};
pub use patients::{ClinicalNote, PatientClinicalProfile, RefillSchedule, ReminderStatus, Sex};
pub use stock::{
    ReceiptCheckKey, ReceiptQualityCheck, RecallAction, RecallActionType, RecallClass,
    RecallNotice, RecallStatus, TemperatureLog,
};

/// A temperature in degrees Celsius, as a `numeric(5,2)` column keeps it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, DbNewtype)]
pub struct Celsius(pub Decimal);

/// A count of units that may be fractional, as a `numeric(18,3)` column keeps
/// it: 14 bottles, or 2.5 strips.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, DbNewtype)]
pub struct Quantity(pub Decimal);

/// Every value of a status enum, so a test can hold it against the migration's
/// `check` constraint.
pub trait AllValues: Sized + 'static {
    /// Every variant, in declaration order.
    const ALL: &'static [Self];

    /// The text the database stores for this variant.
    fn stored(&self) -> &'static str;
}

/// Implement [`AllValues`] for a `DbEnum` from its variants.
macro_rules! all_values {
    ($name:ident: $($variant:ident),+ $(,)?) => {
        impl $crate::models::AllValues for $name {
            const ALL: &'static [Self] = &[$($name::$variant),+];

            fn stored(&self) -> &'static str {
                self.as_str()
            }
        }
    };
}
pub(crate) use all_values;
