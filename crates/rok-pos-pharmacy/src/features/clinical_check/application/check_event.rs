//! What can happen to the clinical check.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckEvent {
    /// Read the prescription's check for the first time.
    Load,
    /// Read it again, keeping the check on screen while it does.
    Refresh,
    /// Read again after the last read failed.
    Retry,
}

impl CheckEvent {
    /// Whether this event keeps what is already on screen.
    #[must_use]
    pub const fn keeps_the_check(&self) -> bool {
        matches!(self, CheckEvent::Refresh)
    }
}
