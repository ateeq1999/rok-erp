//! Where a patient's record and the list it is found from are read from.

use std::future::Future;

use crate::features::patient_profile::domain::entities::{Listing, Patient};

/// A read that did not work. The source's own error stops here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryError {
    /// The source could not be reached.
    Unavailable(String),
    /// The source answered with something this feature cannot read.
    Unexpected(String),
}

impl RepositoryError {
    /// What the screen tells the person about it.
    #[must_use]
    pub fn message(&self) -> String {
        match self {
            RepositoryError::Unavailable(detail) => {
                format!("The patient's record could not be read. {detail}")
            }
            RepositoryError::Unexpected(detail) => {
                format!("The record came back in a shape this screen cannot read. {detail}")
            }
        }
    }
}

/// One patient's record, for the route the list opened.
pub trait PatientRepository: Send {
    /// Read the record for `patient_id`.
    fn get_patient(
        &self,
        patient_id: &str,
    ) -> impl Future<Output = Result<Patient, RepositoryError>> + Send;
}

/// The pharmacy's directory: every patient a record is found from.
pub trait DirectoryRepository: Send {
    /// Read the list, most urgent first.
    fn get_list(&self) -> impl Future<Output = Result<Vec<Listing>, RepositoryError>> + Send;
}
