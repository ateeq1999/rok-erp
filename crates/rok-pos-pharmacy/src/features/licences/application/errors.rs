//! The application's own error, which is what the screens draw.

use crate::features::licences::data::repository::RepositoryError;

/// A read that did not work, in the words the screen shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LicenceError {
    /// The folder could not be read.
    Unreadable(String),
    /// It came back in a shape this screen cannot read.
    UnreadableShape(String),
}

impl LicenceError {
    /// What the person is told.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            LicenceError::Unreadable(message) | LicenceError::UnreadableShape(message) => message,
        }
    }
}

impl From<RepositoryError> for LicenceError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::Unavailable(detail) => {
                LicenceError::Unreadable(format!("The folder could not be read. {detail}"))
            }
            RepositoryError::Unexpected(detail) => LicenceError::UnreadableShape(format!(
                "The folder came back in a shape this screen cannot read. {detail}"
            )),
        }
    }
}
