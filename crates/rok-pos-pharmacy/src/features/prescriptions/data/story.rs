//! The board's own queue, as the records a source would hand over.
//!
//! `Pharmacy_prescription_queue.html` is drawn at Mwenge branch on Friday 2
//! October 2026: ten prescriptions received today, six still to dispense.

use super::models::{ItemRecord, OpenPrescriptionRecord, QueueRecord, QueuedRecord};

/// The prescription the board opens on: the warfarin interaction.
pub const SELECTED: &str = "RX-2214";

/// One of the board's queue rows.
fn queued(
    reference: &'static str,
    received: &'static str,
    patient: &'static str,
    medicines: &'static str,
    source: &'static str,
    status: &'static str,
    flags: &[&str],
) -> QueuedRecord {
    QueuedRecord {
        reference: reference.to_string(),
        received: received.to_string(),
        patient: patient.to_string(),
        medicines: medicines.to_string(),
        source: source.to_string(),
        status: status.to_string(),
        flags: flags.iter().map(|flag| (*flag).to_string()).collect(),
    }
}

/// The day as the board draws it, newest receipt first.
#[must_use]
pub fn today() -> QueueRecord {
    QueueRecord {
        range: "All prescriptions today \u{b7} RX-2210 to RX-2219".to_string(),
        oldest_waited: "oldest waiting 2 h 45 min".to_string(),
        queue: vec![
            queued(
                "RX-2217",
                "Received 15:11",
                "Daudi M.",
                "Metformin 500mg, Atorvastatin 20mg",
                "E-prescription",
                "New",
                &["National health insurance"],
            ),
            queued(
                "RX-2216",
                "Received 15:06",
                "[Patient name]",
                "Photo not read yet",
                "WhatsApp photo",
                "New",
                &["Photo needs reading"],
            ),
            queued(
                "RX-2215",
                "Received 15:02",
                "Rehema Juma",
                "Child antibiotic syrup \u{b7} Amoxicillin 250mg/5ml",
                "Paper at counter",
                "Needs pharmacist check",
                &["Batch AMS-2404 recalled \u{b7} use another batch"],
            ),
            queued(
                "RX-2214",
                "Received 14:48",
                "Mzee Salim R.",
                "Metronidazole 400mg, Amoxicillin 500mg",
                "Paper at counter",
                "Needs pharmacist check",
                &["Interaction: warfarin"],
            ),
            queued(
                "RX-2218",
                "Received 13:35",
                "Asha P.",
                "Dose unclear on paper \u{b7} called clinic 13:50",
                "Paper at counter",
                "Waiting for prescriber",
                &["Call-back due 15:30"],
            ),
            queued(
                "RX-2219",
                "Received 12:30",
                "Khamis B.",
                "Amlodipine 5mg, Losartan 50mg \u{b7} packed by John M.",
                "E-prescription",
                "Ready to collect",
                &["Text sent 13:15"],
            ),
            queued(
                "RX-2210",
                "Dispensed 14:05",
                "Ali Hassan",
                "Amoxicillin, Paracetamol, ORS, Tramadol \u{b7} Grace N.",
                "Paper at counter",
                "Dispensed",
                &["Controlled item signed"],
            ),
            queued(
                "RX-2213",
                "Dispensed 11:55",
                "Mariam S.",
                "Ferrous sulphate, Folic acid \u{b7} John M.",
                "E-prescription",
                "Dispensed",
                &[],
            ),
            queued(
                "RX-2212",
                "Dispensed 10:40",
                "Juma K.",
                "Salbutamol inhaler \u{b7} Grace N.",
                "WhatsApp photo",
                "Dispensed",
                &[],
            ),
            queued(
                "RX-2211",
                "Dispensed 09:12",
                "Upendo L.",
                "Levothyroxine 50mcg \u{b7} John M.",
                "Paper at counter",
                "Dispensed",
                &[],
            ),
        ],
        selected: "RX-2214".to_string(),
        open: rx_2214(),
    }
}

/// The prescription the board opens: the warfarin interaction.
fn rx_2214() -> OpenPrescriptionRecord {
    OpenPrescriptionRecord {
        reference: "RX-2214".to_string(),
        arrived: "Paper at counter \u{b7} received 14:48".to_string(),
        status: "Needs pharmacist check".to_string(),
        patient: "Mzee Salim R.".to_string(),
        patient_detail: "Male \u{b7} 68 years \u{b7} +255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 508"
            .to_string(),
        alert_title: Some("Interaction alert: warfarin + metronidazole.".to_string()),
        alert_body: Some(
            "Metronidazole may increase the effect of warfarin and the risk of bleeding. Contact the prescriber."
                .to_string(),
        ),
        items: vec![
            ItemRecord {
                name: "Metronidazole 400mg tablets".to_string(),
                quantity: 21,
                directions: "1 tablet 3 times a day for 7 days, avoid alcohol".to_string(),
                availability: "In stock".to_string(),
            },
            ItemRecord {
                name: "Amoxicillin 500mg capsules".to_string(),
                quantity: 15,
                directions: "1 capsule 3 times a day for 5 days".to_string(),
                availability: "In stock \u{b7} AMX-2409".to_string(),
            },
        ],
        also_takes: "Warfarin (as directed by clinic), Metformin 500mg, Amlodipine 5mg".to_string(),
        allergies: "none known".to_string(),
        paid_by: "National health insurance \u{b7} member active".to_string(),
        prescriber: "[Prescriber name] \u{b7} [Dental clinic name]".to_string(),
    }
}
