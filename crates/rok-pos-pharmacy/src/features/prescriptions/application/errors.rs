//! The application's own error, which is what the screen draws.

use crate::features::prescriptions::data::repository::RepositoryError;

/// A read that did not work, in the words the screen shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrescriptionsError {
    /// The queue could not be read.
    Unreadable(String),
    /// The queue came back in a shape this screen cannot read.
    UnreadableShape(String),
}

impl PrescriptionsError {
    /// What the person is told.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            PrescriptionsError::Unreadable(message)
            | PrescriptionsError::UnreadableShape(message) => message,
        }
    }
}

impl From<RepositoryError> for PrescriptionsError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::Unavailable(detail) => PrescriptionsError::Unreadable(format!(
                "The prescription queue could not be read. {detail}"
            )),
            RepositoryError::Unexpected(detail) => PrescriptionsError::UnreadableShape(format!(
                "The queue came back in a shape this screen cannot read. {detail}"
            )),
        }
    }
}
