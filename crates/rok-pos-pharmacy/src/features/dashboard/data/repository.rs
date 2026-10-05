//! Where the dashboard's figures are read from.
//!
//! The `BLoC` depends on this trait and never on what implements it. The method
//! returns a future rather than taking one, so an implementation can be written
//! as `async fn` without a boxed future or a proc-macro.

use std::future::Future;

use crate::features::dashboard::domain::entities::Dashboard;

/// A read that did not work.
///
/// The source's own error stops here: the application layer maps it into
/// something the screen can draw.
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
                format!("The dashboard figures could not be read. {detail}")
            }
            RepositoryError::Unexpected(detail) => {
                format!(
                    "The dashboard figures came back in a shape this screen cannot read. {detail}"
                )
            }
        }
    }
}

/// A day's figures for one branch.
pub trait DashboardRepository: Send {
    /// Read the day for `branch_id`.
    fn get_dashboard(
        &self,
        branch_id: &str,
    ) -> impl Future<Output = Result<Dashboard, RepositoryError>> + Send;
}
