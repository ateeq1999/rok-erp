//! The board's own figures, as the records a source would hand over.
//!
//! `PharmacyDashboard` is drawn at Mwenge branch on Friday 2 October 2026 at
//! 15:30. Until the dashboard's phase gives it a query, these records are the
//! figures the screen draws, so the board's numbers are the test fixtures.

use super::models::{
    DashboardRecord, HourRecord, MedicineSoldRecord, OtherBranchRecord, PaymentRecord, TaskRecord,
};

/// The branch the board is drawn at.
pub const MWENGE: &str = "Mwenge";

/// The branch the board compares against.
pub const TEGETA: &str = "Tegeta";

/// The moment the board is drawn at.
pub const NOW: &str = "Fri 2 Oct 2026, 15:30";

/// Mwenge branch's day, as the source records it.
#[must_use]
pub fn mwenge_day() -> DashboardRecord {
    DashboardRecord {
        branch: MWENGE.to_string(),
        hours: [
            ("08", 96_500),
            ("09", 184_200),
            ("10", 312_400),
            ("11", 268_900),
            ("12", 221_300),
            ("13", 247_600),
            ("14", 289_800),
            ("15", 225_500),
        ]
        .into_iter()
        .map(|(hour, sales)| HourRecord {
            hour: hour.to_string(),
            sales,
        })
        .collect(),
        payments: vec![
            PaymentRecord {
                label: "Insurance".to_string(),
                amount: 701_600,
            },
            PaymentRecord {
                label: "Mobile money".to_string(),
                amount: 632_200,
            },
            PaymentRecord {
                label: "Cash".to_string(),
                amount: 512_400,
            },
        ],
        prescriptions_dispensed: 23,
        prescriptions_waiting: 6,
        oldest_waiting_minutes: 34,
        expiring_within_30_days: 186_400,
        expiring_batches: 5,
        expired_batches_blocked: 2,
        claims_queried: 11,
        claims_queried_in: "National health insurance \u{b7} September batch".to_string(),
        average_basket: 18_600,
        tasks: story_tasks(),
        top_medicines: [
            ("Paracetamol 500mg tablets", "640 tabs", 32_000),
            ("Amoxicillin 500mg capsules", "231 caps", 46_200),
            ("Metformin 500mg tablets", "420 tabs", 50_400),
            ("Amlodipine 5mg tablets", "300 tabs", 45_000),
            ("Oral rehydration salts", "62 sachets", 31_000),
        ]
        .into_iter()
        .map(|(name, quantity, sales)| MedicineSoldRecord {
            name: name.to_string(),
            quantity: quantity.to_string(),
            sales,
        })
        .collect(),
        other_branch: OtherBranchRecord {
            name: TEGETA.to_string(),
            sales: 1_212_800,
            prescriptions_dispensed: 15,
            insurance_share_percent: 44,
            average_basket: 16_900,
            prescriptions_waiting: 2,
        },
    }
}

/// The day's five alerts: what the board lists under "Needs you now".
fn story_tasks() -> Vec<TaskRecord> {
    vec![
        TaskRecord {
            kind: "Recall".to_string(),
            severity: "Danger".to_string(),
            title: "Recall RC-0047 \u{b7} Amoxicillin 250mg/5ml, batch AMS-2404".to_string(),
            detail: "14 bottles quarantined \u{b7} 2 of 5 patients still to reach".to_string(),
            screen: "recalls".to_string(),
            action: "Open recall".to_string(),
        },
        TaskRecord {
            kind: "Check".to_string(),
            severity: "Warning".to_string(),
            title: "RX-2214 \u{b7} Mzee Salim R. \u{b7} interaction check".to_string(),
            detail: "Metronidazole with warfarin may increase bleeding risk. Contact the prescriber."
                .to_string(),
            screen: "prescriptions".to_string(),
            action: "Review".to_string(),
        },
        TaskRecord {
            kind: "Delivery".to_string(),
            severity: "Info".to_string(),
            title: "Delivery UZ-7781 arrived from Uzima Pharmaceuticals".to_string(),
            detail: "Arrived 11:20 \u{b7} 6 lines \u{b7} insulin in cold box \u{b7} count and check batches"
                .to_string(),
            screen: "receive".to_string(),
            action: "Receive".to_string(),
        },
        TaskRecord {
            kind: "Licence".to_string(),
            severity: "Warning".to_string(),
            title: "Premises licence renewal due in 57 days".to_string(),
            detail: "Pharmacy Council \u{b7} expires 30 Nov 2026 \u{b7} submit by 28 Nov".to_string(),
            screen: "licences".to_string(),
            action: "Start renewal".to_string(),
        },
        TaskRecord {
            kind: "Done".to_string(),
            severity: "Success".to_string(),
            title: "Controlled count done 08:15".to_string(),
            detail: "Tramadol 50mg and 5 other lines counted by Grace N. \u{b7} no difference"
                .to_string(),
            screen: "controlled-register".to_string(),
            action: "View register".to_string(),
        },
    ]
}
