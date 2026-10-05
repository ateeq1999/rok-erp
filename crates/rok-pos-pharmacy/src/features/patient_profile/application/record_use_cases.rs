//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::PatientError;
use crate::features::patient_profile::data::repository::PatientRepository;
use crate::features::patient_profile::domain::entities::Patient;

/// Read one patient's record, turning a source's failure into the error the
/// screen draws.
#[derive(Clone, Debug)]
pub struct GetPatient<R> {
    /// Where the record comes from.
    repository: R,
}

impl<R> GetPatient<R>
where
    R: PatientRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the record for `patient_id`.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(&self, patient_id: &str) -> Result<Patient, PatientError> {
        self.repository
            .get_patient(patient_id)
            .await
            .map_err(PatientError::from)
    }
}
