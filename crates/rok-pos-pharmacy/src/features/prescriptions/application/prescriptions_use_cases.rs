//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::PrescriptionsError;
use crate::features::prescriptions::data::repository::PrescriptionRepository;
use crate::features::prescriptions::domain::entities::PrescriptionQueue;

/// Read a day's prescriptions, turning a source's failure into the error the
/// screen draws.
#[derive(Clone, Debug)]
pub struct GetPrescriptions<R> {
    /// Where the queue comes from.
    repository: R,
}

impl<R> GetPrescriptions<R>
where
    R: PrescriptionRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the day for `branch_id`, opening `selected` in the rail.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(
        &self,
        branch_id: &str,
        selected: &str,
    ) -> Result<PrescriptionQueue, PrescriptionsError> {
        self.repository
            .get_queue(branch_id, selected)
            .await
            .map_err(PrescriptionsError::from)
    }
}
