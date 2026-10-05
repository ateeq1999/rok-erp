//! The application's own error, which is what the screen draws.

use crate::features::clinical_check::data::repository::RepositoryError;

/// A read that did not work, in the words the screen shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckError {
    /// The prescription could not be read.
    Unreadable(String),
    /// The check came back in a shape this screen cannot read.
    UnreadableShape(String),
}

impl CheckError {
    /// What the person is told.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            CheckError::Unreadable(message) | CheckError::UnreadableShape(message) => message,
        }
    }
}

impl From<RepositoryError> for CheckError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::Unavailable(detail) => {
                CheckError::Unreadable(format!("The prescription could not be read. {detail}"))
            }
            RepositoryError::Unexpected(detail) => CheckError::UnreadableShape(format!(
                "The check came back in a shape this screen cannot read. {detail}"
            )),
        }
    }
}
