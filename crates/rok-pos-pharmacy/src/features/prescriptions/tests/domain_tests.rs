//! The queue's rules, against the board's own rows.

use crate::features::prescriptions::data::models::QueueRecord;
use crate::features::prescriptions::data::story;
use crate::features::prescriptions::domain::calculations::{
    counts_add_up, dispensed, flagged, selected, status_counts, unread_photos, waiting,
};
use crate::features::prescriptions::domain::entities::PrescriptionQueue;
use crate::features::prescriptions::domain::enums::{Source, Status, StatusSeverity};

/// The board's day, as the entity it becomes.
fn day() -> PrescriptionQueue {
    PrescriptionQueue::from(story::today())
}

#[test]
fn the_status_boxes_add_up_to_the_rows_the_table_shows() {
    let queue = day();
    assert_eq!(queue.queue.len(), 10);
    assert!(
        counts_add_up(&queue),
        "a box claiming a row that is not there"
    );
    let boxes = status_counts(&queue);
    assert_eq!(boxes.len(), 5);
    assert_eq!(boxes[0].status, Status::New);
    assert_eq!(boxes[0].count, 2);
    assert_eq!(boxes[3].status, Status::ReadyToCollect);
    assert_eq!(boxes[3].count, 1);
}

#[test]
fn waiting_is_the_rows_that_have_not_left() {
    let queue = day();
    assert_eq!(waiting(&queue), 6);
    assert_eq!(dispensed(&queue), 4);
    assert_eq!(
        waiting(&queue) + dispensed(&queue),
        u32::try_from(queue.queue.len()).unwrap()
    );
}

#[test]
fn the_flagged_rows_are_the_ones_the_pharmacist_works_first() {
    let queue = day();
    let flagged_refs: Vec<&str> = queue
        .queue
        .iter()
        .filter(|queued| queued.is_flagged())
        .map(|queued| &*queued.reference as &str)
        .collect();
    assert_eq!(
        flagged_refs,
        [
            "RX-2217", "RX-2216", "RX-2215", "RX-2214", "RX-2218", "RX-2219", "RX-2210"
        ]
    );
    assert_eq!(flagged(&queue), 7);
}

#[test]
fn only_one_photo_is_still_waiting_to_be_read() {
    assert_eq!(unread_photos(&day()), 1);
}

#[test]
fn the_rail_opens_the_interaction_alert() {
    let queue = day();
    let open = selected(&queue).expect("the rail opens a row");
    assert_eq!(&*open.reference, story::SELECTED);
    assert_eq!(open.status, Status::NeedsCheck);
    assert!(queue.open.needs_a_pharmacist());
    assert_eq!(queue.open.labels(), 2);
    assert_eq!(queue.open.items[0].quantity, 21);
    assert_eq!(
        queue.open.out_of_stock().len(),
        0,
        "both items are in stock"
    );
}

#[test]
fn a_dispensed_prescription_is_finished_not_successful() {
    assert!(Status::Dispensed.is_finished());
    assert!(!Status::ReadyToCollect.is_finished());
    assert_eq!(Status::Dispensed.label(), "Dispensed today");
    assert_eq!(Status::NeedsCheck.severity(), StatusSeverity::Warning);
    assert_eq!(Status::Dispensed.severity(), StatusSeverity::Done);
    assert!(Source::Photo.needs_reading());
    assert!(!Source::Paper.needs_reading());
}

#[test]
fn a_day_with_nothing_in_it_has_no_rows_and_no_boxes_that_lie() {
    let empty = QueueRecord {
        range: String::new(),
        oldest_waited: String::new(),
        queue: Vec::new(),
        selected: String::new(),
        open: story::today().open,
    };
    let queue = PrescriptionQueue::from(empty);
    assert_eq!(waiting(&queue), 0);
    assert_eq!(flagged(&queue), 0);
    assert_eq!(unread_photos(&queue), 0);
    assert_eq!(status_counts(&queue).len(), 5);
    assert!(counts_add_up(&queue), "no rows means no box claims one");
    assert_eq!(selected(&queue), None);
}

#[test]
fn a_row_whose_batch_is_not_on_the_shelf_is_out_of_stock() {
    let mut record = story::today();
    record.open.items[1].availability = "Out of stock \u{b7} recalled".to_string();
    let queue = PrescriptionQueue::from(record);
    assert_eq!(queue.open.out_of_stock().len(), 1);
    assert!(!queue.open.out_of_stock()[0].in_stock);
}
