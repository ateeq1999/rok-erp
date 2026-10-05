//! The application's own error, which is what the screens draw.

use crate::features::patient_profile::data::repository::RepositoryError;

/// A read that did not work, in the words the screen shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatientError {
    /// The record or the list could not be read.
    Unreadable(String),
    /// It came back in a shape this screen cannot read.
    UnreadableShape(String),
}

impl PatientError {
    /// What the person is told.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            PatientError::Unreadable(message) | PatientError::UnreadableShape(message) => message,
        }
    }
}

impl From<RepositoryError> for PatientError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::Unavailable(detail) => PatientError::Unreadable(format!(
                "The patient's record could not be read. {detail}"
            )),
            RepositoryError::Unexpected(detail) => PatientError::UnreadableShape(format!(
                "The record came back in a shape this screen cannot read. {detail}"
            )),
        }
    }
}
