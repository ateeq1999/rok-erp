//! What is on the reports screen right now.

use crate::features::reports::domain::entities::Reports;

/// Everything the reports page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ReportState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight and there is nothing to keep.
    Loading,
    /// The reports are on screen.
    Loaded {
        /// The five reports, on the filters the screen opens with.
        reports: Box<Reports>,
    },
    /// The last read failed, so there are no reports.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl ReportState {
    /// The reports, when they are on screen.
    #[must_use]
    pub const fn reports(&self) -> Option<&Reports> {
        match self {
            ReportState::Loaded { reports } => Some(reports),
            ReportState::Initial | ReportState::Loading | ReportState::Error { .. } => None,
        }
    }

    /// Whether a read is in flight.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        matches!(self, ReportState::Loading)
    }

    /// Whether the reports are on screen.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, ReportState::Loaded { .. })
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            ReportState::Error { message } => Some(message),
            _ => None,
        }
    }
}
