//! Where the prescription queue is read from.

use std::future::Future;

use crate::features::prescriptions::domain::entities::PrescriptionQueue;

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
                format!("The prescription queue could not be read. {detail}")
            }
            RepositoryError::Unexpected(detail) => {
                format!("The queue came back in a shape this screen cannot read. {detail}")
            }
        }
    }
}

/// Today's queue for one branch.
pub trait PrescriptionRepository: Send {
    /// Read the day for `branch_id`, opening `selected` in the detail rail.
    fn get_queue(
        &self,
        branch_id: &str,
        selected: &str,
    ) -> impl Future<Output = Result<PrescriptionQueue, RepositoryError>> + Send;
}
