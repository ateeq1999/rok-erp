//! How the dashboard is drawn.
//!
//! The page dispatches events, the screen picks a board for the state it is
//! given, and the widgets draw figures they are handed. A widget reads no
//! source and derives no figure.

pub mod dashboard_page;
pub mod dashboard_screen;
pub mod styles;
pub(crate) mod widgets;
