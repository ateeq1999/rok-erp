//! The prescription queue's state machine.

use super::errors::PrescriptionsError;
use super::prescriptions_event::PrescriptionsEvent;
use super::prescriptions_state::PrescriptionsState;
use super::prescriptions_use_cases::GetPrescriptions;
use crate::features::prescriptions::data::repository::PrescriptionRepository;
use crate::features::prescriptions::domain::entities::PrescriptionQueue;

/// The queue's state, over whatever source its rows come from.
#[derive(Clone, Debug)]
pub struct PrescriptionsBloc<R>
where
    R: PrescriptionRepository,
{
    /// Where the day is read from.
    use_cases: GetPrescriptions<R>,
    /// The branch this bloc reads.
    branch_id: String,
    /// Which prescription the rail is on.
    selected: String,
    /// What is on screen.
    state: PrescriptionsState,
}

impl<R> PrescriptionsBloc<R>
where
    R: PrescriptionRepository,
{
    /// A bloc reading `branch_id` from `repository`.
    #[must_use]
    pub fn new(repository: R, branch_id: &str, selected: &str) -> Self {
        Self {
            use_cases: GetPrescriptions::new(repository),
            branch_id: branch_id.to_string(),
            selected: selected.to_string(),
            state: PrescriptionsState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &PrescriptionsState {
        &self.state
    }

    /// The branch this bloc reads.
    #[must_use]
    pub fn branch_id(&self) -> &str {
        &self.branch_id
    }

    /// Which prescription the rail is on.
    #[must_use]
    pub fn selected(&self) -> &str {
        &self.selected
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: PrescriptionsEvent) {
        match event {
            PrescriptionsEvent::Refresh => self.refresh().await,
            PrescriptionsEvent::Select { reference } => self.select(reference).await,
            PrescriptionsEvent::Load | PrescriptionsEvent::Retry => self.load().await,
        }
    }

    /// Read the day for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = PrescriptionsState::Loading;
        self.state = self.finish().await;
    }

    /// Read the day again over the queue already on screen.
    async fn refresh(&mut self) {
        self.state = match self.state.queue() {
            Some(queue) => PrescriptionsState::Refreshing {
                queue: queue.clone(),
            },
            None => PrescriptionsState::Loading,
        };
        self.state = self.finish().await;
    }

    /// Open another prescription, keeping the table on screen while it is read.
    async fn select(&mut self, reference: String) {
        self.selected = reference;
        self.refresh().await;
    }

    /// The read itself, so every path that reads goes through the use case.
    async fn finish(&mut self) -> PrescriptionsState {
        match self.read().await {
            Ok(queue) => {
                self.selected = queue.selected.to_string();
                PrescriptionsState::Loaded { queue }
            }
            Err(error) => PrescriptionsState::Error {
                message: error.message().to_string(),
            },
        }
    }

    /// The read the use case performs.
    async fn read(&self) -> Result<PrescriptionQueue, PrescriptionsError> {
        self.use_cases
            .execute(&self.branch_id, &self.selected)
            .await
    }
}
