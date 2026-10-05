//! The check's repository, reading the board's own prescription.
//!
//! Phase 7 replaces this with the recorded checks from the database. The
//! `BLoC` only knows the trait, so nothing above this file changes when it does.

use std::future::Future;

use super::models::CheckRecordSet;
use super::repository::{ClinicalCheckRepository, RepositoryError};
use super::story;
use crate::features::clinical_check::domain::entities::ClinicalCheck;

/// Reads one prescription's check from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryClinicalCheckRepository;

impl ClinicalCheckRepository for StoryClinicalCheckRepository {
    fn get_check(
        &self,
        _branch_id: &str,
        prescription_id: &str,
    ) -> impl Future<Output = Result<ClinicalCheck, RepositoryError>> + Send {
        let mut record: CheckRecordSet = story::rx_2214();
        if !prescription_id.is_empty() {
            record.reference = prescription_id.to_string();
        }
        std::future::ready(Ok(into_check(record)))
    }
}

/// The record as the entity it converts into.
fn into_check(record: CheckRecordSet) -> ClinicalCheck {
    ClinicalCheck::from(record)
}

/// A source that is not there yet, so the error state can be drawn and tested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnavailableClinicalCheckRepository {
    /// Why the source cannot be read.
    pub detail: &'static str,
}

impl ClinicalCheckRepository for UnavailableClinicalCheckRepository {
    fn get_check(
        &self,
        _branch_id: &str,
        _prescription_id: &str,
    ) -> impl Future<Output = Result<ClinicalCheck, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}
