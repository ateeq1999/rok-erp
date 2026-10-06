//! The application's own error, which is what the screen draws.

use crate::features::reports::data::repository::RepositoryError;

/// A read that did not work, in the words the screen shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReportError {
    /// The reports could not be read.
    Unreadable(String),
    /// They came back in a shape this screen cannot read.
    UnreadableShape(String),
}

impl ReportError {
    /// What the person is told.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            ReportError::Unreadable(message) | ReportError::UnreadableShape(message) => message,
        }
    }
}

impl From<RepositoryError> for ReportError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::Unavailable(detail) => {
                ReportError::Unreadable(format!("The reports could not be read. {detail}"))
            }
            RepositoryError::Unexpected(detail) => ReportError::UnreadableShape(format!(
                "The reports came back in a shape this screen cannot read. {detail}"
            )),
        }
    }
}
