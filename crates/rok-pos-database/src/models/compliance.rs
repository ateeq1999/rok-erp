//! The licences and certificates an inspector asks to see.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use uuid::Uuid;

use crate::models::all_values;

/// What a licence or certificate is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum LicenceType {
    /// The premises licence from the Pharmacy Council.
    Premises,
    /// A pharmacist's registration.
    PharmacistRegistration,
    /// The business licence.
    Business,
    /// The permit to hold controlled medicines.
    ControlledPermit,
    /// The fire certificate.
    FireCertificate,
    /// The fridge's calibration certificate.
    FridgeCalibration,
}
all_values!(
    LicenceType: Premises,
    PharmacistRegistration,
    Business,
    ControlledPermit,
    FireCertificate,
    FridgeCalibration,
);

/// One licence or certificate held by a branch: `pharmacy.licence_documents`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.licence_documents", timestamps, soft_delete)]
pub struct LicenceDocument {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The branch it covers.
    pub branch_id: Uuid,
    /// What it is.
    pub licence_type: LicenceType,
    /// Whose name is on it.
    pub holder_name: String,
    /// Its number.
    pub licence_number: String,
    /// When it was issued.
    pub issued_on: Option<NaiveDate>,
    /// When it runs out.
    pub expires_on: Option<NaiveDate>,
    /// The scan, once uploaded.
    pub attachment_id: Option<Uuid>,
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
