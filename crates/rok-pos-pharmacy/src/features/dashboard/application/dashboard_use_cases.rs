//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::DashboardError;
use crate::features::dashboard::data::repository::DashboardRepository;
use crate::features::dashboard::domain::entities::Dashboard;

/// Read a branch's day, turning a source's failure into the error the screen
/// draws.
#[derive(Clone, Debug)]
pub struct GetDashboard<R> {
    /// Where the figures come from.
    repository: R,
}

impl<R> GetDashboard<R>
where
    R: DashboardRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the day for `branch_id`.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(&self, branch_id: &str) -> Result<Dashboard, DashboardError> {
        self.repository
            .get_dashboard(branch_id)
            .await
            .map_err(DashboardError::from)
    }
}
