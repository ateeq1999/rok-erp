//! What can happen to the patient list.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListEvent {
    /// Read the list for the first time.
    Load,
    /// Read it again, keeping the rows on screen while it does.
    Refresh,
    /// Read again after the last read failed.
    Retry,
}
