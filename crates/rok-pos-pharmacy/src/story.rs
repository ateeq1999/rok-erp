//! The Afya Pharmacy story the boards are drawn with: Mwenge branch on Friday
//! 2 October 2026 at 15:30, with Grace N. in charge.
//!
//! Until a screen's phase gives it a query, it draws these figures, so the app
//! looks like its board and the board's numbers are the test fixtures. Each
//! value says which phase replaces it; when that phase lands, the value goes.

use rok_pos_shell::{Tone, TopBarStatus, UserChip};

use crate::navigation::SidebarCounts;

/// The business's name, in the sidebar and every subheading.
pub const BUSINESS: &str = "Afya Pharmacy";

/// The branch this window is signed in to.
pub const BRANCH: &str = "Mwenge branch";

/// The moment the boards are drawn at, as the dashboard writes it.
pub const NOW: &str = "Fri 2 Oct 2026, 15:30";

/// What the boards put between the parts of a subheading.
pub const SEPARATOR: &str = " \u{b7} ";

/// Who is signed in. Phase 3 (sign-in) replaces this with the session's user.
#[must_use]
pub fn signed_in() -> UserChip {
    UserChip::new("GN", "Grace N.", "Pharmacist in charge")
}

/// The counts beside the sidebar rows. Each phase that owns a screen replaces
/// its count with a query: prescriptions in 7, refills in 9, batches in 5,
/// deliveries in 11, recalls in 12, claims in 10.
#[must_use]
pub const fn sidebar_counts() -> SidebarCounts {
    SidebarCounts {
        prescriptions_waiting: Some(6),
        refills_due: Some(9),
        batches_needing_action: Some(7),
        deliveries_to_receive: Some(1),
        open_recalls: Some(1),
        claim_batches_queried: Some(3),
    }
}

/// The sync chip in the top bar. Phase 15 (branch sync) replaces it.
#[must_use]
pub fn sync_status() -> TopBarStatus {
    TopBarStatus::new("Synced 2 min ago", Tone::Success)
}

/// The line under a page's heading: the business and branch, and on the
/// dashboard the time as well.
///
/// ```
/// # use rok_pos_pharmacy::story;
/// assert_eq!(story::subheading(false), "Afya Pharmacy \u{b7} Mwenge branch");
/// assert!(story::subheading(true).ends_with("Fri 2 Oct 2026, 15:30"));
/// ```
#[must_use]
pub fn subheading(with_time: bool) -> String {
    let mut line = format!("{BUSINESS}{SEPARATOR}{BRANCH}");
    if with_time {
        line.push_str(SEPARATOR);
        line.push_str(NOW);
    }
    line
}
