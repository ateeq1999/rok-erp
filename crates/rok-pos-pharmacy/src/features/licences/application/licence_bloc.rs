//! The licences folder's state machine.

use super::errors::LicenceError;
use super::licence_event::LicenceEvent;
use super::licence_state::LicenceState;
use super::licence_use_cases::ReadLicences;
use crate::features::licences::data::repository::LicenceRepository;
use crate::features::licences::domain::entities::Licences;

/// The folder's state, over whatever source it comes from.
#[derive(Clone, Debug)]
pub struct LicenceBloc<R>
where
    R: LicenceRepository,
{
    /// Where the folder is read from.
    use_cases: ReadLicences<R>,
    /// What is on screen.
    state: LicenceState,
}

impl<R> LicenceBloc<R>
where
    R: LicenceRepository,
{
    /// A bloc reading the folder from `repository`.
    #[must_use]
    pub fn new(repository: R) -> Self {
        Self {
            use_cases: ReadLicences::new(repository),
            state: LicenceState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &LicenceState {
        &self.state
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: LicenceEvent) {
        match event {
            LicenceEvent::Load | LicenceEvent::Retry => self.load().await,
        }
    }

    /// Read the folder for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = LicenceState::Loading;
        self.state = match self.read().await {
            Ok(licences) => LicenceState::Loaded {
                licences: Box::new(licences),
            },
            Err(error) => LicenceState::Error {
                message: error.message().to_string(),
            },
        };
    }

    /// The read the use case performs.
    async fn read(&self) -> Result<Licences, LicenceError> {
        self.use_cases.execute().await
    }
}
