//! The pharmacy's frame: the sidebar the window keeps, and the page each route
//! draws inside it, both filled the same way so no route repeats them.

use rok_pos_shell::{Page, Sidebar};
use rok_ui::prelude::SharedString;
use rust_i18n::t;

use crate::{navigation, story};

/// What the signed-in user may do. Grace N. is the pharmacist in charge, who may
/// do everything; Phase 3 (sign-in and roles) replaces this with the session's
/// permissions.
#[must_use]
pub fn can(_permission: &str) -> bool {
    true
}

/// The sidebar, with the rows and counts the signed-in user may see.
pub fn sidebar() -> Sidebar {
    let mut sidebar = Sidebar::new(story::BUSINESS, story::BRANCH)
        .groups(navigation::groups(&can, &story::sidebar_counts()))
        .user(story::signed_in());
    if let Some(till) = navigation::till_action(&can) {
        sidebar = sidebar.action(till);
    }
    sidebar
}

/// The page for the screen at `href`: its heading, the business and branch
/// under it (and the time on the dashboard), and the top bar's status.
///
/// ```
/// # use rok_pos_pharmacy::frame;
/// let page = frame::page("/medicines");
/// ```
pub fn page(href: &str) -> Page {
    framed(navigation::label(href), href == "/")
}

/// A page whose heading is not a sidebar label: a patient's record, one recall.
pub fn titled(heading: impl Into<SharedString>) -> Page {
    framed(heading, false)
}

/// The record page's heading, in the session's language, for the route that
/// frames the patient feature.
#[must_use]
pub fn record_heading() -> SharedString {
    t!("patient.record.page_title").to_string().into()
}

fn framed(heading: impl Into<SharedString>, with_time: bool) -> Page {
    Page::new(heading, story::subheading(with_time))
        .actions(vec![crate::locale::toggle()])
        .status(story::sync_status())
        .unread(true)
}
