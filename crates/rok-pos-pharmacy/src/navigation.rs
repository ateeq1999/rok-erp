//! The pharmacy's navigation, as the `PharmacySidebar` board lays it out: what
//! each screen is called, which group it sits in, what permission it needs and
//! which count it carries.
//!
//! The sidebar is data, so this is data too: [`ITEMS`] is every screen, and
//! [`groups`] turns it into the groups the app hands the shell. A screen behind
//! a dynamic path needs no row of its own, because a non-exact row stays lit on
//! the paths below it. The till is not a row: it is the big button above them.

use rok_pos_shell::{NavGroup, NavItem, SidebarAction, Tone};

/// One screen: what the sidebar calls it, where it opens, and who may open it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Screen {
    /// The label the sidebar and the top bar show.
    pub label: &'static str,
    /// The path it opens.
    pub href: &'static str,
    /// Count as current only on this exact path, so a screen below a list does
    /// not light up the list.
    pub exact: bool,
    /// The permission the screen needs, as a module manifest names it.
    pub permission: Option<&'static str>,
}

const fn screen(
    label: &'static str,
    href: &'static str,
    exact: bool,
    permission: Option<&'static str>,
) -> Screen {
    Screen {
        label,
        href,
        exact,
        permission,
    }
}

/// Every screen, in the order the sidebar shows them, then the till.
pub const ITEMS: [Screen; 14] = [
    screen("Dashboard", "/", true, None),
    screen(
        "Prescriptions",
        "/prescriptions",
        false,
        Some("pharmacy.prescriptions.view"),
    ),
    screen(
        "Patients",
        "/patients",
        false,
        Some("customers.customers.view"),
    ),
    screen(
        "Refills due",
        "/refills",
        true,
        Some("pharmacy.prescriptions.view"),
    ),
    screen(
        "Medicines",
        "/medicines",
        true,
        Some("catalog.products.view"),
    ),
    screen(
        "Batches & expiry",
        "/batches",
        true,
        Some("inventory.stock.view"),
    ),
    screen(
        "Controlled register",
        "/controlled-register",
        true,
        Some("pharmacy.controlled_register.view"),
    ),
    screen(
        "Order medicines",
        "/order",
        true,
        Some("purchasing.purchase_orders.create"),
    ),
    screen(
        "Receive deliveries",
        "/receive",
        false,
        Some("purchasing.goods_receipts.receive"),
    ),
    // Everyone at the counter has to see a recall: it blocks what they sell.
    screen("Recalls", "/recalls", false, None),
    screen(
        "Insurance claims",
        "/claims",
        true,
        Some("pharmacy.insurance_claims.manage"),
    ),
    screen(
        "Licences & inspection",
        "/licences",
        true,
        Some("pharmacy.licences.manage"),
    ),
    screen(
        "Reports",
        "/reports",
        true,
        Some("point_of_sale.sales.view"),
    ),
    screen(
        "Dispensary till",
        TILL,
        true,
        Some("point_of_sale.sales.create"),
    ),
];

/// Where the till opens.
pub const TILL: &str = "/till";

/// The sidebar's groups, in order: the heading, then each screen's path.
const GROUPS: [(&str, &[&str]); 3] = [
    (
        "Dispensary",
        &["/", "/prescriptions", "/patients", "/refills"],
    ),
    (
        "Medicines & stock",
        &[
            "/medicines",
            "/batches",
            "/controlled-register",
            "/order",
            "/receive",
            "/recalls",
        ],
    ),
    ("Money & rules", &["/claims", "/licences", "/reports"]),
];

/// The counts beside the sidebar rows. `None` draws no badge, and so does zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SidebarCounts {
    /// Prescriptions waiting to be checked or dispensed.
    pub prescriptions_waiting: Option<u32>,
    /// Refills due in the next seven days.
    pub refills_due: Option<u32>,
    /// Batches expired on the shelf or expiring within 30 days.
    pub batches_needing_action: Option<u32>,
    /// Deliveries that have arrived and are not yet received.
    pub deliveries_to_receive: Option<u32>,
    /// Recalls not yet closed.
    pub open_recalls: Option<u32>,
    /// Claim batches with queried claims.
    pub claim_batches_queried: Option<u32>,
}

impl SidebarCounts {
    /// The count and tone for the row at `href`, as the board draws them.
    fn badge(&self, href: &str) -> Option<(u32, Tone)> {
        let (count, tone) = match href {
            "/prescriptions" => (self.prescriptions_waiting, Tone::Brand),
            "/refills" => (self.refills_due, Tone::Brand),
            "/batches" => (self.batches_needing_action, Tone::Warning),
            "/receive" => (self.deliveries_to_receive, Tone::Brand),
            "/recalls" => (self.open_recalls, Tone::Danger),
            "/claims" => (self.claim_batches_queried, Tone::Warning),
            _ => return None,
        };
        count.filter(|count| *count > 0).map(|count| (count, tone))
    }
}

/// The sidebar for a user whose permissions are `can`, with `counts` beside
/// the rows.
///
/// ```
/// # use rok_pos_pharmacy::navigation::{self, SidebarCounts};
/// let everything = navigation::groups(&|_| true, &SidebarCounts::default());
/// assert_eq!(everything.len(), 3);
/// let nothing = navigation::groups(&|_| false, &SidebarCounts::default());
/// assert_eq!(nothing.len(), 2, "the dashboard and recalls need no permission");
/// ```
#[must_use]
pub fn groups(can: &dyn Fn(&str) -> bool, counts: &SidebarCounts) -> Vec<NavGroup> {
    GROUPS
        .iter()
        .filter_map(|(label, hrefs)| {
            let items = hrefs
                .iter()
                .filter_map(|href| item(href, can, counts))
                .collect::<Vec<NavItem>>();
            (!items.is_empty()).then(|| NavGroup::new(*label, items))
        })
        .collect()
}

/// One navigation row, or nothing when `can` refuses its permission.
///
/// ```
/// # use rok_pos_pharmacy::navigation::{self, SidebarCounts};
/// let counts = SidebarCounts::default();
/// assert!(navigation::item("/", &|_| false, &counts).is_some());
/// assert!(navigation::item("/controlled-register", &|_| false, &counts).is_none());
/// assert!(navigation::item("/controlled-register", &|_| true, &counts).is_some());
/// ```
#[must_use]
pub fn item(href: &str, can: &dyn Fn(&str) -> bool, counts: &SidebarCounts) -> Option<NavItem> {
    find(href)
        .filter(|screen| screen.permission.is_none_or(can))
        .map(|screen| {
            let row = NavItem::new(screen.label)
                .href(screen.href)
                .exact(screen.exact);
            match counts.badge(screen.href) {
                Some((count, tone)) => row.badge(count.to_string(), tone),
                None => row,
            }
        })
}

/// The big button above the rows, when `can` allows selling.
#[must_use]
pub fn till_action(can: &dyn Fn(&str) -> bool) -> Option<SidebarAction> {
    find(TILL)
        .filter(|screen| screen.permission.is_none_or(can))
        .map(|_| SidebarAction::new("Open dispensary till", TILL))
}

/// A screen's label, for its top bar.
///
/// ```
/// # use rok_pos_pharmacy::navigation;
/// assert_eq!(navigation::label("/till"), "Dispensary till");
/// assert_eq!(navigation::label("/nowhere"), "Pharmacy");
/// ```
#[must_use]
pub fn label(href: &str) -> &'static str {
    find(href).map_or("Pharmacy", |screen| screen.label)
}

fn find(href: &str) -> Option<&'static Screen> {
    ITEMS.iter().find(|screen| screen.href == href)
}

#[cfg(test)]
mod tests {
    use super::{GROUPS, ITEMS, SidebarCounts, TILL, groups, item, label, till_action};
    use rok_pos_shell::Tone;
    use std::collections::HashSet;
    use std::path::Path;

    const ALL: fn(&str) -> bool = |_| true;
    const NONE: fn(&str) -> bool = |_| false;

    #[test]
    fn every_screen_but_the_till_is_in_exactly_one_group() {
        let paths: HashSet<&str> = ITEMS.iter().map(|screen| screen.href).collect();
        assert_eq!(paths.len(), ITEMS.len(), "two screens share a path");
        let grouped: Vec<&str> = GROUPS
            .iter()
            .flat_map(|(_, hrefs)| hrefs.iter().copied())
            .collect();
        let unique: HashSet<&str> = grouped.iter().copied().collect();
        assert_eq!(unique.len(), grouped.len(), "a screen is in two groups");
        for href in &grouped {
            assert!(
                paths.contains(href),
                "{href} is in a group but is not a screen"
            );
        }
        assert_eq!(grouped.len(), ITEMS.len() - 1);
        assert!(!unique.contains(TILL), "the till is the button, not a row");
    }

    #[test]
    fn the_groups_and_rows_are_the_boards() {
        let drawn: Vec<(String, Vec<String>)> = groups(&ALL, &SidebarCounts::default())
            .iter()
            .map(|group| {
                (
                    group.label().to_string(),
                    group
                        .items()
                        .iter()
                        .map(|row| row.label().to_string())
                        .collect(),
                )
            })
            .collect();
        assert_eq!(
            drawn,
            [
                (
                    "Dispensary".to_string(),
                    vec!["Dashboard", "Prescriptions", "Patients", "Refills due"]
                        .into_iter()
                        .map(String::from)
                        .collect::<Vec<_>>()
                ),
                (
                    "Medicines & stock".to_string(),
                    [
                        "Medicines",
                        "Batches & expiry",
                        "Controlled register",
                        "Order medicines",
                        "Receive deliveries",
                        "Recalls"
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect()
                ),
                (
                    "Money & rules".to_string(),
                    ["Insurance claims", "Licences & inspection", "Reports"]
                        .into_iter()
                        .map(String::from)
                        .collect()
                ),
            ]
        );
    }

    #[test]
    fn counts_become_badges_in_the_boards_tones() {
        let counts = SidebarCounts {
            prescriptions_waiting: Some(6),
            batches_needing_action: Some(7),
            open_recalls: Some(1),
            refills_due: Some(0),
            ..SidebarCounts::default()
        };
        let badge = |href: &str| {
            item(href, &ALL, &counts).and_then(|row| {
                row.badge_value()
                    .map(|badge| (badge.text.to_string(), badge.tone))
            })
        };
        assert_eq!(badge("/prescriptions"), Some(("6".into(), Tone::Brand)));
        assert_eq!(badge("/batches"), Some(("7".into(), Tone::Warning)));
        assert_eq!(badge("/recalls"), Some(("1".into(), Tone::Danger)));
        assert_eq!(badge("/refills"), None, "zero draws no badge");
        assert_eq!(badge("/claims"), None, "an unknown count draws no badge");
        assert_eq!(badge("/"), None);
    }

    #[test]
    fn a_permission_gates_its_screen_and_nothing_else() {
        let counts = SidebarCounts::default();
        assert!(item("/", &NONE, &counts).is_some());
        assert!(item("/recalls", &NONE, &counts).is_some());
        for screen in ITEMS.iter().filter(|screen| screen.permission.is_some()) {
            assert!(
                item(screen.href, &NONE, &counts).is_none(),
                "{} is behind a permission",
                screen.href
            );
        }
        assert!(till_action(&NONE).is_none());
        assert_eq!(till_action(&ALL).expect("the till").href, TILL);
    }

    #[test]
    fn a_list_stays_lit_on_the_screens_below_it() {
        let counts = SidebarCounts::default();
        for href in ["/prescriptions", "/patients", "/receive", "/recalls"] {
            assert!(
                !item(href, &ALL, &counts).expect("a row").is_exact(),
                "{href}"
            );
        }
        assert!(item("/", &ALL, &counts).expect("dashboard").is_exact());
    }

    #[test]
    fn every_permission_is_one_a_module_declares() {
        let modules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../database/modules");
        let mut declared = String::new();
        for entry in std::fs::read_dir(&modules).expect("the modules folder can be read") {
            let manifest = entry.expect("a module folder").path().join("module.toml");
            if let Ok(text) = std::fs::read_to_string(manifest) {
                declared.push_str(&text);
            }
        }
        for screen in ITEMS {
            if let Some(permission) = screen.permission {
                assert!(
                    declared.contains(&format!("key = \"{permission}\"")),
                    "{} needs {permission}, which no module declares",
                    screen.href
                );
            }
        }
    }

    #[test]
    fn every_screen_has_its_label() {
        assert_eq!(label("/nowhere"), "Pharmacy");
        for screen in ITEMS {
            assert_eq!(label(screen.href), screen.label);
        }
    }
}
