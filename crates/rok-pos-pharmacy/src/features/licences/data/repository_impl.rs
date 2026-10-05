//! The licences repository, reading the board's own figures.
//!
//! The phase that stores licence documents replaces this with the pharmacy's
//! database. The `BLoC` only knows the trait, so nothing above this file
//! changes when it does.

use std::future::Future;

use super::repository::{LicenceRepository, RepositoryError};
use super::story;
use crate::features::licences::domain::entities::Licences;

/// Reads the folder from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryLicencesRepository;

impl LicenceRepository for StoryLicencesRepository {
    fn get_licences(&self) -> impl Future<Output = Result<Licences, RepositoryError>> + Send {
        std::future::ready(Ok(Licences::from(story::folder())))
    }
}

/// A source that is not there yet, so the error state can be drawn and tested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnavailableLicencesRepository {
    /// Why the source cannot be read.
    pub detail: &'static str,
}

impl LicenceRepository for UnavailableLicencesRepository {
    fn get_licences(&self) -> impl Future<Output = Result<Licences, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}
