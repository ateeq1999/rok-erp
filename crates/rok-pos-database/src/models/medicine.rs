//! Medicines and their substitutes: what the catalogue says about how a
//! medicine may be sold, stored and swapped.

use chrono::{DateTime, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

/// How a medicine may be sold, matching the `medicine_schedule` check
/// constraint. It decides what the till demands before it lets a line through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum MedicineSchedule {
    /// Off the shelf without a pharmacist: vitamins, plasters, paracetamol.
    GeneralSale,
    /// Behind the counter, where the pharmacist decides: cough syrups.
    PharmacyMedicine,
    /// Dispensed against a prescription only.
    PrescriptionOnly,
    /// Dispensed against a prescription and kept in the locked cabinet with
    /// its own register.
    Controlled,
}

/// How a medicine must be kept, matching the `storage_condition` check
/// constraint. It is what puts insulin in Fridge 1 and what the temperature
/// logs are there to prove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum StorageCondition {
    /// The default: a cool dry shelf, 15 to 25 degrees.
    RoomTemperature,
    /// Two to eight degrees, and nowhere else.
    RefrigeratedTwoToEight,
}

/// Why a product may stand in for another, matching the `substitution_kind`
/// check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum SubstitutionKind {
    /// The same generic molecule, different brand or strength presentation.
    SameGeneric,
    /// A different molecule doing the same job, which needs the pharmacist's
    /// judgement rather than the till's.
    Therapeutic,
}

/// One row per medicine product: what the till, the labels and the rules need
/// beyond the catalogue row.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.medicine_details", timestamps, soft_delete)]
pub struct MedicineDetails {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this medicine belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The catalogue product this row describes.
    pub product_id: Uuid,
    /// The INN or established name the interaction rules match on.
    pub generic_name: String,
    /// What it is, as on the box: "500mg".
    pub strength_text: String,
    /// Capsule, tablet, syrup, injection.
    pub dosage_form: String,
    /// What a pack contains, as on the box: "box of 100".
    pub pack_size_text: Option<String>,
    /// How it may be sold.
    pub medicine_schedule: MedicineSchedule,
    /// How it must be kept.
    pub storage_condition: StorageCondition,
    /// The regulator's number for it, for the inspection pack.
    pub regulator_registration_number: Option<String>,
    /// The shelf life the pharmacy refuses to accept on delivery, in months.
    pub minimum_shelf_life_months_on_delivery: i32,
    /// What the label must warn about, printed on every dispensing label.
    pub label_warnings: Vec<String>,
    /// Whether the national insurer pays for it.
    pub is_on_insurance_formulary: bool,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
    /// When the row was removed, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// The optimistic lock: a save that lost a race fails rather than
    /// overwriting.
    #[rok(version)]
    pub row_version: i64,
}

/// A medicine that may be dispensed in place of another, decided by the
/// pharmacist rather than the till.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.medicine_substitutes", timestamps, soft_delete)]
pub struct MedicineSubstitute {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this substitution belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The medicine being substituted.
    pub product_id: Uuid,
    /// The medicine offered in its place.
    pub substitute_product_id: Uuid,
    /// Whether it is the same generic or a therapeutic alternative.
    pub substitution_kind: SubstitutionKind,
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
