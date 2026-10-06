//! The reports repository, reading the board's own figures.
//!
//! The phase that stores sales, claims and movements in one place replaces
//! this with the pharmacy's database. The `BLoC` only knows the trait, so
//! nothing above this file changes when it does.

use std::future::Future;

use super::repository::{ReportsRepository, RepositoryError};
use super::story;
use crate::features::reports::domain::entities::Reports;

/// Reads the reports from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryReportsRepository;

impl ReportsRepository for StoryReportsRepository {
    fn get_reports(&self) -> impl Future<Output = Result<Reports, RepositoryError>> + Send {
        std::future::ready(Ok(Reports::from(story::board())))
    }
}

/// A source that is not there yet, so the error state can be drawn and tested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnavailableReportsRepository {
    /// Why the source cannot be read.
    pub detail: &'static str,
}

impl ReportsRepository for UnavailableReportsRepository {
    fn get_reports(&self) -> impl Future<Output = Result<Reports, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}
