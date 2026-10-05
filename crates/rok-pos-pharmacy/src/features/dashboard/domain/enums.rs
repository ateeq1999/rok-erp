//! What kinds of thing the dashboard talks about.

/// What a row of "Needs you now" is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskKind {
    /// A batch recall is open and a patient may still hold the medicine.
    Recall,
    /// A clinical check has found something a pharmacist has to resolve.
    Check,
    /// A delivery is at the door and not yet received.
    Delivery,
    /// A licence is close to running out.
    Licence,
    /// The day's work is already done, which is why the row stays quiet.
    Done,
}

impl TaskKind {
    /// The chip the row carries.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            TaskKind::Recall => "Recall",
            TaskKind::Check => "Check",
            TaskKind::Delivery => "Delivery",
            TaskKind::Licence => "Licence",
            TaskKind::Done => "Done",
        }
    }

    /// Whether this kind of work is finished for the day.
    ///
    /// A finished row is a fact about the day, not a style choice, so it lives
    /// here rather than in the screen that draws it quietly.
    #[must_use]
    pub const fn is_done(self) -> bool {
        matches!(self, TaskKind::Done)
    }
}

/// How serious a task is, which the screen turns into a colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskSeverity {
    /// Someone may already have taken the medicine: stop.
    Danger,
    /// Time is running out, or a call has to be made.
    Warning,
    /// Worth knowing about, with no deadline attached.
    Info,
    /// Finished.
    Success,
}

/// Where a tile or a task goes when it is followed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Destination {
    /// The sales and margin reports.
    Reports,
    /// The prescription queue.
    Prescriptions,
    /// One prescription's clinical check.
    PrescriptionCheck,
    /// The batches and expiry board.
    Batches,
    /// The insurance claims board.
    Claims,
    /// One open recall.
    Recall,
    /// Receiving deliveries.
    Receive,
    /// Licences and inspection readiness.
    Licences,
    /// The controlled register.
    ControlledRegister,
}
