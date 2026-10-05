//! The records a source hands over for a patient and the patient list, and the
//! entities they become.

use crate::features::patient_profile::domain::entities::{
    Condition, Detail, Fill, Listing, Medicine, Note, Patient, text,
};
use crate::features::patient_profile::domain::enums::{Cover, Reminders, Urgency};

/// One fact about a patient, as a source pairs it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailRecord {
    /// What the fact is.
    pub label: String,
    /// The value beside it.
    pub value: String,
}

/// One of the patient's long-term conditions, as a source lists it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConditionRecord {
    /// What it is.
    pub name: String,
    /// Where it came from.
    pub source: String,
}

/// One medicine the patient takes, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MedicineRecord {
    /// The medicine.
    pub name: String,
    /// How it is taken.
    pub directions: String,
    /// When it was last filled.
    pub last_filled: String,
    /// When the next one is due, or who sets that.
    pub due: String,
}

/// One past fill, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FillRecord {
    /// When.
    pub date: String,
    /// Which prescription.
    pub reference: String,
    /// What was dispensed.
    pub medicines: String,
    /// Who prescribed it.
    pub prescriber: String,
    /// Who dispensed it.
    pub pharmacist: String,
    /// Where it got to.
    pub status: String,
}

/// One clinical note, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoteRecord {
    /// Its title.
    pub title: String,
    /// When it was written and by whom.
    pub meta: String,
    /// What it says.
    pub body: String,
}

/// A patient, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientRecord {
    /// Their initials, as the record's avatar shows them.
    pub initials: String,
    /// Their name.
    pub name: String,
    /// The details the record pairs off.
    pub details: Vec<DetailRecord>,
    /// Who pays for them and whether they are covered.
    pub insurer: String,
    /// Whether the insurer covers them today.
    pub cover: String,
    /// Their known allergies.
    pub allergies: String,
    /// The label the allergies panel opens with.
    pub allergies_label: String,
    /// What the record says about where allergies come from.
    pub allergy_source: String,
    /// Whether they get refill reminders, as the source writes it.
    pub reminders: String,
    /// The terms they agreed to.
    pub consent: String,
    /// Their long-term conditions.
    pub conditions: Vec<ConditionRecord>,
    /// What they take now.
    pub medicines: Vec<MedicineRecord>,
    /// Their fills, newest first.
    pub fills: Vec<FillRecord>,
    /// Their notes, newest first.
    pub notes: Vec<NoteRecord>,
}

/// One row of the patient list, as a source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListingRecord {
    /// Their record's path, as the router spells it.
    pub href: String,
    /// Their initials, as the list's avatar shows them.
    pub initials: String,
    /// Their name.
    pub name: String,
    /// The phone the pharmacy reaches them on.
    pub phone: String,
    /// Who pays, as the source names it.
    pub cover: String,
    /// Their long-term conditions, joined for the row.
    pub conditions: String,
    /// What is next for them, when something is.
    pub next_due: Option<String>,
    /// Whether the row is one to act on this week.
    pub due_this_week: bool,
}

/// What a source calls a patient's cover.
fn cover(cover: &str) -> Cover {
    if cover == "Cash" {
        Cover::Cash
    } else {
        Cover::Insurance
    }
}

/// What a source calls a patient's reminder choice.
fn reminders(reminders: &str) -> Reminders {
    if reminders == "On" {
        Reminders::On
    } else {
        Reminders::Off
    }
}

impl From<DetailRecord> for Detail {
    fn from(record: DetailRecord) -> Self {
        Self {
            label: text(&record.label),
            value: text(&record.value),
        }
    }
}

impl From<ConditionRecord> for Condition {
    fn from(record: ConditionRecord) -> Self {
        Self {
            name: text(&record.name),
            source: text(&record.source),
        }
    }
}

impl From<MedicineRecord> for Medicine {
    fn from(record: MedicineRecord) -> Self {
        Self {
            name: text(&record.name),
            directions: text(&record.directions),
            last_filled: text(&record.last_filled),
            due: text(&record.due),
        }
    }
}

impl From<FillRecord> for Fill {
    fn from(record: FillRecord) -> Self {
        Self {
            date: text(&record.date),
            reference: text(&record.reference),
            medicines: text(&record.medicines),
            prescriber: text(&record.prescriber),
            pharmacist: text(&record.pharmacist),
            status: text(&record.status),
        }
    }
}

impl From<NoteRecord> for Note {
    fn from(record: NoteRecord) -> Self {
        Self {
            title: text(&record.title),
            meta: text(&record.meta),
            body: text(&record.body),
        }
    }
}

impl From<PatientRecord> for Patient {
    fn from(record: PatientRecord) -> Self {
        Self {
            initials: text(&record.initials),
            name: text(&record.name),
            details: record.details.into_iter().map(Detail::from).collect(),
            insurer: text(&record.insurer),
            cover: text(&record.cover),
            allergies: text(&record.allergies),
            allergies_label: text(&record.allergies_label),
            allergy_source: text(&record.allergy_source),
            reminders: reminders(&record.reminders),
            consent: text(&record.consent),
            conditions: record.conditions.into_iter().map(Condition::from).collect(),
            medicines: record.medicines.into_iter().map(Medicine::from).collect(),
            fills: record.fills.into_iter().map(Fill::from).collect(),
            notes: record.notes.into_iter().map(Note::from).collect(),
        }
    }
}

impl From<ListingRecord> for Listing {
    fn from(record: ListingRecord) -> Self {
        let urgency = if record.due_this_week {
            Urgency::ThisWeek
        } else if record.next_due.is_some() {
            Urgency::Planned
        } else {
            Urgency::Nothing
        };
        Self {
            href: text(&record.href),
            initials: text(&record.initials),
            name: text(&record.name),
            phone: text(&record.phone),
            cover: cover(&record.cover),
            conditions: text(&record.conditions),
            next_due: record.next_due.map(|due| text(&due)),
            urgency,
        }
    }
}
