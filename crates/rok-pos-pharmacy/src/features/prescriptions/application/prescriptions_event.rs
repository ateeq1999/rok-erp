//! What can happen to the prescription queue.

/// Something the person did, or something the screen needs to know about.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrescriptionsEvent {
    /// Read the day for the first time.
    Load,
    /// Read the day again, keeping the queue on screen while it does.
    Refresh,
    /// Open another prescription in the detail rail.
    Select {
        /// The reference to open.
        reference: String,
    },
    /// Read again after the last read failed.
    Retry,
}

impl PrescriptionsEvent {
    /// Whether this event keeps what is already on screen.
    #[must_use]
    pub const fn keeps_the_queue(&self) -> bool {
        matches!(
            self,
            PrescriptionsEvent::Refresh | PrescriptionsEvent::Select { .. }
        )
    }
}
