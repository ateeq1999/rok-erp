//! What kinds of thing the clinical check talks about.

/// What a check found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finding {
    /// Nothing that stops the prescription: it can be approved once the call
    /// behind any other finding is recorded.
    Clear,
    /// Something a pharmacist has to resolve before it is dispensed.
    Alert,
}

impl Finding {
    /// How much attention the finding is asking for, which the screen turns into
    /// a colour.
    #[must_use]
    pub const fn severity(self) -> Severity {
        match self {
            Finding::Clear => Severity::Clear,
            Finding::Alert => Severity::Alert,
        }
    }

    /// Whether the finding stops the prescription.
    #[must_use]
    pub const fn blocks(self) -> bool {
        matches!(self, Finding::Alert)
    }
}

/// How much attention a finding is asking for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Nothing to act on.
    Clear,
    /// A stop until it is resolved.
    Alert,
}

/// Where the check's rows and buttons go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Destination {
    /// One patient's record.
    PatientRecord,
    /// The prescription queue.
    Prescriptions,
    /// The dispensary till, where an approved prescription lands.
    Till,
}
