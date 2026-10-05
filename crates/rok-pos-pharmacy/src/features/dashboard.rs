//! The pharmacy dashboard: today's sales, the prescriptions moving, what needs
//! the pharmacist now, and how this branch is doing against the other one.
//!
//! The feature is four layers that only ever point one way:
//!
//! ```text
//! presentation -> application -> domain <- data
//! ```
//!
//! The domain holds the figures and the arithmetic over them. The data layer
//! turns a source's own records into those figures. The application layer moves
//! state: an event in, a state out. The presentation draws one board per state
//! and dispatches events; it never reads a source.

pub mod application;
pub mod data;
pub mod domain;
pub(crate) mod presentation;

#[cfg(test)]
mod tests;

pub use presentation::dashboard_page::DashboardPage;
