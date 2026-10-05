//! The board's own patient and list, as the records a source would hand over.
//!
//! `Pharmacy_patient_record.html` is drawn for Mzee Salim R. on Friday 2
//! October 2026; the list opens on the seven patients the pharmacy sees most.

use super::models::{
    ConditionRecord, DetailRecord, FillRecord, ListingRecord, MedicineRecord, NoteRecord,
    PatientRecord,
};

/// One fact about the board's patient.
fn detail(label: &'static str, value: &'static str) -> DetailRecord {
    DetailRecord {
        label: label.to_string(),
        value: value.to_string(),
    }
}

/// One of the board patient's conditions.
fn condition(name: &'static str, source: &'static str) -> ConditionRecord {
    ConditionRecord {
        name: name.to_string(),
        source: source.to_string(),
    }
}

/// One medicine the board patient takes.
fn medicine(
    name: &'static str,
    directions: &'static str,
    last_filled: &'static str,
    due: &'static str,
) -> MedicineRecord {
    MedicineRecord {
        name: name.to_string(),
        directions: directions.to_string(),
        last_filled: last_filled.to_string(),
        due: due.to_string(),
    }
}

/// One fill on the board patient's history.
fn fill(
    date: &'static str,
    reference: &'static str,
    medicines: &'static str,
    prescriber: &'static str,
    pharmacist: &'static str,
    status: &'static str,
) -> FillRecord {
    FillRecord {
        date: date.to_string(),
        reference: reference.to_string(),
        medicines: medicines.to_string(),
        prescriber: prescriber.to_string(),
        pharmacist: pharmacist.to_string(),
        status: status.to_string(),
    }
}

/// One note on the board patient's record.
fn note(title: &'static str, meta: &'static str, body: &'static str) -> NoteRecord {
    NoteRecord {
        title: title.to_string(),
        meta: meta.to_string(),
        body: body.to_string(),
    }
}

/// The fills the board's history shows, newest first.
fn mzee_salims_fills() -> Vec<FillRecord> {
    vec![
        fill(
            "02 Oct 2026",
            "RX-2214",
            "Metronidazole 400mg \u{d7} 21, Amoxicillin 500mg \u{d7} 15",
            "[Dental clinic name]",
            "Grace N.",
            "Approved, at till",
        ),
        fill(
            "18 Sep 2026",
            "RX-2068",
            "Warfarin, as directed",
            "[Warfarin clinic]",
            "Grace N.",
            "Collected",
        ),
        fill(
            "12 Sep 2026",
            "RX-2012",
            "Amlodipine 5mg \u{d7} 30",
            "[Prescriber name]",
            "John M.",
            "Collected",
        ),
        fill(
            "05 Sep 2026",
            "RX-1944",
            "Metformin 500mg \u{d7} 60",
            "[Prescriber name]",
            "Grace N.",
            "Collected",
        ),
        fill(
            "21 Aug 2026",
            "RX-1871",
            "Warfarin, as directed",
            "[Warfarin clinic]",
            "Grace N.",
            "Collected",
        ),
        fill(
            "13 Aug 2026",
            "RX-1839",
            "Amlodipine 5mg \u{d7} 30",
            "[Prescriber name]",
            "John M.",
            "Collected",
        ),
    ]
}

/// The board's patient, Mzee Salim R.
#[must_use]
pub fn mzee_salim() -> PatientRecord {
    PatientRecord {
        initials: "SR".to_string(),
        name: "Mzee Salim R.".to_string(),
        details: vec![
            detail("Patient since", "03/2024"),
            detail("Phone", "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 508"),
            detail("Date of birth", "[Date of birth] \u{b7} 68 years"),
            detail("Sex", "Male"),
            detail("Area", "Mwenge, Dar es Salaam"),
            detail("Preferred language", "Kiswahili"),
        ],
        insurer: "National health insurance \u{b7} member 10-xxxx-508".to_string(),
        cover: "Active \u{b7} checked 02 Oct 14:50".to_string(),
        allergies: "none known \u{b7} confirmed 02 Oct".to_string(),
        allergies_label: "Allergies:".to_string(),
        allergy_source:
            "From prescriptions on file. As written by prescribers. Not a diagnosis by the pharmacy."
                .to_string(),
        conditions: vec![
            condition(
                "On warfarin therapy (followed by warfarin clinic)",
                "prescription",
            ),
            condition("Type 2 diabetes", "prescription"),
            condition("High blood pressure", "prescription"),
        ],
        reminders: "On".to_string(),
        consent: "Consent given at the counter 14 Mar 2024. Kiswahili messages.".to_string(),
        medicines: vec![
            medicine(
                "Warfarin tablets",
                "As directed by the clinic",
                "18 Sep 2026",
                "Set by clinic",
            ),
            medicine(
                "Metformin 500mg tablets",
                "1 tablet twice a day with food",
                "05 Sep 2026",
                "Due 5 Oct",
            ),
            medicine(
                "Amlodipine 5mg tablets",
                "1 tablet once a day",
                "12 Sep 2026",
                "Due 12 Oct",
            ),
        ],
        fills: mzee_salims_fills(),
        notes: vec![
            note(
                "Prescriber call \u{b7} RX-2214",
                "02 Oct 15:10 \u{b7} Grace N.",
                "Interaction alert warfarin + metronidazole. Called [Dental clinic name], spoke to [Prescriber name]. Prescriber agreed to keep both and asked for an extra INR check.",
            ),
            note(
                "Counselling",
                "18 Sep 14:22 \u{b7} Grace N.",
                "Patient asked about pain relief. Advised to ask the pharmacist before taking any new medicine, including painkillers bought elsewhere.",
            ),
        ],
    }
}

/// One row of the board's list.
#[allow(clippy::too_many_arguments)]
fn listing(
    href: &'static str,
    initials: &'static str,
    name: &'static str,
    phone: &'static str,
    cover: &'static str,
    conditions: &'static str,
    next_due: Option<&'static str>,
    due_this_week: bool,
) -> ListingRecord {
    ListingRecord {
        href: href.to_string(),
        initials: initials.to_string(),
        name: name.to_string(),
        phone: phone.to_string(),
        cover: cover.to_string(),
        conditions: conditions.to_string(),
        next_due: next_due.map(std::string::ToString::to_string),
        due_this_week,
    }
}

/// The board's list, most urgent first.
#[must_use]
pub fn directory() -> Vec<ListingRecord> {
    vec![
        listing(
            "/patients/P-1042",
            "SR",
            "Mzee Salim R.",
            "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 508",
            "National health insurance",
            "Type 2 diabetes \u{b7} Hypertension",
            Some("Metformin 500mg \u{b7} due 5 Oct"),
            true,
        ),
        listing(
            "/patients/P-0987",
            "NA",
            "Zainabu A.",
            "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 117",
            "National health insurance",
            "Hypothyroidism",
            Some("Levothyroxine 50mcg \u{b7} due 3 Oct"),
            true,
        ),
        listing(
            "/patients/P-1103",
            "NK",
            "Neema K.",
            "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 342",
            "Cash",
            "Hypertension",
            Some("Amlodipine 5mg \u{b7} due 4 Oct"),
            true,
        ),
        listing(
            "/patients/P-0512",
            "BM",
            "Baraka M.",
            "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 860",
            "National health insurance",
            "Asthma",
            None,
            false,
        ),
        listing(
            "/patients/P-0744",
            "FH",
            "Fatuma H.",
            "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 093",
            "National health insurance",
            "Hypothyroidism \u{b7} Hyperlipidaemia",
            Some("Levothyroxine 100mcg \u{b7} due 6 Oct"),
            true,
        ),
        listing(
            "/patients/P-1220",
            "JL",
            "Joseph L.",
            "+255 6\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 451",
            "National health insurance",
            "Type 2 diabetes",
            Some("Metformin 500mg \u{b7} due 7 Oct"),
            false,
        ),
        listing(
            "/patients/P-0088",
            "MT",
            "Mwanaisha T.",
            "+255 7\u{2022}\u{2022}\u{2022} \u{2022}\u{2022}\u{2022} 278",
            "Cash",
            "Hypertension",
            Some("Losartan 50mg \u{b7} due 8 Oct"),
            false,
        ),
    ]
}
