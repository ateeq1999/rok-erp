//! The pharmacy's navigation: what permission a screen needs, and which group
//! of the sidebar it sits in.
//!
//! The sidebar is data, so this is data too: [`ITEMS`] is every screen, and
//! [`groups`] turns it into the groups the app hands the shell. A screen behind
//! a dynamic path needs no row of its own, because a non-exact row stays lit on
//! the paths below it.

use rok_pos_shell::{NavGroup, NavItem};
use rok_ui::prelude::*;

/// One screen: what the sidebar calls it, where it opens, and who may open it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Screen {
    /// The label the sidebar shows.
    pub label: &'static str,
    /// The icon beside the label.
    pub icon: IconName,
    /// The path it opens.
    pub href: &'static str,
    /// Count as current only on this exact path, so a screen below a list does
    /// not light up the list.
    pub exact: bool,
    /// The permission the screen needs, when it needs one.
    pub permission: Option<&'static str>,
}

/// Every screen, in the order the sidebar shows them.
pub const ITEMS: [Screen; 13] = [
    Screen {
        label: "Dashboard",
        icon: IconName::Home,
        href: "/",
        exact: true,
        permission: None,
    },
    Screen {
        label: "Prescriptions",
        icon: IconName::FileText,
        href: "/prescriptions",
        exact: true,
        permission: None,
    },
    Screen {
        label: "Patients",
        icon: IconName::User,
        href: "/patients",
        exact: false,
        permission: None,
    },
    Screen {
        label: "Refills due",
        icon: IconName::Refresh,
        href: "/refills",
        exact: true,
        permission: None,
    },
    Screen {
        label: "Medicines",
        icon: IconName::Folder,
        href: "/medicines",
        exact: true,
        permission: Some("medicines.read"),
    },
    Screen {
        label: "Batches & expiry",
        icon: IconName::Calendar,
        href: "/batches",
        exact: true,
        permission: Some("medicines.read"),
    },
    Screen {
        label: "Controlled register",
        icon: IconName::File,
        href: "/controlled-register",
        exact: true,
        permission: Some("controlled.read"),
    },
    Screen {
        label: "Order medicines",
        icon: IconName::Send,
        href: "/order",
        exact: true,
        permission: Some("orders.write"),
    },
    Screen {
        label: "Receive delivery",
        icon: IconName::Download,
        href: "/receive",
        exact: false,
        permission: Some("orders.write"),
    },
    Screen {
        label: "Recalls",
        icon: IconName::TriangleAlert,
        href: "/recalls",
        exact: false,
        permission: None,
    },
    Screen {
        label: "Insurance claims",
        icon: IconName::Link,
        href: "/claims",
        exact: true,
        permission: None,
    },
    Screen {
        label: "Licences & inspection",
        icon: IconName::CircleCheck,
        href: "/licences",
        exact: true,
        permission: Some("compliance.read"),
    },
    Screen {
        label: "Till",
        icon: IconName::Circle,
        href: "/till",
        exact: true,
        permission: None,
    },
];

/// The sidebar's groups, in order: each screen's path, then the group's label.
/// The first group is the common destinations and carries no label, because the
/// boards put the dashboard and the till side by side at the top.
const GROUPS: [(&[&str], &str); 5] = [
    (&["/", "/prescriptions", "/till"], ""),
    (&["/patients", "/refills"], "Patients"),
    (
        &["/medicines", "/batches", "/controlled-register"],
        "Inventory",
    ),
    (&["/order", "/receive", "/recalls"], "Purchasing"),
    (&["/claims", "/licences"], "Records"),
];

/// The sidebar for a user whose permissions are `can`.
///
/// ```
/// # use rok_pos_pharmacy::navigation;
/// let everything = navigation::groups(&|_| true);
/// assert_eq!(everything.len(), 5);
/// assert!(navigation::groups(&|_| false).iter().all(|group| group.items().len() <= 3));
/// ```
#[must_use]
pub fn groups(can: &dyn Fn(&str) -> bool) -> Vec<NavGroup> {
    GROUPS
        .iter()
        .filter_map(|(hrefs, label)| {
            let items = hrefs
                .iter()
                .filter_map(|href| item(href, can))
                .collect::<Vec<NavItem>>();
            (!items.is_empty()).then(|| NavGroup::new(*label, items))
        })
        .collect()
}

/// The row the sidebar draws for a screen.
fn row(screen: &Screen) -> NavItem {
    NavItem::new(screen.label, screen.icon)
        .href(screen.href)
        .exact(screen.exact)
}

/// One navigation item, or nothing when `can` refuses its permission.
///
/// ```
/// # use rok_pos_pharmacy::navigation;
/// assert!(navigation::item("/", &|_| false).is_some());
/// assert!(navigation::item("/controlled-register", &|_| false).is_none());
/// assert!(navigation::item("/controlled-register", &|_| true).is_some());
/// ```
#[must_use]
pub fn item(href: &str, can: &dyn Fn(&str) -> bool) -> Option<NavItem> {
    screen(href)
        .filter(|screen| screen.permission.is_none_or(can))
        .map(row)
}

/// A screen's label, for a page header or a top bar.
///
/// ```
/// # use rok_pos_pharmacy::navigation;
/// assert_eq!(navigation::label("/till"), "Till");
/// assert_eq!(navigation::label("/nowhere"), "Pharmacy");
/// ```
#[must_use]
pub fn label(href: &str) -> &'static str {
    ITEMS
        .iter()
        .find(|screen| screen.href == href)
        .map_or("Pharmacy", |screen| screen.label)
}

/// The line under a heading: the branch, and the clock on the dashboard.
#[must_use]
pub fn subheading(href: &str) -> &'static str {
    if href == "/" {
        DASHBOARD_SUBHEADING
    } else {
        BRANCH_SUBHEADING
    }
}

/// The dashboard's second line: branch, date and time.
pub const DASHBOARD_SUBHEADING: &str =
    "Afya Pharmacy - Mwenge branch - Friday 2 October 2026, 15:30";

/// The second line every other screen uses.
pub const BRANCH_SUBHEADING: &str = "Afya Pharmacy - Mwenge branch";

fn screen(href: &str) -> Option<&'static Screen> {
    ITEMS.iter().find(|screen| screen.href == href)
}

#[cfg(test)]
mod tests {
    use super::{BRANCH_SUBHEADING, DASHBOARD_SUBHEADING, ITEMS, groups, item, label, subheading};
    use std::collections::HashSet;

    #[test]
    fn a_group_path_is_a_screen() {
        let paths: HashSet<&str> = ITEMS.iter().map(|screen| screen.href).collect();
        assert_eq!(paths.len(), ITEMS.len(), "two screens share a path");
        for (hrefs, _) in &super::GROUPS {
            for &href in *hrefs {
                assert!(
                    paths.contains(href),
                    "{href} is in a group but is not a screen"
                );
            }
        }
    }

    #[test]
    fn every_screen_is_in_exactly_one_group() {
        let grouped: HashSet<&str> = super::GROUPS
            .iter()
            .flat_map(|(hrefs, _)| hrefs.iter().copied())
            .collect();
        assert_eq!(
            grouped.len(),
            ITEMS.len(),
            "a screen is in two groups or none"
        );
    }

    #[test]
    fn the_group_labels_are_the_ones_the_sidebar_draws() {
        let labels: Vec<String> = groups(&|_| true)
            .iter()
            .map(|group| group.label().to_string())
            .collect();
        assert_eq!(
            labels,
            ["", "Patients", "Inventory", "Purchasing", "Records"]
        );
    }

    #[test]
    fn a_permission_gates_its_screen_and_nothing_else() {
        let no_rights = |_: &str| false;
        for key in [
            "/",
            "/prescriptions",
            "/patients",
            "/refills",
            "/recalls",
            "/claims",
            "/till",
        ] {
            assert!(item(key, &no_rights).is_some(), "{key} needs no permission");
        }
        for key in [
            "/medicines",
            "/batches",
            "/controlled-register",
            "/order",
            "/receive",
            "/licences",
        ] {
            assert!(
                item(key, &no_rights).is_none(),
                "{key} is behind a permission"
            );
        }
        assert!(item("/", &no_rights).expect("dashboard").is_exact());
    }

    #[test]
    fn a_list_stays_lit_on_the_screens_below_it() {
        let all = |_: &str| true;
        assert!(!item("/patients", &all).expect("patients").is_exact());
        assert!(!item("/receive", &all).expect("receive").is_exact());
        assert!(!item("/recalls", &all).expect("recalls").is_exact());
        assert!(
            item("/prescriptions", &all)
                .expect("prescriptions")
                .is_exact()
        );
    }

    #[test]
    fn labels_and_subheadings_answer_for_every_route() {
        assert_eq!(label("/till"), "Till");
        assert_eq!(label("/nowhere"), "Pharmacy");
        assert_eq!(subheading("/"), DASHBOARD_SUBHEADING);
        assert_eq!(subheading("/medicines"), BRANCH_SUBHEADING);
        for screen in ITEMS {
            assert_eq!(label(screen.href), screen.label);
        }
    }
}
