//! The board's own licences, readiness list and document folder, as the
//! records a source would hand over.
//!
//! `Pharmacy_licences_amp_inspection.html` is drawn on Friday 4 October 2026,
//! with the premises licence's renewal the one thing the board leads with.

use super::models::{DocumentRecord, FolderRecord, LicenceRecord, RequirementRecord};

/// One licence the pharmacy holds.
#[allow(clippy::too_many_arguments)]
fn licence(
    title: &'static str,
    issuer: &'static str,
    number: &'static str,
    branch: &'static str,
    expires_on: &'static str,
    standing: &'static str,
    renewal_application_due: Option<&'static str>,
) -> LicenceRecord {
    LicenceRecord {
        title: title.to_string(),
        issuer: issuer.to_string(),
        number: number.to_string(),
        branch: branch.to_string(),
        expires_on: expires_on.to_string(),
        standing: standing.to_string(),
        renewal_application_due: renewal_application_due.map(std::string::ToString::to_string),
    }
}

/// One thing the inspector will ask for.
fn requirement(name: &'static str, detail: &'static str, ready: bool) -> RequirementRecord {
    RequirementRecord {
        name: name.to_string(),
        detail: detail.to_string(),
        ready,
    }
}

/// One document in the folder.
fn document(
    name: &'static str,
    branch: &'static str,
    expires_on: Option<&'static str>,
    reminder: &'static str,
    on_file: bool,
) -> DocumentRecord {
    DocumentRecord {
        name: name.to_string(),
        branch: branch.to_string(),
        expires_on: expires_on.map(std::string::ToString::to_string),
        reminder: reminder.to_string(),
        on_file,
    }
}

/// The licences and registrations the pharmacy holds.
fn held() -> Vec<LicenceRecord> {
    vec![
        licence(
            "Pharmacy premises licence",
            "Pharmacy Council",
            "PC/TZ/MWE/2214",
            "Mwenge",
            "30 Nov 2026",
            "Renewing",
            Some("28 Nov 2026"),
        ),
        licence(
            "Pharmacy premises licence",
            "Pharmacy Council",
            "PC/TZ/TGT/2098",
            "Tegeta",
            "14 Mar 2027",
            "Valid",
            None,
        ),
        licence(
            "Drug trader licence",
            "TFDA",
            "TFDA/DT/08841",
            "Mwenge and Tegeta",
            "02 Feb 2027",
            "Valid",
            None,
        ),
        licence(
            "Controlled substances registration",
            "Pharmacy Council",
            "PC/CS/MWE/0071",
            "Mwenge",
            "31 Dec 2026",
            "Renewing",
            None,
        ),
        licence(
            "Business licence",
            "District council",
            "MMC/BL/2019/4412",
            "Mwenge and Tegeta",
            "30 Jun 2027",
            "Valid",
            None,
        ),
    ]
}

/// What an inspector asks for, hardest to satisfy first.
fn readiness() -> Vec<RequirementRecord> {
    vec![
        requirement(
            "Prescription registers for both branches",
            "kept as issued, no corrections after the fact",
            true,
        ),
        requirement(
            "Controlled substances register",
            "running balance per substance, per patient",
            true,
        ),
        requirement(
            "Temperature logs for the fridge",
            "continuous logger, read every working day",
            true,
        ),
        requirement(
            "Batch recall record",
            "RC-0047 in progress, the list is kept either way",
            true,
        ),
        requirement(
            "Premises licence renewal application",
            "due 28 Nov, not yet submitted",
            false,
        ),
        requirement(
            "Pharmacist's professional registration",
            "Grace N. renews in January",
            true,
        ),
    ]
}

/// The folder the inspector reads.
fn documents() -> Vec<DocumentRecord> {
    vec![
        document(
            "Pharmacy premises licence",
            "Mwenge",
            Some("30 Nov 2026"),
            "90 days before",
            true,
        ),
        document(
            "Controlled substances registration",
            "Mwenge",
            Some("31 Dec 2026"),
            "90 days before",
            true,
        ),
        document(
            "Pharmacy premises licence",
            "Tegeta",
            Some("14 Mar 2027"),
            "90 days before",
            true,
        ),
        document(
            "Drug trader licence",
            "Mwenge and Tegeta",
            Some("02 Feb 2027"),
            "60 days before",
            true,
        ),
        document(
            "Premises licence renewal application",
            "Mwenge",
            Some("28 Nov 2026"),
            "submit by",
            false,
        ),
    ]
}

/// The board's whole folder.
#[must_use]
pub fn folder() -> FolderRecord {
    FolderRecord {
        licences: held(),
        readiness: readiness(),
        documents: documents(),
    }
}
