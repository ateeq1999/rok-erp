//! Every pharmacy screen, the frame they sit in, and the navigation that
//! reaches them.
//!
//! The app owns the window and the routes; this crate owns what a route shows.
//! Until a screen's phase gives it a query, it draws the board's own figures
//! from [`story`].
//!
//! The copy is English and Arabic, in `locales/`; [`locale::Language`] holds
//! the choice and the direction it draws in.

// The `t!` macro reads these at compile time. Story figures are data, not
// copy, so they stay where the story puts them.
rust_i18n::i18n!("locales", fallback = "en");

pub mod features;
pub mod frame;
pub mod locale;
pub mod navigation;
pub mod screens;
pub mod story;
