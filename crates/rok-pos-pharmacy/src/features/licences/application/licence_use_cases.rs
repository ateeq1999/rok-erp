//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::LicenceError;
use crate::features::licences::data::repository::LicenceRepository;
use crate::features::licences::domain::entities::Licences;

/// Read the folder, turning a source's failure into the error the screen draws.
#[derive(Clone, Debug)]
pub struct ReadLicences<R> {
    /// Where the folder comes from.
    repository: R,
}

impl<R> ReadLicences<R>
where
    R: LicenceRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the licences, the checklist and the documents.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(&self) -> Result<Licences, LicenceError> {
        self.repository
            .get_licences()
            .await
            .map_err(LicenceError::from)
    }
}
