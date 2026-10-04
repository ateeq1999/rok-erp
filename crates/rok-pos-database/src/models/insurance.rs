//! The monthly batches a pharmacy submits to an insurer, and how much of each
//! batch was paid, queried or deducted.

use chrono::{DateTime, NaiveDate, Utc};
use rok_db::{DbEnum, Model};
use rok_pos_domain::Money;
use uuid::Uuid;

/// Where a monthly batch has got to, matching the `status` check constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum)]
pub enum ClaimBatchStatus {
    /// Open for the period: October builds as sales happen.
    Open,
    /// Sent to the insurer with a total.
    Submitted,
    /// The insurer has queried part of it and wants fixes.
    Queried,
    /// The insurer has paid, with deductions recorded.
    Paid,
}

/// A month's claims to one insurer, submitted together and matched to
/// payment. September's batch on the board is 214 claims and 4,862,300.
#[derive(Debug, Clone, PartialEq, Model)]
#[rok(table = "pharmacy.insurance_claim_batches", timestamps, soft_delete)]
pub struct InsuranceClaimBatch {
    /// The row's identity.
    #[rok(primary_key, generated)]
    pub id: Uuid,
    /// The business this batch belongs to.
    #[rok(tenant)]
    pub organization_id: Uuid,
    /// Who the claims go to.
    pub insurance_provider_id: Uuid,
    /// The first day of the period.
    pub period_start_on: NaiveDate,
    /// The last day of the period.
    pub period_end_on: NaiveDate,
    /// Where the batch has got to.
    pub status: ClaimBatchStatus,
    /// When it went to the insurer: 30 Sep for September's batch.
    pub submitted_at: Option<DateTime<Utc>>,
    /// What was claimed across the batch.
    pub claimed_amount: Money,
    /// What the insurer paid against it.
    pub paid_amount: Money,
    /// What the insurer kept back, which payment matching fills in.
    pub deducted_amount: Money,
    /// When the row was created.
    pub created_at: DateTime<Utc>,
    /// When the row last changed.
    pub updated_at: DateTime<Utc>,
    /// When the row was removed, if it was.
    pub deleted_at: Option<DateTime<Utc>>,
    /// The optimistic lock.
    #[rok(version)]
    pub row_version: i64,
}
