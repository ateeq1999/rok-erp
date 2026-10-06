//! What can happen to the reports.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportEvent {
    /// Read the reports for the first time.
    Load,
    /// Read again after the last read failed.
    Retry,
}
