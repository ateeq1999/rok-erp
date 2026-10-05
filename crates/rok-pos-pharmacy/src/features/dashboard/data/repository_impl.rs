//! The dashboard's repository, reading the board's own figures.
//!
//! This is the implementation Phase 13 replaces with the queries. It lives in
//! the data layer, so the `BLoC` and the screen never change when it does: the
//! `BLoC` only knows [`DashboardRepository`].

use std::future::Future;

use super::models::DashboardRecord;
use super::repository::{DashboardRepository, RepositoryError};
use super::story;
use crate::features::dashboard::domain::entities::Dashboard;

/// Reads a branch's day from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryDashboardRepository;

impl DashboardRepository for StoryDashboardRepository {
    fn get_dashboard(
        &self,
        branch_id: &str,
    ) -> impl Future<Output = Result<Dashboard, RepositoryError>> + Send {
        let mut record = story::mwenge_day();
        record.branch = branch_id.to_string();
        std::future::ready(Ok(into_dashboard(record)))
    }
}

/// The record as the domain entity it converts into.
fn into_dashboard(record: DashboardRecord) -> Dashboard {
    Dashboard::from(record)
}

/// A source that is not there yet, so the screen can be drawn with its error
/// state rather than an empty board.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnavailableDashboardRepository {
    /// Why the source cannot be read.
    pub detail: &'static str,
}

impl DashboardRepository for UnavailableDashboardRepository {
    fn get_dashboard(
        &self,
        _branch_id: &str,
    ) -> impl Future<Output = Result<Dashboard, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}
