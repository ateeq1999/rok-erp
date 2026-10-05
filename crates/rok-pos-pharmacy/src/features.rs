//! The pharmacy's features, one folder each.
//!
//! A feature owns its domain, its data access, its state machine, its screen
//! and its tests. Nothing outside a feature needs its widgets, and no feature
//! reaches into another feature's presentation.

pub mod clinical_check;
pub mod dashboard;
pub mod prescriptions;

pub use clinical_check::ClinicalCheckPage;
pub use dashboard::DashboardPage;
pub use prescriptions::PrescriptionsPage;
