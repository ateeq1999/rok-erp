//! What is on the clinical check screen right now.

use crate::features::clinical_check::domain::entities::ClinicalCheck;

/// Everything the page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CheckState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight and there is no check to keep.
    Loading,
    /// A read is in flight over a check that is already on screen.
    Refreshing {
        /// The check being kept while the read runs.
        check: ClinicalCheck,
    },
    /// The prescription's check is on screen.
    Loaded {
        /// The check itself.
        check: ClinicalCheck,
    },
    /// The last read failed, so there is no check.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl CheckState {
    /// The check, when there is one to keep on screen.
    #[must_use]
    pub const fn check(&self) -> Option<&ClinicalCheck> {
        match self {
            CheckState::Refreshing { check } | CheckState::Loaded { check } => Some(check),
            CheckState::Initial | CheckState::Loading | CheckState::Error { .. } => None,
        }
    }

    /// Whether a read is in flight.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        matches!(self, CheckState::Loading | CheckState::Refreshing { .. })
    }

    /// Whether the check is on screen.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, CheckState::Loaded { .. })
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            CheckState::Error { message } => Some(message),
            _ => None,
        }
    }
}
