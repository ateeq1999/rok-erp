//! The records a source hands over for a check, and the entity they become.

use crate::features::clinical_check::domain::entities::{
    Check, ClinicalCheck, Medicine, Outcome, text,
};
use crate::features::clinical_check::domain::enums::Finding;

/// One medicine, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MedicineRecord {
    /// What it is called.
    pub name: String,
    /// How many were prescribed.
    pub quantity: String,
    /// The batch to draw from.
    pub batch: String,
    /// The directions as written on the prescription.
    pub directions: String,
    /// The same directions in the words the patient is given.
    pub label_words: String,
    /// How long the supply lasts.
    pub supply: String,
    /// How long the batch has left.
    pub expiry: String,
}

/// One check, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckRecord {
    /// What was checked.
    pub label: String,
    /// What was found.
    pub detail: String,
    /// What the chip says.
    pub result: String,
    /// What the finding was, as the source names it.
    pub finding: String,
}

/// One call outcome, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutcomeRecord {
    /// What the call came to.
    pub label: String,
    /// Whether it was recorded.
    pub chosen: bool,
}

/// The check, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckRecordSet {
    /// The prescription's reference.
    pub reference: String,
    /// The patient it is for.
    pub patient: String,
    /// The patient's age and sex.
    pub age: String,
    /// Who prescribed it.
    pub prescriber: String,
    /// Where they practice.
    pub clinic: String,
    /// When the paper was issued.
    pub issued: String,
    /// When the paper was scanned.
    pub scanned: String,
    /// The medicines read off the paper.
    pub medicines: Vec<MedicineRecord>,
    /// The checks the pharmacist works through.
    pub checks: Vec<CheckRecord>,
    /// The outcomes the call can have.
    pub outcomes: Vec<OutcomeRecord>,
    /// What the patient is told.
    pub counselling: Vec<String>,
    /// The note for the patient's record.
    pub note: String,
    /// Who is checking.
    pub pharmacist: String,
    /// Their initials.
    pub initials: String,
}

/// What a source calls a finding.
fn finding(finding: &str) -> Finding {
    match finding {
        "Alert" => Finding::Alert,
        _ => Finding::Clear,
    }
}

impl From<MedicineRecord> for Medicine {
    fn from(record: MedicineRecord) -> Self {
        Self {
            name: text(&record.name),
            quantity: text(&record.quantity),
            batch: text(&record.batch),
            directions: text(&record.directions),
            label_words: text(&record.label_words),
            supply: text(&record.supply),
            expiry: text(&record.expiry),
        }
    }
}

impl From<CheckRecord> for Check {
    fn from(record: CheckRecord) -> Self {
        Self {
            label: text(&record.label),
            detail: text(&record.detail),
            result: text(&record.result),
            finding: finding(&record.finding),
        }
    }
}

impl From<OutcomeRecord> for Outcome {
    fn from(record: OutcomeRecord) -> Self {
        Self {
            label: text(&record.label),
            chosen: record.chosen,
        }
    }
}

impl From<CheckRecordSet> for ClinicalCheck {
    fn from(record: CheckRecordSet) -> Self {
        Self {
            reference: text(&record.reference),
            patient: text(&record.patient),
            age: text(&record.age),
            prescriber: text(&record.prescriber),
            clinic: text(&record.clinic),
            issued: text(&record.issued),
            scanned: text(&record.scanned),
            medicines: record.medicines.into_iter().map(Medicine::from).collect(),
            checks: record.checks.into_iter().map(Check::from).collect(),
            outcomes: record.outcomes.into_iter().map(Outcome::from).collect(),
            counselling: record.counselling.iter().map(|point| text(point)).collect(),
            note: text(&record.note),
            pharmacist: text(&record.pharmacist),
            initials: text(&record.initials),
        }
    }
}
