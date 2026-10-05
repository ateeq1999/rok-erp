//! What is on the dashboard screen right now.
//!
//! Each variant is something the screen draws differently, so a refresh that
//! keeps the board is its own state rather than a loading board with figures
//! missing out of it.

use crate::features::dashboard::domain::entities::Dashboard;

/// Everything the page needs to draw, and nothing else.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum DashboardState {
    /// Nothing has been asked for yet.
    #[default]
    Initial,
    /// The first read is in flight and there is no board to keep.
    Loading,
    /// A read is in flight over a board that is already on screen.
    Refreshing {
        /// The board being kept while the read runs.
        dashboard: Dashboard,
    },
    /// The day is on screen.
    Loaded {
        /// The branch's day.
        dashboard: Dashboard,
    },
    /// The last read failed, so there is no board.
    Error {
        /// What the person is told.
        message: String,
    },
}

impl DashboardState {
    /// The board, when there is one to keep on screen.
    #[must_use]
    pub const fn dashboard(&self) -> Option<&Dashboard> {
        match self {
            DashboardState::Refreshing { dashboard } | DashboardState::Loaded { dashboard } => {
                Some(dashboard)
            }
            DashboardState::Initial | DashboardState::Loading | DashboardState::Error { .. } => {
                None
            }
        }
    }

    /// Whether a read is in flight.
    #[must_use]
    pub const fn is_busy(&self) -> bool {
        matches!(
            self,
            DashboardState::Loading | DashboardState::Refreshing { .. }
        )
    }

    /// Whether the day is on screen.
    #[must_use]
    pub const fn is_loaded(&self) -> bool {
        matches!(self, DashboardState::Loaded { .. })
    }

    /// What the person is told when the last read failed.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        match self {
            DashboardState::Error { message } => Some(message),
            _ => None,
        }
    }
}
