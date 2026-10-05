//! What can happen to the patient's record.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordEvent {
    /// Read the record for the first time.
    Load,
    /// Read it again after the last read failed.
    Retry,
}
