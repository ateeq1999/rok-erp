//! What can happen to the dashboard.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DashboardEvent {
    /// Read the day for the first time. The board is empty until it answers.
    Load,
    /// Read the day again, keeping the board on screen while it does.
    Refresh,
    /// Read a different branch's day.
    ChangeBranch {
        /// The branch to read.
        branch_id: String,
    },
    /// Read again after the last read failed.
    Retry,
}

impl DashboardEvent {
    /// Whether this event keeps what is already on screen.
    ///
    /// A refresh does, so the screen has its own state for it rather than
    /// emptying the page back to a loading board.
    #[must_use]
    pub const fn keeps_the_board(&self) -> bool {
        matches!(self, DashboardEvent::Refresh)
    }
}
