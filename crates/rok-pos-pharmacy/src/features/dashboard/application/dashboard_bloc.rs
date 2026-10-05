//! The dashboard's state machine.
//!
//! It takes an event, reads the day through the use case, and leaves a state
//! behind. It draws nothing, names no route and knows nothing about the source
//! beyond the trait it is given.

use super::dashboard_event::DashboardEvent;
use super::dashboard_state::DashboardState;
use super::dashboard_use_cases::GetDashboard;
use crate::features::dashboard::data::repository::DashboardRepository;
use crate::features::dashboard::domain::entities::Dashboard;

/// The dashboard's state, over whatever source its figures come from.
#[derive(Clone, Debug)]
pub struct DashboardBloc<R>
where
    R: DashboardRepository,
{
    /// Where the day is read from.
    use_cases: GetDashboard<R>,
    /// The branch this bloc reads.
    branch_id: String,
    /// What is on screen.
    state: DashboardState,
}

impl<R> DashboardBloc<R>
where
    R: DashboardRepository,
{
    /// A bloc reading `branch_id` from `repository`.
    #[must_use]
    pub fn new(repository: R, branch_id: &str) -> Self {
        Self {
            use_cases: GetDashboard::new(repository),
            branch_id: branch_id.to_string(),
            state: DashboardState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &DashboardState {
        &self.state
    }

    /// The branch this bloc reads.
    #[must_use]
    pub fn branch_id(&self) -> &str {
        &self.branch_id
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: DashboardEvent) {
        match event {
            DashboardEvent::Refresh => self.refresh().await,
            DashboardEvent::ChangeBranch { branch_id } => self.change_branch(branch_id).await,
            DashboardEvent::Load | DashboardEvent::Retry => self.load().await,
        }
    }

    /// Read the day for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = DashboardState::Loading;
        self.state = match self.read().await {
            Ok(dashboard) => DashboardState::Loaded { dashboard },
            Err(error) => DashboardState::Error {
                message: error.message().to_string(),
            },
        };
    }

    /// Read the day again over the board already on screen.
    async fn refresh(&mut self) {
        self.state = match self.state.dashboard() {
            Some(dashboard) => DashboardState::Refreshing {
                dashboard: dashboard.clone(),
            },
            None => DashboardState::Loading,
        };
        self.state = match self.read().await {
            Ok(dashboard) => DashboardState::Loaded { dashboard },
            Err(error) => DashboardState::Error {
                message: error.message().to_string(),
            },
        };
    }

    /// Read a different branch's day.
    async fn change_branch(&mut self, branch_id: String) {
        self.branch_id = branch_id;
        self.load().await;
    }

    /// The read itself, so every path that reads goes through the use case.
    async fn read(&self) -> Result<Dashboard, super::errors::DashboardError> {
        self.use_cases.execute(&self.branch_id).await
    }
}
