//! PostgreSQL access for rok POS: business sessions, and the models the screens
//! read.
//!
//! The one thing that lives here and belongs to no screen is
//! [`business_session`]: how every query says which business it is for. The
//! tenant filter sets `app.organization_id`, the transaction sets
//! `app.user_id`, and row level security catches whatever either one misses.
//!
//! ```
//! use uuid::Uuid;
//! use rok_pos_database::BusinessSession;
//!
//! let afya = Uuid::now_v7();
//! let session = BusinessSession::new(afya, Uuid::now_v7(), Uuid::now_v7(), "Grace N.");
//!
//! assert!(session.is_in(afya), "a session is in its own business");
//! assert!(!session.is_in(Uuid::nil()), "and in no other");
//! ```

pub mod business_session;

pub use business_session::{BusinessSession, in_business};
