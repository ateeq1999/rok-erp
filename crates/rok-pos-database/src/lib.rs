//! PostgreSQL access for rok POS: business sessions, the module installer, and
//! the models the screens read.
//!
//! Two things live here that belong to no screen.
//!
//! [`business_session`] is how every query says which business it is for: the
//! tenant filter sets `app.organization_id`, the transaction sets
//! `app.user_id`, and row level security catches whatever either one misses.
//!
//! [`module_installer`] is how the database itself is built: one command reads
//! every `database/modules/<key>/module.toml`, puts the modules in dependency
//! order, and runs each module's pending migrations, recording a checksum for
//! every one so a migration that has already run can never change underneath
//! the databases that ran it.
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
pub mod module_installer;
pub mod module_manifest;
pub mod models;

pub use business_session::{BusinessSession, in_business};
pub use module_installer::{InstallReport, InstallRequest, install, revert_all};
pub use module_manifest::{ModuleError, ModuleManifest, ModuleSource};
