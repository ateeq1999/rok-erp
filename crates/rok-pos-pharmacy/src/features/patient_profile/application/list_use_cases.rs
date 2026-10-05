//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::PatientError;
use crate::features::patient_profile::data::repository::DirectoryRepository;
use crate::features::patient_profile::domain::entities::Listing;

/// Read the pharmacy's directory, turning a source's failure into the error the
/// screen draws.
#[derive(Clone, Debug)]
pub struct ListDirectory<R> {
    /// Where the list comes from.
    repository: R,
}

impl<R> ListDirectory<R>
where
    R: DirectoryRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the list, most urgent first.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(&self) -> Result<Vec<Listing>, PatientError> {
        self.repository.get_list().await.map_err(PatientError::from)
    }
}
