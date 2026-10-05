//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::CheckError;
use crate::features::clinical_check::data::repository::ClinicalCheckRepository;
use crate::features::clinical_check::domain::entities::ClinicalCheck;

/// Read one prescription's check, turning a source's failure into the error the
/// screen draws.
#[derive(Clone, Debug)]
pub struct GetClinicalCheck<R> {
    /// Where the check comes from.
    repository: R,
}

impl<R> GetClinicalCheck<R>
where
    R: ClinicalCheckRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the check for `prescription_id` at `branch_id`.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(
        &self,
        branch_id: &str,
        prescription_id: &str,
    ) -> Result<ClinicalCheck, CheckError> {
        self.repository
            .get_check(branch_id, prescription_id)
            .await
            .map_err(CheckError::from)
    }
}
