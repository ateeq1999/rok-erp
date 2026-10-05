//! The states a licence or a document can be in, in the words the board uses.

/// How a licence stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Standing {
    /// Held, but the renewal is due.
    #[default]
    Renewing,
    /// Held and valid.
    Valid,
    /// Not held yet.
    Missing,
}

impl Standing {
    /// Whether the licence needs the pharmacy to act.
    #[must_use]
    pub const fn needs_renewal(self) -> bool {
        !matches!(self, Self::Valid)
    }
}

/// When the folder reminds the pharmacy about a document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reminder {
    /// A set number of days before the document expires.
    DaysBefore(u32),
    /// The date is a submission deadline, not an expiry.
    SubmitBy,
}
