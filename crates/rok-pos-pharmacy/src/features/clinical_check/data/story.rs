//! The board's own prescription, as the records a source would hand over.

use super::models::{CheckRecord, CheckRecordSet, MedicineRecord, OutcomeRecord};

/// The prescription the board opens on.
pub const RX_2214: &str = "RX-2214";

/// The board's prescription, as the source records it.
#[must_use]
pub fn rx_2214() -> CheckRecordSet {
    CheckRecordSet {
        reference: RX_2214.to_string(),
        patient: "Mzee Salim R.".to_string(),
        age: "68 years \u{b7} male".to_string(),
        prescriber: "[Prescriber name]".to_string(),
        clinic: "[Dental clinic name]".to_string(),
        issued: "02/10/2026".to_string(),
        scanned: "14:48 at counter 2".to_string(),
        medicines: vec![
            MedicineRecord {
                name: "Metronidazole 400mg tablets".to_string(),
                quantity: "21".to_string(),
                batch: "MTZ-2503".to_string(),
                directions: "1 tablet 3 times a day for 7 days, avoid alcohol".to_string(),
                label_words: "Take 1 tablet 3 times a day for 7 days. Avoid alcohol. Complete the course."
                    .to_string(),
                supply: "7 days".to_string(),
                expiry: "06/2027".to_string(),
            },
            MedicineRecord {
                name: "Amoxicillin 500mg capsules".to_string(),
                quantity: "15".to_string(),
                batch: "AMX-2409".to_string(),
                directions: "1 capsule 3 times a day for 5 days".to_string(),
                label_words: "Take 1 capsule 3 times a day for 5 days. Finish the course."
                    .to_string(),
                supply: "5 days".to_string(),
                expiry: "03/2027".to_string(),
            },
        ],
        checks: vec![
            CheckRecord {
                label: "Patient identity".to_string(),
                detail: "Name and phone match the patient record. Age 68 confirmed with the patient."
                    .to_string(),
                result: "Confirmed".to_string(),
                finding: "Clear".to_string(),
            },
            CheckRecord {
                label: "Allergies".to_string(),
                detail: "None known. Asked the patient again today.".to_string(),
                result: "None known".to_string(),
                finding: "Clear".to_string(),
            },
            CheckRecord {
                label: "Interactions".to_string(),
                detail: "Warfarin + metronidazole. Metronidazole may increase the effect of warfarin and the risk of bleeding. Contact the prescriber before dispensing, or agree extra INR monitoring."
                    .to_string(),
                result: "Alert".to_string(),
                finding: "Alert".to_string(),
            },
            CheckRecord {
                label: "Duplicate therapy".to_string(),
                detail: "No other antibiotic on file in the last 30 days.".to_string(),
                result: "None found".to_string(),
                finding: "Clear".to_string(),
            },
            CheckRecord {
                label: "Dose".to_string(),
                detail: "Both doses are within the usual label range for adults.".to_string(),
                result: "Within range".to_string(),
                finding: "Clear".to_string(),
            },
            CheckRecord {
                label: "Insurance cover".to_string(),
                detail: "National health insurance \u{b7} member active \u{b7} both items on the formulary."
                    .to_string(),
                result: "Covered".to_string(),
                finding: "Clear".to_string(),
            },
        ],
        outcomes: vec![
            OutcomeRecord {
                label: "Prescriber changed the medicine".to_string(),
                chosen: false,
            },
            OutcomeRecord {
                label: "Kept as written, extra INR check".to_string(),
                chosen: true,
            },
            OutcomeRecord {
                label: "Not reached, hold".to_string(),
                chosen: false,
            },
        ],
        counselling: vec![
            "Take metronidazole with or after food. Do not drink alcohol during the course and for 48 hours after."
                .to_string(),
            "Take amoxicillin at evenly spaced times and finish the full 5 days.".to_string(),
            "Keep taking warfarin exactly as the clinic directs. Do not change the dose yourself."
                .to_string(),
            "Book the INR check in 3\u{2013}5 days at the warfarin clinic.".to_string(),
            "Contact the clinic quickly if you notice unusual bleeding, bruising, nosebleeds, blood in urine or dark stools."
                .to_string(),
        ],
        note: "Called [Dental clinic name] 15:10, spoke to [Prescriber name]. Prescriber agreed to keep metronidazole and amoxicillin as written. Patient to have an INR check in 3\u{2013}5 days at the warfarin clinic. Patient told."
            .to_string(),
        pharmacist: "Grace N.".to_string(),
        initials: "GN".to_string(),
    }
}
