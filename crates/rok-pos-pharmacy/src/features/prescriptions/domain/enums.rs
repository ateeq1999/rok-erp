//! What kinds of thing the prescription queue talks about.

/// Where a prescription has got to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    /// Received, not yet read into the system.
    New,
    /// Read, waiting for the pharmacist's clinical check.
    NeedsCheck,
    /// The prescriber has to answer before it can go on.
    WaitingPrescriber,
    /// Checked, packed and waiting for the patient.
    ReadyToCollect,
    /// Handed over today.
    Dispensed,
}

impl Status {
    /// Every state a prescription passes through, in the board's order.
    pub const ALL: [Status; 5] = [
        Status::New,
        Status::NeedsCheck,
        Status::WaitingPrescriber,
        Status::ReadyToCollect,
        Status::Dispensed,
    ];

    /// The word the queue's status chip shows.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Status::New => "New",
            Status::NeedsCheck => "Needs pharmacist check",
            Status::WaitingPrescriber => "Waiting for prescriber",
            Status::ReadyToCollect => "Ready to collect",
            Status::Dispensed => "Dispensed today",
        }
    }

    /// How much attention this state is asking for.
    ///
    /// A prescription that has left the pharmacy is finished, not successful:
    /// the fact that it is done is what the pharmacist needs to read.
    #[must_use]
    pub const fn severity(self) -> StatusSeverity {
        match self {
            Status::New => StatusSeverity::Info,
            Status::NeedsCheck => StatusSeverity::Warning,
            Status::WaitingPrescriber => StatusSeverity::Danger,
            Status::ReadyToCollect => StatusSeverity::Brand,
            Status::Dispensed => StatusSeverity::Done,
        }
    }

    /// Whether the prescription has left the pharmacy.
    #[must_use]
    pub const fn is_finished(self) -> bool {
        matches!(self, Status::Dispensed)
    }
}

/// How much attention a state is asking for, which the screen turns into a
/// colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusSeverity {
    /// Worth knowing, with no deadline attached.
    Info,
    /// The pharmacist's check is the next thing that has to happen.
    Warning,
    /// Someone is waiting on a phone call.
    Danger,
    /// Packed and waiting for the patient.
    Brand,
    /// Gone: handed over today.
    Done,
}

/// Where a prescription came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Handed over on paper at the counter.
    Paper,
    /// Photographed and sent on `WhatsApp`.
    Photo,
    /// Fetched from a prescriber by its code.
    Electronic,
}

impl Source {
    /// The word the queue's source column shows.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Source::Paper => "Paper at counter",
            Source::Photo => "WhatsApp photo",
            Source::Electronic => "E-prescription",
        }
    }

    /// Whether the source still has to be read by a person.
    #[must_use]
    pub const fn needs_reading(self) -> bool {
        matches!(self, Source::Photo)
    }
}

/// Where the queue's rows go, and what the detail rail links to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Destination {
    /// One prescription's clinical check.
    Check,
    /// One patient's record.
    PatientRecord,
}
