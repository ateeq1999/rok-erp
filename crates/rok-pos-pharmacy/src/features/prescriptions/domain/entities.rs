//! The prescriptions received today, and the one the pharmacist has open.

use std::sync::Arc;

use super::enums::{Source, Status};

/// A word the queue draws on every frame.
pub type Text = Arc<str>;

/// A word the queue draws, from anything a source holds.
#[must_use]
pub fn text(value: &str) -> Text {
    Arc::from(value)
}

/// One prescription in the queue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Queued {
    /// Its reference, `RX-2217`.
    pub reference: Text,
    /// When it reached the pharmacy, as the board writes it: `"Received 15:11"`.
    pub received: Text,
    /// Who it is for.
    pub patient: Text,
    /// What was prescribed, as the board's second line.
    pub medicines: Text,
    /// Where it came from.
    pub source: Source,
    /// Where it has got to.
    pub status: Status,
    /// Anything the pharmacist must not miss, in the board's own words.
    pub flags: Vec<Text>,
}

impl Queued {
    /// Whether anything on the row has to be looked at before it goes on.
    #[must_use]
    pub fn is_flagged(&self) -> bool {
        !self.flags.is_empty()
    }
}

/// One prescribed medicine on the open prescription.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    /// The medicine as prescribed.
    pub name: Text,
    /// How many were prescribed.
    pub quantity: u32,
    /// The directions, in the pharmacist's language.
    pub directions: Text,
    /// Whether it can be sold right now, and from which batch.
    pub availability: Text,
    /// Whether the shelf has it.
    pub in_stock: bool,
}

/// The prescription the rail opens: what it is, what it carries, what to do.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenPrescription {
    /// Its reference.
    pub reference: Text,
    /// How it reached the pharmacy and when.
    pub arrived: Text,
    /// Where it has got to.
    pub status: Status,
    /// Who it is for.
    pub patient: Text,
    /// The patient line under the name.
    pub patient_detail: Text,
    /// The clinical alert, if the check found one.
    pub alert: Option<(Text, Text)>,
    /// What was prescribed.
    pub items: Vec<Item>,
    /// What the patient already takes.
    pub also_takes: Text,
    /// Known allergies.
    pub allergies: Text,
    /// Who pays.
    pub paid_by: Text,
    /// Who prescribed it.
    pub prescriber: Text,
    /// The rule under the buttons.
    pub rule: Text,
}

impl OpenPrescription {
    /// Whether a pharmacist has to sign this one before it is dispensed.
    #[must_use]
    pub fn needs_a_pharmacist(&self) -> bool {
        self.alert.is_some()
    }

    /// How many items the label printer will have to be fed for.
    #[must_use]
    pub fn labels(&self) -> usize {
        self.items.len()
    }

    /// The items that are not on the shelf.
    #[must_use]
    pub fn out_of_stock(&self) -> Vec<&Item> {
        self.items.iter().filter(|item| !item.in_stock).collect()
    }
}

/// Everything the queue knows about today.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrescriptionQueue {
    /// The range of references on the board, for the table's meta line.
    pub range: Text,
    /// How long the oldest prescription has waited, as the board writes it.
    pub oldest_waited: Text,
    /// Today's prescriptions, waiting first then dispensed.
    pub queue: Vec<Queued>,
    /// Which reference the rail opens.
    pub selected: Text,
    /// The open prescription, with the detail the rail shows.
    pub open: OpenPrescription,
}
