//! Where one prescription's check is read from.

use std::future::Future;

use crate::features::clinical_check::domain::entities::ClinicalCheck;

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
                format!("The prescription could not be read. {detail}")
            }
            RepositoryError::Unexpected(detail) => {
                format!("The check came back in a shape this screen cannot read. {detail}")
            }
        }
    }
}

/// One prescription's check.
pub trait ClinicalCheckRepository: Send {
    /// Read the check for `prescription_id` at `branch_id`.
    fn get_check(
        &self,
        branch_id: &str,
        prescription_id: &str,
    ) -> impl Future<Output = Result<ClinicalCheck, RepositoryError>> + Send;
}
