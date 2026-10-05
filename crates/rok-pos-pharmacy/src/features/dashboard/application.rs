//! What the dashboard does when something happens to it.
//!
//! This layer moves state. It reads no source of its own beyond the use case,
//! it draws nothing and it names no route.

pub mod dashboard_bloc;
pub mod dashboard_event;
pub mod dashboard_state;
pub mod dashboard_use_cases;
pub mod errors;
