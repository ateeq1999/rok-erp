//! A medicine's clinical facts, and what may be dispensed in its place.

use chrono::{DateTime, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

use crate::models::all_values;

/// Who may sell a medicine, from the shelf to the locked cupboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum MedicineSchedule {
    /// Anyone at the till may sell it.
    GeneralSale,
    /// Sold only with a pharmacist on duty.
    PharmacyMedicine,
    /// Dispensed only against a prescription.
    PrescriptionOnly,
    /// A prescription, the pharmacist's PIN, and a register entry.
    Controlled,
}
all_values!(MedicineSchedule: GeneralSale, PharmacyMedicine, PrescriptionOnly, Controlled);

/// Where a medicine is kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum StorageCondition {
    /// On the shelf.
    RoomTemperature,
    /// In the fridge, between 2 and 8 degrees.
    RefrigeratedTwoToEight,
}
all_values!(StorageCondition: RoomTemperature, RefrigeratedTwoToEight);

/// Why one product may stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum SubstitutionKind {
    /// The same generic medicine, another brand.
    SameGeneric,
    /// A different medicine for the same purpose, which needs the prescriber.
    Therapeutic,
}
all_values!(SubstitutionKind: SameGeneric, Therapeutic);

/// One row per medicine product: `pharmacy.medicine_details`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.medicine_details", timestamps, soft_delete)]
pub struct MedicineDetails {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The catalogue product these details describe.
    pub product_id: Uuid,
    /// The generic name the interaction rules match on: "amoxicillin".
    pub generic_name: String,
    /// "500mg", "250mg/5ml".
    pub strength_text: String,
    /// "capsule", "suspension".
    pub dosage_form: String,
    /// "100 capsules", "100ml bottle".
    pub pack_size_text: Option<String>,
    /// Who may sell it.
    pub medicine_schedule: MedicineSchedule,
    /// Where it is kept.
    pub storage_condition: StorageCondition,
    /// The regulator's registration number.
    pub regulator_registration_number: Option<String>,
    /// The shortest shelf life a delivery may arrive with.
    pub minimum_shelf_life_months_on_delivery: i32,
    /// Warnings printed on every dosage label.
    pub label_warnings: Vec<String>,
    /// Whether the insurers' formulary covers it.
    pub is_on_insurance_formulary: bool,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When it last changed.
    pub updated_at: DateTime<Utc>,
    /// When it was deleted, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// Bumped on every change, so two people cannot overwrite each other.
    #[rok(version)]
    pub row_version: i64,
}

/// A product that may be dispensed in place of another:
/// `pharmacy.medicine_substitutes`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.medicine_substitutes", timestamps, soft_delete)]
pub struct MedicineSubstitute {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The product prescribed.
    pub product_id: Uuid,
    /// The product that may be given instead.
    pub substitute_product_id: Uuid,
    /// Why it may.
    pub substitution_kind: SubstitutionKind,
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
