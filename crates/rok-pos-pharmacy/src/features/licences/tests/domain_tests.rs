//! The folder's rules, against the board's own figures.

use crate::features::licences::data::models::{DocumentRecord, LicenceRecord};
use crate::features::licences::data::story;
use crate::features::licences::domain::calculations::{
    self, BOARD_TODAY, days_until_renewal, held, missing_documents, not_ready, ready, renewing,
};
use crate::features::licences::domain::entities::{CivilDate, Licence, Licences, days_until};
use crate::features::licences::domain::enums::{Reminder, Standing};

/// The board's folder, as the entities it becomes.
fn folder() -> Licences {
    Licences::from(story::folder())
}

#[test]
fn a_date_counts_days_across_months_and_leaps() {
    let epoch = CivilDate::new(1970, 1, 1);
    assert_eq!(epoch.epoch_days(), 0);
    assert_eq!(days_until(epoch, CivilDate::new(1970, 1, 2)), 1);
    assert_eq!(
        days_until(CivilDate::new(2024, 2, 28), CivilDate::new(2024, 2, 29)),
        1,
        "a leap day is a day"
    );
    assert_eq!(
        days_until(CivilDate::new(2026, 11, 30), CivilDate::new(2026, 10, 4)),
        -57,
        "the difference reads backwards as well"
    );
}

#[test]
fn the_board_opens_57_days_before_the_renewal_is_due() {
    let due = CivilDate::new(2026, 11, 30);
    assert_eq!(days_until(BOARD_TODAY, due), 57);
}

#[test]
fn the_board_counts_are_derived_from_the_rows() {
    let licences = folder();
    assert_eq!(held(&licences.licences), 5);
    assert_eq!(renewing(&licences.licences), 2);
    assert_eq!(ready(&licences.readiness), 5);
    assert_eq!(not_ready(&licences.readiness), 1);
    assert_eq!(missing_documents(&licences.documents), 1);
}

#[test]
fn the_premises_licence_is_the_one_the_board_leads_with() {
    let licences = folder();
    let renewal = licences.renewal().expect("the board has a renewal");
    assert!(
        renewal.title.contains("premises"),
        "a lapsed premises licence closes the pharmacy, so it leads"
    );
    assert!(renewal.needs_renewal());
    assert_eq!(days_until_renewal(&licences, BOARD_TODAY), Some(57));
}

#[test]
fn a_licence_held_but_not_on_file_does_not_count_as_ready() {
    let licences = folder();
    let renewal = licences.renewal().expect("the board has a renewal");
    let filed = licences
        .documents
        .iter()
        .find(|document| document.name == renewal.title && document.branch == renewal.branch)
        .expect("the board files the premises licence");
    assert!(
        filed.on_file,
        "the licence itself is on file; what is missing is the renewal application"
    );
    let missing = licences
        .documents
        .iter()
        .find(|document| !document.on_file)
        .expect("one gap");
    assert!(
        missing.name.contains("renewal application"),
        "the gap is the application, not the licence"
    );
}

#[test]
fn the_application_deadline_is_stored_as_a_date() {
    let licences = folder();
    let renewal = licences.renewal().expect("the board has a renewal");
    assert_eq!(
        renewal.renewal_application_due,
        Some(CivilDate::new(2026, 11, 28))
    );
}

#[test]
fn a_misspelled_expiry_reads_as_long_lapsed_not_as_fine() {
    let mut record = story::folder();
    record.licences[0].expires_on = "sometime".to_string();
    let licences = Licences::from(record);
    let renewal = licences.renewal().expect("still a renewal by standing");
    assert!(
        renewal.days_until_expiry(BOARD_TODAY) < 0,
        "an unparsed date falls back to the epoch, which reads as lapsed"
    );
}

#[test]
fn the_folder_spells_standings_reminders_and_dates_and_the_parse_keeps_up() {
    let folder = story::folder();
    let licence = LicenceRecord {
        standing: "unknown word".to_string(),
        ..folder.licences[0].clone()
    };
    assert_eq!(
        Licence::from(licence).standing,
        Standing::Renewing,
        "an unnamed standing is one that needs acting on"
    );

    let before = DocumentRecord {
        reminder: "90 days before".to_string(),
        ..folder.documents[0].clone()
    };
    assert_eq!(
        crate::features::licences::domain::entities::Document::from(before).reminder,
        Reminder::DaysBefore(90)
    );
    let submit = DocumentRecord {
        reminder: "submit by".to_string(),
        ..folder.documents[4].clone()
    };
    assert_eq!(
        crate::features::licences::domain::entities::Document::from(submit).reminder,
        Reminder::SubmitBy
    );

    assert!(matches!(Standing::Valid, Standing::Valid));
    assert!(calculations::standing_is_valid(Standing::Valid));
    assert!(!calculations::standing_is_valid(Standing::Renewing));
}

#[test]
fn a_folder_record_round_trips_through_the_entities() {
    let licences = Licences::from(story::folder());
    assert_eq!(licences.licences.len(), 5);
    assert_eq!(licences.readiness.len(), 6);
    assert_eq!(licences.documents.len(), 5);
}
