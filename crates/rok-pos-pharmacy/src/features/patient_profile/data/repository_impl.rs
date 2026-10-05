//! The patient's repositories, reading the board's own figures.
//!
//! Phase 3 replaces this with the pharmacy's database. The `BLoC` only knows
//! the traits, so nothing above this file changes when it does.

use std::future::Future;

use super::repository::{DirectoryRepository, PatientRepository, RepositoryError};
use super::story;
use crate::features::patient_profile::domain::entities::{Listing, Patient};

/// Reads a patient from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryPatientRepository;

impl PatientRepository for StoryPatientRepository {
    fn get_patient(
        &self,
        _patient_id: &str,
    ) -> impl Future<Output = Result<Patient, RepositoryError>> + Send {
        std::future::ready(Ok(Patient::from(story::mzee_salim())))
    }
}

/// Reads the directory from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryDirectoryRepository;

impl DirectoryRepository for StoryDirectoryRepository {
    fn get_list(&self) -> impl Future<Output = Result<Vec<Listing>, RepositoryError>> + Send {
        let list = story::directory()
            .into_iter()
            .map(Listing::from)
            .collect::<Vec<_>>();
        std::future::ready(Ok(list))
    }
}

/// A source that is not there yet, so the error state can be drawn and tested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnavailablePatientRepository {
    /// Why the source cannot be read.
    pub detail: &'static str,
}

impl PatientRepository for UnavailablePatientRepository {
    fn get_patient(
        &self,
        _patient_id: &str,
    ) -> impl Future<Output = Result<Patient, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}

impl DirectoryRepository for UnavailablePatientRepository {
    fn get_list(&self) -> impl Future<Output = Result<Vec<Listing>, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}
