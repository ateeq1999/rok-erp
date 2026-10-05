//! The pharmacy's features, one folder each.
//!
//! A feature owns its domain, its data access, its state machine, its screen
//! and its tests. Nothing outside a feature needs its widgets, and no feature
//! reaches into another feature's presentation.

pub mod dashboard;

pub use dashboard::DashboardPage;
