//! The patient list's state machine.

use super::errors::PatientError;
use super::list_event::ListEvent;
use super::list_state::ListState;
use super::list_use_cases::ListDirectory;
use crate::features::patient_profile::data::repository::DirectoryRepository;
use crate::features::patient_profile::domain::entities::Listing;

/// The list's state, over whatever source it comes from.
#[derive(Clone, Debug)]
pub struct ListBloc<R>
where
    R: DirectoryRepository,
{
    /// Where the list is read from.
    use_cases: ListDirectory<R>,
    /// What is on screen.
    state: ListState,
}

impl<R> ListBloc<R>
where
    R: DirectoryRepository,
{
    /// A bloc reading the directory from `repository`.
    #[must_use]
    pub fn new(repository: R) -> Self {
        Self {
            use_cases: ListDirectory::new(repository),
            state: ListState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &ListState {
        &self.state
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: ListEvent) {
        match event {
            ListEvent::Load | ListEvent::Retry => self.load().await,
            ListEvent::Refresh => self.refresh().await,
        }
    }

    /// Read the list for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = ListState::Loading;
        self.state = self.finish().await;
    }

    /// Read the list again over the rows already on screen.
    async fn refresh(&mut self) {
        self.state = match self.state.listings() {
            Some(listings) => ListState::Refreshing {
                listings: listings.clone(),
            },
            None => ListState::Loading,
        };
        self.state = self.finish().await;
    }

    /// The read itself, so every path that reads goes through the use case.
    async fn finish(&mut self) -> ListState {
        match self.read().await {
            Ok(listings) => ListState::Loaded { listings },
            Err(error) => ListState::Error {
                message: error.message().to_string(),
            },
        }
    }

    /// The read the use case performs.
    async fn read(&self) -> Result<Vec<Listing>, PatientError> {
        self.use_cases.execute().await
    }
}
