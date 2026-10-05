//! The clinical check's state machine.

use super::check_event::CheckEvent;
use super::check_state::CheckState;
use super::check_use_cases::GetClinicalCheck;
use super::errors::CheckError;
use crate::features::clinical_check::data::repository::ClinicalCheckRepository;
use crate::features::clinical_check::domain::entities::ClinicalCheck;

/// The check's state, over whatever source its findings come from.
#[derive(Clone, Debug)]
pub struct CheckBloc<R>
where
    R: ClinicalCheckRepository,
{
    /// Where the prescription is read from.
    use_cases: GetClinicalCheck<R>,
    /// The branch this bloc reads.
    branch_id: String,
    /// Which prescription it is checking.
    prescription_id: String,
    /// What is on screen.
    state: CheckState,
}

impl<R> CheckBloc<R>
where
    R: ClinicalCheckRepository,
{
    /// A bloc checking `prescription_id` at `branch_id`.
    #[must_use]
    pub fn new(repository: R, branch_id: &str, prescription_id: &str) -> Self {
        Self {
            use_cases: GetClinicalCheck::new(repository),
            branch_id: branch_id.to_string(),
            prescription_id: prescription_id.to_string(),
            state: CheckState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &CheckState {
        &self.state
    }

    /// The branch this bloc reads.
    #[must_use]
    pub fn branch_id(&self) -> &str {
        &self.branch_id
    }

    /// Which prescription it is checking.
    #[must_use]
    pub fn prescription_id(&self) -> &str {
        &self.prescription_id
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: CheckEvent) {
        match event {
            CheckEvent::Refresh => self.refresh().await,
            CheckEvent::Load | CheckEvent::Retry => self.load().await,
        }
    }

    /// Read the prescription for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = CheckState::Loading;
        self.state = self.finish().await;
    }

    /// Read it again over the check already on screen.
    async fn refresh(&mut self) {
        self.state = match self.state.check() {
            Some(check) => CheckState::Refreshing {
                check: check.clone(),
            },
            None => CheckState::Loading,
        };
        self.state = self.finish().await;
    }

    /// The read itself, so every path that reads goes through the use case.
    async fn finish(&mut self) -> CheckState {
        match self.read().await {
            Ok(check) => {
                self.prescription_id = check.reference.to_string();
                CheckState::Loaded { check }
            }
            Err(error) => CheckState::Error {
                message: error.message().to_string(),
            },
        }
    }

    /// The read the use case performs.
    async fn read(&self) -> Result<ClinicalCheck, CheckError> {
        self.use_cases
            .execute(&self.branch_id, &self.prescription_id)
            .await
    }
}
