//! A month of claims to one insurer, sent and paid as one batch.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use rok_pos_domain::Money;
use uuid::Uuid;

use crate::models::all_values;

/// Where a claim batch has got to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DbEnum)]
pub enum ClaimBatchStatus {
    /// Claims are still being added.
    Open,
    /// Sent to the insurer.
    Submitted,
    /// The insurer queried some of its claims.
    Queried,
    /// Paid.
    Paid,
}
all_values!(ClaimBatchStatus: Open, Submitted, Queried, Paid);

/// One insurer's claims for one period: `pharmacy.insurance_claim_batches`.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.insurance_claim_batches", timestamps, soft_delete)]
pub struct InsuranceClaimBatch {
    /// The row's key.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business it belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// The insurer.
    pub insurance_provider_id: Uuid,
    /// The first day it covers.
    pub period_start_on: NaiveDate,
    /// The last day it covers.
    pub period_end_on: NaiveDate,
    /// Where it has got to.
    pub status: ClaimBatchStatus,
    /// When it was sent.
    pub submitted_at: Option<DateTime<Utc>>,
    /// What was claimed.
    pub claimed_amount: Money,
    /// What the insurer paid.
    pub paid_amount: Money,
    /// What the insurer held back.
    pub deducted_amount: Money,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When it last changed.
    pub updated_at: DateTime<Utc>,
    /// When it was deleted, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// Bumped on every change.
    #[rok(version)]
    pub row_version: i64,
}
