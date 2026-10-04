//! Every pharmacy screen, the frame they sit in, and the navigation that
//! reaches them.
//!
//! The app owns the window and the routes; this crate owns what a route shows.
//! Until a screen's phase gives it a query, it draws the board's own figures
//! from [`story`].

pub mod frame;
pub mod navigation;
pub mod screens;
pub mod story;
