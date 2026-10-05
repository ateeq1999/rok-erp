//! What can happen to the licences folder.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LicenceEvent {
    /// Read the folder for the first time.
    Load,
    /// Read again after the last read failed.
    Retry,
}
