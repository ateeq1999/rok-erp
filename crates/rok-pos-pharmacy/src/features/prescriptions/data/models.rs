//! The records a source hands over for the day's prescriptions, and the
//! entities they become.

use crate::features::prescriptions::domain::entities::{
    Item, OpenPrescription, PrescriptionQueue, Queued, text,
};
use crate::features::prescriptions::domain::enums::{Source, Status};

/// One prescription, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueuedRecord {
    /// Its reference.
    pub reference: String,
    /// When it reached the pharmacy.
    pub received: String,
    /// Who it is for.
    pub patient: String,
    /// What was prescribed.
    pub medicines: String,
    /// Where it came from, as the source names it.
    pub source: String,
    /// Where it has got to, as the source names it.
    pub status: String,
    /// Anything the pharmacist must not miss.
    pub flags: Vec<String>,
}

/// One prescribed medicine, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemRecord {
    /// The medicine as prescribed.
    pub name: String,
    /// How many were prescribed.
    pub quantity: u32,
    /// The directions.
    pub directions: String,
    /// Whether it can be sold right now, and from which batch.
    pub availability: String,
}

/// The open prescription, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenPrescriptionRecord {
    /// Its reference.
    pub reference: String,
    /// How it reached the pharmacy and when.
    pub arrived: String,
    /// Where it has got to.
    pub status: String,
    /// Who it is for.
    pub patient: String,
    /// The patient line under the name.
    pub patient_detail: String,
    /// The clinical alert's title, if the check found one.
    pub alert_title: Option<String>,
    /// The clinical alert's body.
    pub alert_body: Option<String>,
    /// What was prescribed.
    pub items: Vec<ItemRecord>,
    /// What the patient already takes.
    pub also_takes: String,
    /// Known allergies.
    pub allergies: String,
    /// Who pays.
    pub paid_by: String,
    /// Who prescribed it.
    pub prescriber: String,
}

/// The day, as the source records it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueueRecord {
    /// The range of references, for the table's meta line.
    pub range: String,
    /// How long the oldest prescription has waited.
    pub oldest_waited: String,
    /// Today's prescriptions.
    pub queue: Vec<QueuedRecord>,
    /// Which reference the rail opens.
    pub selected: String,
    /// The open prescription.
    pub open: OpenPrescriptionRecord,
}

/// What a source calls a state.
fn status(status: &str) -> Status {
    match status {
        "New" => Status::New,
        "Waiting for prescriber" => Status::WaitingPrescriber,
        "Ready to collect" => Status::ReadyToCollect,
        "Dispensed" => Status::Dispensed,
        _ => Status::NeedsCheck,
    }
}

/// What a source calls a source.
fn source(source: &str) -> Source {
    match source {
        "WhatsApp photo" => Source::Photo,
        "E-prescription" => Source::Electronic,
        _ => Source::Paper,
    }
}

impl From<QueuedRecord> for Queued {
    fn from(record: QueuedRecord) -> Self {
        Self {
            reference: text(&record.reference),
            received: text(&record.received),
            patient: text(&record.patient),
            medicines: text(&record.medicines),
            source: source(&record.source),
            status: status(&record.status),
            flags: record.flags.iter().map(|flag| text(flag)).collect(),
        }
    }
}

impl From<ItemRecord> for Item {
    fn from(record: ItemRecord) -> Self {
        let availability = record.availability.clone();
        Self {
            name: text(&record.name),
            quantity: record.quantity,
            directions: text(&record.directions),
            availability: text(&availability),
            in_stock: availability.starts_with("In stock"),
        }
    }
}

impl From<OpenPrescriptionRecord> for OpenPrescription {
    fn from(record: OpenPrescriptionRecord) -> Self {
        Self {
            reference: text(&record.reference),
            arrived: text(&record.arrived),
            status: status(&record.status),
            patient: text(&record.patient),
            patient_detail: text(&record.patient_detail),
            alert: record
                .alert_title
                .zip(record.alert_body)
                .map(|(title, body)| (text(&title), text(&body))),
            items: record.items.into_iter().map(Item::from).collect(),
            also_takes: text(&record.also_takes),
            allergies: text(&record.allergies),
            paid_by: text(&record.paid_by),
            prescriber: text(&record.prescriber),
            rule: text("Only a pharmacist can approve a prescription with an interaction alert."),
        }
    }
}

impl From<QueueRecord> for PrescriptionQueue {
    fn from(record: QueueRecord) -> Self {
        Self {
            range: text(&record.range),
            oldest_waited: text(&record.oldest_waited),
            queue: record.queue.into_iter().map(Queued::from).collect(),
            selected: text(&record.selected),
            open: OpenPrescription::from(record.open),
        }
    }
}
