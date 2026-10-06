//! The application's operations, kept out of the `BLoC` so it only orchestrates.

use super::errors::ReportError;
use crate::features::reports::data::repository::ReportsRepository;
use crate::features::reports::domain::entities::Reports;

/// Read the reports, turning a source's failure into the error the screen draws.
#[derive(Clone, Debug)]
pub struct ReadReports<R> {
    /// Where the reports come from.
    repository: R,
}

impl<R> ReadReports<R>
where
    R: ReportsRepository,
{
    /// The use case over one source.
    #[must_use]
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Read the reports on the filters the screen opens with.
    ///
    /// # Errors
    ///
    /// Returns the application's error when the source cannot be read or hands
    /// back something this screen cannot read.
    pub async fn execute(&self) -> Result<Reports, ReportError> {
        self.repository
            .get_reports()
            .await
            .map_err(ReportError::from)
    }
}
