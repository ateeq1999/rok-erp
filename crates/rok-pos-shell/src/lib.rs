//! The shell every Afya Pharmacy screen sits in: the window frame, the
//! navigation column, the top bar, and the small pieces of formatting the
//! boards are particular about.
//!
//! The shell draws and formats. It does not know what a prescription is, what a
//! batch is worth or what a claim is owed: those rules live in
//! `rok-pos-domain` and in `crates/rok-pos-pharmacy`. An app supplies its own
//! destinations, brand and user, so the supplier app can use the same frame
//! with its own accent.
//!
//! ```
//! use rok_ui::prelude::*;
//! use rok_pos_shell::{AppFrame, NavGroup, NavItem, PageHeader, PageMain, Sidebar, TopBar};
//!
//! let window = AppFrame::new(Sidebar::new("Afya Pharmacy", "Mwenge branch").groups(vec![
//!     NavGroup::new("Dispensary", [NavItem::new("Dashboard").href("/")]),
//! ]).into_any_element())
//! .child(TopBar::new("Dashboard", "Afya Pharmacy - Mwenge branch"))
//! .child(PageMain::new().child(PageHeader::new("Dashboard", "Tuesday, 4 August")));
//! ```

pub mod app_frame;
pub mod assistant_button;
pub mod batch_code_text;
pub mod chip;
pub mod digits;
pub mod fonts;
pub mod money_text;
pub mod page;
pub mod page_header;
pub mod page_main;
pub mod sidebar;
pub mod theme;
pub mod tone;
pub mod top_bar;

pub use app_frame::AppFrame;
pub use assistant_button::AssistantButton;
pub use batch_code_text::{BatchCodeText, format_batch_line, format_expiry};
pub use chip::Chip;
pub use digits::{NumberText, group_digits};
pub use fonts::{NUMBERS_FAMILY, TEXT_FAMILY};
pub use money_text::{CURRENCY, MoneyText, format_amount, format_money, format_money_precise};
pub use page::Page;
pub use page_header::PageHeader;
pub use page_main::PageMain;
pub use sidebar::{NavBadge, NavGroup, NavItem, Sidebar, SidebarAction, UserChip};
pub use theme::{install_theme, pharmacy_theme, sync_with_system_appearance};
pub use tone::{Tone, ToneColors};
pub use top_bar::{TopBar, TopBarStatus};
