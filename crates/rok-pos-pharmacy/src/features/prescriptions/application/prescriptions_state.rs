//! What is on the prescription queue right now.

use crate::features::prescriptions::domain::entities::PrescriptionQueue;

/// Everything the page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PrescriptionsState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight and there is no queue to keep.
    Loading,
    /// A read is in flight over a queue that is already on screen.
    Refreshing {
        /// The queue being kept while the read runs.
        queue: PrescriptionQueue,
    },
    /// The day is on screen.
    Loaded {
        /// Today's queue.
        queue: PrescriptionQueue,
    },
    /// The last read failed, so there is no queue.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl PrescriptionsState {
    /// The queue, when there is one to keep on screen.
    #[must_use]
    pub const fn queue(&self) -> Option<&PrescriptionQueue> {
        match self {
            PrescriptionsState::Refreshing { queue } | PrescriptionsState::Loaded { queue } => {
                Some(queue)
            }
            PrescriptionsState::Initial
            | PrescriptionsState::Loading
            | PrescriptionsState::Error { .. } => None,
        }
    }

    /// Whether a read is in flight.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        matches!(
            self,
            PrescriptionsState::Loading | PrescriptionsState::Refreshing { .. }
        )
    }

    /// Whether the day is on screen.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, PrescriptionsState::Loaded { .. })
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            PrescriptionsState::Error { message } => Some(message),
            _ => None,
        }
    }
}
