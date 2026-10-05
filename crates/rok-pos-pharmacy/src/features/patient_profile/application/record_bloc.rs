//! The patient record's state machine.

use super::errors::PatientError;
use super::record_event::RecordEvent;
use super::record_state::RecordState;
use super::record_use_cases::GetPatient;
use crate::features::patient_profile::data::repository::PatientRepository;
use crate::features::patient_profile::domain::entities::Patient;

/// The record's state, over whatever source it comes from.
#[derive(Clone, Debug)]
pub struct RecordBloc<R>
where
    R: PatientRepository,
{
    /// Where the record is read from.
    use_cases: GetPatient<R>,
    /// The patient this bloc reads, as the route spells their id.
    patient_id: String,
    /// What is on screen.
    state: RecordState,
}

impl<R> RecordBloc<R>
where
    R: PatientRepository,
{
    /// A bloc reading `patient_id` from `repository`.
    #[must_use]
    pub fn new(repository: R, patient_id: &str) -> Self {
        Self {
            use_cases: GetPatient::new(repository),
            patient_id: patient_id.to_string(),
            state: RecordState::Initial,
        }
    }

    /// What is on screen.
    #[must_use]
    pub const fn state(&self) -> &RecordState {
        &self.state
    }

    /// The patient this bloc reads, as the route spells their id.
    #[must_use]
    pub fn patient_id(&self) -> &str {
        &self.patient_id
    }

    /// React to `event`, leaving the new state behind.
    pub async fn dispatch(&mut self, event: RecordEvent) {
        match event {
            RecordEvent::Load | RecordEvent::Retry => self.load().await,
        }
    }

    /// Read the record for the first time, or again after a failure.
    async fn load(&mut self) {
        self.state = RecordState::Loading;
        self.state = match self.read().await {
            Ok(patient) => RecordState::Loaded {
                patient: Box::new(patient),
            },
            Err(error) => RecordState::Error {
                message: error.message().to_string(),
            },
        };
    }

    /// The read the use case performs.
    async fn read(&self) -> Result<Patient, PatientError> {
        self.use_cases.execute(&self.patient_id).await
    }
}
