//! Which business a query belongs to, and how that is told to PostgreSQL.
//!
//! Every screen reads and writes through [`in_business`]. It opens one
//! transaction, tells the database which organization and user the work belongs
//! to, and runs the caller's work inside it. Two things then hold the line:
//!
//! - the ORM tenant filter, which reads [`rok_db::tenant::with_tenant`] and adds
//!   `organization_id` to every query on a `#[rok(tenant)]` column;
//! - row level security, which reads `app.organization_id` and hides another
//!   business's rows even from raw SQL.
//!
//! The app connects as a role that owns no tables and has no `BYPASSRLS`, so the
//! policies apply to it and it cannot turn them off. Migrations run as the owner
//! role, and `force row level security` keeps the owner honest too.
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

use rok_db::tenant;
use rok_db::{BoxFuture, Db, Result, Tx};
use uuid::Uuid;

/// Who is working, and for which business and branch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessSession {
    /// The business every query is scoped to.
    pub organization_id: Uuid,
    /// The branch the person is standing in.
    pub branch_id: Uuid,
    /// The signed-in person.
    pub user_id: Uuid,
    /// Their name, for audit rows and greetings.
    pub display_name: String,
}

impl BusinessSession {
    /// A session for a person, a branch and a business.
    #[must_use]
    pub fn new(organization_id: Uuid, branch_id: Uuid, user_id: Uuid, display_name: &str) -> Self {
        Self {
            organization_id,
            branch_id,
            user_id,
            display_name: display_name.to_string(),
        }
    }

    /// `true` when `organization_id` is the business this session belongs to.
    #[must_use]
    pub fn is_in(&self, organization_id: Uuid) -> bool {
        self.organization_id == organization_id
    }
}

/// Run `work` in a transaction that belongs to `session`.
///
/// The transaction sets `app.organization_id` and `app.user_id` for row level
/// security, and the whole call runs inside [`rok_db::tenant::with_tenant`] so
/// the ORM filter knows the business too.
///
/// # Errors
///
/// Fails when the transaction cannot be opened, when the settings cannot be
/// applied, or with whatever `work` returns; the transaction is rolled back in
/// both cases.
pub async fn in_business<T, F>(database: &Db, session: &BusinessSession, work: F) -> Result<T>
where
    F: for<'transaction> FnOnce(&'transaction mut Tx) -> BoxFuture<'transaction, Result<T>> + Send,
    T: Send,
{
    let organization_id = session.organization_id;
    let user_id = session.user_id;
    tenant::with_tenant(organization_id, async move {
        let mut transaction = database.begin().await?;
        let result = async {
            rok_db::raw(
                "select set_config('app.organization_id', ?, true), \
                 set_config('app.user_id', ?, true)",
            )
            .bind(organization_id.to_string())
            .bind(user_id.to_string())
            .execute(&mut transaction)
            .await?;
            work(&mut transaction).await
        }
        .await;

        match result {
            Ok(value) => {
                transaction.commit().await?;
                Ok(value)
            }
            Err(error) => {
                transaction.rollback().await?;
                Err(error)
            }
        }
    })
    .await
}
