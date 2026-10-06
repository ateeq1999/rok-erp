//! Where the reports board is read from.

use std::future::Future;

use crate::features::reports::domain::entities::Reports;

/// A read that did not work. The source's own error stops here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryError {
    /// The source could not be reached.
    Unavailable(String),
    /// The source answered with something this feature cannot read.
    Unexpected(String),
}

/// The pharmacy's reports, as one reading.
pub trait ReportsRepository: Send {
    /// Read the reports on the filters the screen opens with.
    fn get_reports(&self) -> impl Future<Output = Result<Reports, RepositoryError>> + Send;
}
