//! The application's own error, which is what the screen draws.

use crate::features::dashboard::data::repository::RepositoryError;

/// A read that did not work, in the words the screen shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DashboardError {
    /// The figures could not be read.
    Unreadable(String),
    /// The figures came back in a shape this screen cannot read.
    UnreadableShape(String),
}

impl DashboardError {
    /// What the person is told.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            DashboardError::Unreadable(message) | DashboardError::UnreadableShape(message) => {
                message
            }
        }
    }
}

impl From<RepositoryError> for DashboardError {
    fn from(error: RepositoryError) -> Self {
        match error {
            RepositoryError::Unavailable(detail) => {
                DashboardError::Unreadable(format!("The dashboard could not be read. {detail}"))
            }
            RepositoryError::Unexpected(detail) => DashboardError::UnreadableShape(format!(
                "The dashboard came back in a shape this screen cannot read. {detail}"
            )),
        }
    }
}
