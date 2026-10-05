//! The few states a patient's row or record can be in.

/// Who pays for a patient, as the list splits them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cover {
    /// An insurer covers them.
    Insurance,
    /// They pay at the counter.
    Cash,
}

impl Cover {
    /// Whether the insurer is billed, so the counter is not.
    #[must_use]
    pub const fn is_insured(self) -> bool {
        matches!(self, Cover::Insurance)
    }

    /// What the list calls it in the cover column.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Cover::Insurance => "Insurance",
            Cover::Cash => "Cash",
        }
    }
}

/// Whether the patient gets refill reminders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reminders {
    /// They do, and the record says on what terms.
    On,
    /// They do not.
    Off,
}

impl Reminders {
    /// Whether reminders are on.
    #[must_use]
    pub const fn is_on(self) -> bool {
        matches!(self, Reminders::On)
    }
}

/// How soon a patient's next refill is due, as the list tints the row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Urgency {
    /// Nothing is due.
    Nothing,
    /// Due within the week, so the row asks for attention.
    ThisWeek,
    /// Due later, so the row is noted rather than urgent.
    Planned,
}

impl Urgency {
    /// Whether the row asks for attention today.
    #[must_use]
    pub const fn is_this_week(self) -> bool {
        matches!(self, Urgency::ThisWeek)
    }
}
