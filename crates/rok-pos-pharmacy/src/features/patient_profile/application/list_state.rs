//! What is on the patient list right now.

use crate::features::patient_profile::domain::entities::Listing;

/// Everything the list page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ListState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight and there is no list to keep.
    Loading,
    /// A read is in flight over a list that is already on screen.
    Refreshing {
        /// The list being kept while the read runs.
        listings: Vec<Listing>,
    },
    /// The directory is on screen.
    Loaded {
        /// The pharmacy's patients, most urgent first.
        listings: Vec<Listing>,
    },
    /// The last read failed, so there is no list.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl ListState {
    /// The list, when there is one to keep on screen.
    #[must_use]
    pub const fn listings(&self) -> Option<&Vec<Listing>> {
        match self {
            ListState::Refreshing { listings } | ListState::Loaded { listings } => Some(listings),
            ListState::Initial | ListState::Loading | ListState::Error { .. } => None,
        }
    }

    /// Whether a read is in flight.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        matches!(self, ListState::Loading | ListState::Refreshing { .. })
    }

    /// Whether the directory is on screen.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, ListState::Loaded { .. })
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            ListState::Error { message } => Some(message),
            _ => None,
        }
    }
}
