//! The prescription being checked, and what the check found.

use std::sync::Arc;

use super::enums::Finding;

/// A word the check draws on every frame.
pub type Text = Arc<str>;

/// A word the check draws, from anything a source holds.
#[must_use]
pub fn text(value: &str) -> Text {
    Arc::from(value)
}

/// One medicine as read off the paper, with the batch that will be dispensed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Medicine {
    /// What it is called.
    pub name: Text,
    /// How many were prescribed.
    pub quantity: Text,
    /// The batch to draw from, which the check has already chosen.
    pub batch: Text,
    /// The directions as written on the prescription.
    pub directions: Text,
    /// The same directions in the words the patient is given.
    pub label_words: Text,
    /// How long the supply lasts.
    pub supply: Text,
    /// How long the batch has left.
    pub expiry: Text,
}

/// One thing the pharmacist has to confirm before dispensing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    /// What was checked.
    pub label: Text,
    /// What was found.
    pub detail: Text,
    /// What the chip says.
    pub result: Text,
    /// What the finding was.
    pub finding: Finding,
}

/// What the prescriber said on the phone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    /// What the call came to.
    pub label: Text,
    /// Whether this is the outcome that was recorded.
    pub chosen: bool,
}

/// Everything the clinical check knows about one prescription.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClinicalCheck {
    /// The prescription's reference.
    pub reference: Text,
    /// The patient it is for.
    pub patient: Text,
    /// The patient's age and sex, as asked at the counter.
    pub age: Text,
    /// Who prescribed it.
    pub prescriber: Text,
    /// Where they practice.
    pub clinic: Text,
    /// When the paper was issued.
    pub issued: Text,
    /// When the paper was scanned at the counter.
    pub scanned: Text,
    /// The medicines read off the paper.
    pub medicines: Vec<Medicine>,
    /// The checks the pharmacist works through.
    pub checks: Vec<Check>,
    /// The outcomes the call can have.
    pub outcomes: Vec<Outcome>,
    /// What the patient is told.
    pub counselling: Vec<Text>,
    /// The note that goes on the patient's record.
    pub note: Text,
    /// Who is checking.
    pub pharmacist: Text,
    /// Their initials, as the label prints them.
    pub initials: Text,
}
