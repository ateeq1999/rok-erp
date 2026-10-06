//! The board's own reports, as the records a source would hand over.
//!
//! `OfficeReports.html` is the reports board drawn on the story's filters: the
//! month to date, both branches. Every heading is a sum of its own rows, which
//! is the domain rule the tests check first.

use super::models::{ReportRecord, ReportsRecord, RowRecord};

/// One line of a report.
#[allow(clippy::too_many_arguments)]
fn row(
    label: &'static str,
    detail: &'static str,
    value: i64,
    share: Option<&'static str>,
    flagged: bool,
) -> RowRecord {
    RowRecord {
        label: label.to_string(),
        detail: detail.to_string(),
        value,
        share: share.map(std::string::ToString::to_string),
        flagged,
    }
}

/// One report.
fn report(
    kind: &'static str,
    title: &'static str,
    question: &'static str,
    total: i64,
    unit: &'static str,
    rows: Vec<RowRecord>,
    note: &'static str,
) -> ReportRecord {
    ReportRecord {
        kind: kind.to_string(),
        title: title.to_string(),
        question: question.to_string(),
        total,
        unit: unit.to_string(),
        rows,
        note: note.to_string(),
    }
}

/// What sold, and on which schedule.
fn sales() -> ReportRecord {
    report(
        "sales by medicine",
        "Sales by medicine and schedule",
        "What sold, and on which schedule",
        1_842_000,
        "shillings",
        vec![
            row(
                "Amoxicillin 500mg capsules",
                "prescription-only",
                412_000,
                Some("22.4%"),
                false,
            ),
            row(
                "Amlodipine 5mg tablets",
                "prescription-only",
                372_000,
                Some("20.2%"),
                false,
            ),
            row(
                "Paracetamol 500mg tablets",
                "general sale",
                341_000,
                Some("18.5%"),
                false,
            ),
            row(
                "Metronidazole 400mg tablets",
                "prescription-only",
                286_000,
                Some("15.5%"),
                false,
            ),
            row(
                "Oral rehydration salts",
                "over-the-counter",
                163_000,
                Some("8.8%"),
                false,
            ),
            row(
                "Human insulin 100 IU/ml",
                "pharmacy medicine",
                150_000,
                Some("8.1%"),
                false,
            ),
            row(
                "Cetirizine 10mg tablets",
                "over-the-counter",
                118_000,
                Some("6.4%"),
                false,
            ),
        ],
        "Medicines are VAT exempt, so this is the whole of what was taken.",
    )
}

/// What each schedule leaves after cost.
fn margin() -> ReportRecord {
    report(
        "margin",
        "Margin",
        "What each schedule leaves after cost",
        612_400,
        "shillings",
        vec![
            row(
                "Prescription-only",
                "384 dispensings",
                284_100,
                Some("33.2%"),
                false,
            ),
            row(
                "Pharmacy medicine",
                "62 dispensings, cold chain",
                176_300,
                Some("28.4%"),
                false,
            ),
            row("General sale", "214 sales", 92_800, Some("22.0%"), false),
            row(
                "Over-the-counter",
                "171 sales",
                59_200,
                Some("16.4%"),
                false,
            ),
        ],
        "Cost is the purchase price of the batch that was sold, not the list price.",
    )
}

/// What stock has been, and is about to be, written off.
fn expiry() -> ReportRecord {
    report(
        "expiry losses",
        "Expiry losses",
        "What stock has been, and is about to be, written off",
        214_700,
        "shillings",
        vec![
            row(
                "Expired on the shelf",
                "2 batches, written off this period",
                86_400,
                None,
                true,
            ),
            row(
                "Expiring within 30 days",
                "5 batches, still sellable",
                74_300,
                None,
                true,
            ),
            row("Expiring within 90 days", "14 batches", 24_100, None, false),
            row(
                "Quarantined, awaiting destruction",
                "1 batch, pharmacist witness needed",
                29_900,
                None,
                true,
            ),
        ],
        "A batch that is still sellable is not a loss yet; it is shown in its own bucket.",
    )
}

/// How long the insurers have been sitting on the queries.
fn claims() -> ReportRecord {
    report(
        "claims aging",
        "Claims aging",
        "How long the insurers have been sitting on the queries",
        92_500,
        "shillings",
        vec![
            row("0 to 30 days", "11 claims", 41_200, None, false),
            row("31 to 60 days", "6 claims", 28_600, None, false),
            row("61 to 90 days", "3 claims", 14_900, None, false),
            row(
                "Over 90 days",
                "2 claims, chased this week",
                7_800,
                None,
                true,
            ),
        ],
        "A claim over 90 days is chased; after 120 days it is written back.",
    )
}

/// What was sold against the controlled register.
fn controlled() -> ReportRecord {
    report(
        "controlled movements",
        "Controlled movements",
        "What was sold against the controlled register",
        929_000,
        "shillings",
        vec![
            row(
                "Tramadol 50mg capsules",
                "20 capsules dispensed, balance 80",
                300_000,
                None,
                false,
            ),
            row(
                "Methylphenidate 10mg tablets",
                "30 tablets dispensed, 1 repeat",
                396_000,
                None,
                false,
            ),
            row(
                "Morphine 10mg/ml injection",
                "14 ampoules dispensed, register signed",
                145_000,
                None,
                false,
            ),
            row(
                "Diazepam 5mg tablets",
                "60 tablets dispensed",
                88_000,
                None,
                false,
            ),
        ],
        "Every row here has a matching entry in the controlled register.",
    )
}

/// The board's own reports, on the filters the board opens with.
#[must_use]
pub fn board() -> ReportsRecord {
    ReportsRecord {
        period: "This month".to_string(),
        branch: "Both branches".to_string(),
        reports: vec![sales(), margin(), expiry(), claims(), controlled()],
    }
}
