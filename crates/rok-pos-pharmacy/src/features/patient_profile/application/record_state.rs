//! What is on the patient's record right now.

use crate::features::patient_profile::domain::entities::Patient;

/// Everything the record page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum RecordState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight.
    Loading,
    /// The patient is on screen. The record is boxed so carrying it through a
    /// state transition stays a move of a pointer, not of every medicine,
    /// fill and note on it.
    Loaded {
        /// The record.
        patient: Box<Patient>,
    },
    /// The last read failed, so there is no record.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl RecordState {
    /// The patient, when there is one on screen.
    #[must_use]
    pub const fn patient(&self) -> Option<&Patient> {
        match self {
            RecordState::Loaded { patient } => Some(patient),
            RecordState::Initial | RecordState::Loading | RecordState::Error { .. } => None,
        }
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            RecordState::Error { message } => Some(message),
            _ => None,
        }
    }
}
