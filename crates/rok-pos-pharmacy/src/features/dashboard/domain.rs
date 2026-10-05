//! What the dashboard is: the figures of one branch's day, and the arithmetic
//! over them.
//!
//! Nothing here knows it will be drawn, or where it came from. No `gpui`, no
//! `rok_ui`, no shell tone, no router, no SQL.

pub mod calculations;
pub mod entities;
pub mod enums;
pub mod measures;
