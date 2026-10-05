//! What is on the licences screen right now.

use crate::features::licences::domain::entities::Licences;

/// Everything the licences page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum LicenceState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight and there is no folder to keep.
    Loading,
    /// The folder is on screen.
    Loaded {
        /// The licences, the checklist and the documents.
        licences: Box<Licences>,
    },
    /// The last read failed, so there is no folder.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl LicenceState {
    /// The folder, when there is one on screen.
    #[must_use]
    pub const fn licences(&self) -> Option<&Licences> {
        match self {
            LicenceState::Loaded { licences } => Some(licences),
            LicenceState::Initial | LicenceState::Loading | LicenceState::Error { .. } => None,
        }
    }

    /// Whether a read is in flight.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        matches!(self, LicenceState::Loading)
    }

    /// Whether the folder is on screen.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, LicenceState::Loaded { .. })
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            LicenceState::Error { message } => Some(message),
            _ => None,
        }
    }
}
