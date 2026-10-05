//! The queue's repository, reading the board's own rows.
//!
//! Phase 4 replaces this with the day's prescriptions from the database. The
//! `BLoC` only knows the trait, so nothing above this file changes when it does.

use std::future::Future;

use super::models::QueueRecord;
use super::repository::{PrescriptionRepository, RepositoryError};
use super::story;
use crate::features::prescriptions::domain::entities::PrescriptionQueue;

/// Reads a day's queue from the board's figures.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StoryPrescriptionRepository;

impl PrescriptionRepository for StoryPrescriptionRepository {
    fn get_queue(
        &self,
        _branch_id: &str,
        selected: &str,
    ) -> impl Future<Output = Result<PrescriptionQueue, RepositoryError>> + Send {
        let mut record: QueueRecord = story::today();
        if !selected.is_empty() {
            record.selected = selected.to_string();
        }
        std::future::ready(Ok(into_queue(record)))
    }
}

/// The record as the entity it converts into.
fn into_queue(record: QueueRecord) -> PrescriptionQueue {
    PrescriptionQueue::from(record)
}

/// A source that is not there yet, so the error state can be drawn and tested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnavailablePrescriptionRepository {
    /// Why the source cannot be read.
    pub detail: &'static str,
}

impl PrescriptionRepository for UnavailablePrescriptionRepository {
    fn get_queue(
        &self,
        _branch_id: &str,
        _selected: &str,
    ) -> impl Future<Output = Result<PrescriptionQueue, RepositoryError>> + Send {
        std::future::ready(Err(RepositoryError::Unavailable(self.detail.to_string())))
    }
}
