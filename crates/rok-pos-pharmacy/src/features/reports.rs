//! Reports with the pharmacy's filters: sales by medicine and schedule, margin,
//! expiry losses, claims aging and controlled movements.
//!
//! Board: `OfficeReports`, built in Phase 13. Every report is a table of rows
//! that add up to the figure in its heading, which is what makes a report
//! worth looking at: the headline is a sum, not a number someone typed. The
//! board's own figures live in [`data::story`]; the phase that reports over
//! the database reads them from there.

pub mod application;
pub mod data;
pub mod domain;
pub(crate) mod presentation;

#[cfg(test)]
mod tests;

pub use presentation::report_page::ReportsPage;
