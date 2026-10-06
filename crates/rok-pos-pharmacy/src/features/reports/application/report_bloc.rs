//! The reports' state machine.

use super::errors::ReportError;
use super::report_event::ReportEvent;
use super::report_state::ReportState;
use super::report_use_cases::ReadReports;
use crate::features::reports::data::repository::ReportsRepository;
use crate::features::reports::domain::entities::Reports;

/// The reports' state, over whatever source they come from.
#[derive(Clone, Debug)]
pub struct ReportBloc<R>
where
    R: ReportsRepository,
{
    /// Where the reports are read from.
    use_cases: ReadReports<R>,
    /// What is on screen.
    state: ReportState,
}

impl<R> ReportBloc<R>
where
    R: ReportsRepository,
{
    /// A bloc reading the reports from `repository`.
    #[must_use]
    pub fn new(repository: R) -> Self {
        Self {
            use_cases: ReadReports::new(repository),
            state: ReportState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &ReportState {
        &self.state
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: ReportEvent) {
        match event {
            ReportEvent::Load | ReportEvent::Retry => self.load().await,
        }
    }

    /// Read the reports for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = ReportState::Loading;
        self.state = match self.read().await {
            Ok(reports) => ReportState::Loaded {
                reports: Box::new(reports),
            },
            Err(error) => ReportState::Error {
                message: error.message().to_string(),
            },
        };
    }

    /// The read the use case performs.
    async fn read(&self) -> Result<Reports, ReportError> {
        self.use_cases.execute().await
    }
}
