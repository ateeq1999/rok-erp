//! The rules the pharmacy and the supplier share: money today, quantities,
//! identifiers and schedules as later phases need them.
//!
//! Nothing here touches the database, the clock or the screen. A value that a
//! rule needs is passed in, so every rule is a plain function with unit tests
//! and no fixtures.

pub mod error;
pub mod money;
#[cfg(feature = "postgres")]
pub mod money_postgres;

pub use error::DomainError;
pub use money::Money;
