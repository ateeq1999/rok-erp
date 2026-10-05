//! The arithmetic over today's prescriptions.
//!
//! Every count the queue shows is derived from the queue itself, so a status box
//! cannot claim a number the table does not have.

use super::entities::{PrescriptionQueue, Queued};
use super::enums::Status;

/// How many rows a test kept, as the count the queue uses. A day cannot hold
/// more prescriptions than a `u32` holds, but the conversion says so rather than
/// assuming it.
fn count_of(kept: impl Iterator<Item = bool>) -> u32 {
    u32::try_from(kept.filter(|row| *row).count()).unwrap_or(u32::MAX)
}

/// One of the five boxes in "Today's queue by status".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusCount {
    /// Which state it counts.
    pub status: Status,
    /// How many are in it.
    pub count: u32,
}

/// How many of today's prescriptions are in `status`.
#[must_use]
pub fn count_in(queue: &PrescriptionQueue, status: Status) -> u32 {
    count_of(queue.queue.iter().map(|queued| queued.status == status))
}

/// The five boxes, in the board's order, counted from the queue.
#[must_use]
pub fn status_counts(queue: &PrescriptionQueue) -> Vec<StatusCount> {
    Status::ALL
        .into_iter()
        .map(|status| StatusCount {
            status,
            count: count_in(queue, status),
        })
        .collect()
}

/// How many prescriptions are still to be dispensed.
#[must_use]
pub fn waiting(queue: &PrescriptionQueue) -> u32 {
    count_of(
        queue
            .queue
            .iter()
            .map(|queued| !queued.status.is_finished()),
    )
}

/// How many prescriptions carry at least one flag, which is what the pharmacist
/// works through first.
#[must_use]
pub fn flagged(queue: &PrescriptionQueue) -> u32 {
    count_of(queue.queue.iter().map(Queued::is_flagged))
}

/// How many have left the pharmacy today.
#[must_use]
pub fn dispensed(queue: &PrescriptionQueue) -> u32 {
    count_in(queue, Status::Dispensed)
}

/// How many arrived as a photo that still has to be read.
#[must_use]
pub fn unread_photos(queue: &PrescriptionQueue) -> u32 {
    count_of(
        queue
            .queue
            .iter()
            .map(|queued| queued.source.needs_reading() && queued.status == Status::New),
    )
}

/// The prescription the rail opens, which is the one the queue says it does.
#[must_use]
pub fn selected(queue: &PrescriptionQueue) -> Option<&super::entities::Queued> {
    queue
        .queue
        .iter()
        .find(|queued| queued.reference == queue.selected)
}

/// Whether every box adds up to the queue's rows, which is what makes the
/// five tiles worth reading.
#[must_use]
pub fn counts_add_up(queue: &PrescriptionQueue) -> bool {
    let boxed: u32 = status_counts(queue).iter().map(|box_| box_.count).sum();
    boxed == u32::try_from(queue.queue.len()).unwrap_or(u32::MAX)
}
