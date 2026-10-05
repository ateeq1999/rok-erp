//! The pharmacy's screens, one module per board in `design/pharmacy`.
//!
//! Each screen is a plain function that draws what its board draws. Where a
//! screen needs a query, a procedure or a rule, its module grows the
//! `queries.rs`, `procedures.rs` or `rules.rs` half of the plan's folder; until
//! then it is the board's frame with a note saying which board it is growing
//! from.

pub mod batches_and_expiry;
pub mod board;
pub mod controlled_register;
pub mod dispensary_till;
pub mod insurance_claims;
pub mod licences_and_inspection;
pub mod medicines;
pub mod order_medicines;
pub mod placeholder;
pub mod recalls;
pub mod receive_delivery;
pub mod refills;
pub mod reports;
