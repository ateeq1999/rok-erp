//! A patient the pharmacy serves, and the list their record is found from.

use std::sync::Arc;

use super::enums::{Cover, Reminders, Urgency};

/// A word the record draws on every frame.
pub type Text = Arc<str>;

/// A word the record draws, from anything a source holds.
#[must_use]
pub fn text(value: &str) -> Text {
    Arc::from(value)
}

/// One fact about a patient, as the record's details panel pairs it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Detail {
    /// What the fact is.
    pub label: Text,
    /// The value beside it.
    pub value: Text,
}

/// One of the patient's long-term conditions, as the record lists them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Condition {
    /// What it is.
    pub name: Text,
    /// Where it came from.
    pub source: Text,
}

/// One medicine the patient takes, and when it is next due.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Medicine {
    /// The medicine.
    pub name: Text,
    /// How it is taken.
    pub directions: Text,
    /// When it was last filled.
    pub last_filled: Text,
    /// When the next one is due, or who sets that: `"Due 5 Oct"` when the
    /// pharmacy counts the days, `"Set by clinic"` when it does not.
    pub due: Text,
}

impl Medicine {
    /// Whether the pharmacy refills this one on a date it knows, rather than
    /// the clinic setting the interval.
    #[must_use]
    pub fn is_a_pharmacy_refill(&self) -> bool {
        self.due.starts_with("Due")
    }
}

/// One past fill.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fill {
    /// When.
    pub date: Text,
    /// Which prescription.
    pub reference: Text,
    /// What was dispensed.
    pub medicines: Text,
    /// Who prescribed it.
    pub prescriber: Text,
    /// Who dispensed it.
    pub pharmacist: Text,
    /// Where it got to.
    pub status: Text,
}

/// One clinical note, which only a pharmacist may read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    /// Its title.
    pub title: Text,
    /// When it was written and by whom.
    pub meta: Text,
    /// What it says.
    pub body: Text,
}

/// Everything the record draws about one patient.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Patient {
    /// Their initials, as the record's avatar shows them.
    pub initials: Text,
    /// Their name.
    pub name: Text,
    /// The details the record pairs off.
    pub details: Vec<Detail>,
    /// Who pays for them and whether they are covered.
    pub insurer: Text,
    /// Whether the insurer covers them today.
    pub cover: Text,
    /// Their known allergies.
    pub allergies: Text,
    /// The label the allergies panel opens with.
    pub allergies_label: Text,
    /// What the record says about where allergies come from.
    pub allergy_source: Text,
    /// Whether they get refill reminders.
    pub reminders: Reminders,
    /// The terms they agreed to.
    pub consent: Text,
    /// Their long-term conditions.
    pub conditions: Vec<Condition>,
    /// What they take now.
    pub medicines: Vec<Medicine>,
    /// Their fills, newest first.
    pub fills: Vec<Fill>,
    /// Their notes, newest first.
    pub notes: Vec<Note>,
}

impl Patient {
    /// How many fills the record shows.
    #[must_use]
    pub fn fill_count(&self) -> u32 {
        u32::try_from(self.fills.len()).unwrap_or(u32::MAX)
    }

    /// The medicine whose refill is due soonest, which is the reason to open
    /// this record from the list.
    #[must_use]
    pub fn next_due(&self) -> Option<&Medicine> {
        self.medicines
            .iter()
            .find(|medicine| medicine.is_a_pharmacy_refill())
    }

    /// Whether the patient has consented to refill reminders.
    #[must_use]
    pub const fn wants_reminders(&self) -> bool {
        self.reminders.is_on()
    }
}

/// One row of the patient list, which is how a record is found in the first
/// place.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listing {
    /// Their record's path, as the router spells it.
    pub href: Text,
    /// Their initials, as the list's avatar shows them.
    pub initials: Text,
    /// Their name.
    pub name: Text,
    /// The phone the pharmacy reaches them on.
    pub phone: Text,
    /// Who pays, and whether they are covered today.
    pub cover: Cover,
    /// Their long-term conditions, joined for the row.
    pub conditions: Text,
    /// What is next for them, when something is.
    pub next_due: Option<Text>,
    /// How soon that is, as the list tints it.
    pub urgency: Urgency,
}

impl Listing {
    /// Whether the row is one to act on now.
    #[must_use]
    pub const fn needs_attention(&self) -> bool {
        self.urgency.is_this_week()
    }
}
