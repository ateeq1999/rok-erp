//! The record's and the list's state machines: what an event does to the state.

use gpui::TestAppContext;

use crate::features::patient_profile::application::errors::PatientError;
use crate::features::patient_profile::application::list_bloc::ListBloc;
use crate::features::patient_profile::application::list_event::ListEvent;
use crate::features::patient_profile::application::list_state::ListState;
use crate::features::patient_profile::application::list_use_cases::ListDirectory;
use crate::features::patient_profile::application::record_bloc::RecordBloc;
use crate::features::patient_profile::application::record_event::RecordEvent;
use crate::features::patient_profile::application::record_state::RecordState;
use crate::features::patient_profile::application::record_use_cases::GetPatient;
use crate::features::patient_profile::data::repository::RepositoryError;
use crate::features::patient_profile::data::repository_impl::{
    StoryDirectoryRepository, StoryPatientRepository, UnavailablePatientRepository,
};

#[test]
fn a_new_record_bloc_has_read_nothing() {
    let bloc = RecordBloc::new(StoryPatientRepository, "P-1042");
    assert_eq!(bloc.state(), &RecordState::Initial);
    assert_eq!(bloc.patient_id(), "P-1042");
}

#[gpui::test]
fn the_first_load_puts_the_patient_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = RecordBloc::new(StoryPatientRepository, "P-1042");
        bloc.dispatch(RecordEvent::Load).await;
        let state = bloc.state().clone();
        let patient = state.patient().expect("a patient on screen");
        assert_eq!(&*patient.name, "Mzee Salim R.");
    });
}

#[gpui::test]
fn a_retry_after_a_failure_recovers_the_record(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut broken = RecordBloc::new(
            UnavailablePatientRepository {
                detail: "the record store is not reachable",
            },
            "P-1042",
        );
        broken.dispatch(RecordEvent::Load).await;
        assert!(broken.state().failure().is_some());

        let mut recovered = RecordBloc::new(StoryPatientRepository, "P-1042");
        recovered.dispatch(RecordEvent::Retry).await;
        assert!(recovered.state().patient().is_some());
    });
}

#[gpui::test]
fn the_record_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let broken = GetPatient::new(UnavailablePatientRepository { detail: "no route" });
        let error = broken.execute("P-1042").await.unwrap_err();
        assert_eq!(
            error.message(),
            "The patient's record could not be read. no route"
        );

        let reading = GetPatient::new(StoryPatientRepository);
        assert!(reading.execute("P-1042").await.is_ok());
    });
}

#[test]
fn a_shaped_error_is_its_own_message() {
    let error = RepositoryError::Unexpected("a birthday became a number".to_string());
    let mapped = PatientError::from(error);
    assert!(mapped.message().contains("shape this screen cannot read"));
}

#[test]
fn a_new_list_bloc_has_read_nothing() {
    let bloc = ListBloc::new(StoryDirectoryRepository);
    assert_eq!(bloc.state(), &ListState::Initial);
}

#[gpui::test]
fn the_first_load_puts_the_directory_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = ListBloc::new(StoryDirectoryRepository);
        bloc.dispatch(ListEvent::Load).await;
        let state = bloc.state().clone();
        assert!(state.is_loaded());
        let listings = state.listings().expect("a directory on screen");
        assert_eq!(listings.len(), 7);
        assert!(listings[0].needs_attention(), "most urgent first");
    });
}

#[gpui::test]
fn a_refresh_keeps_the_list_and_puts_a_new_one_back(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = ListBloc::new(StoryDirectoryRepository);
        bloc.dispatch(ListEvent::Load).await;
        let first = bloc.state().listings().expect("a list").clone();

        bloc.dispatch(ListEvent::Refresh).await;
        assert!(bloc.state().is_loaded());
        assert_eq!(bloc.state().listings().expect("a list again"), &first);
    });
}

#[gpui::test]
fn a_refresh_in_flight_keeps_the_list_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = ListBloc::new(StoryDirectoryRepository);
        bloc.dispatch(ListEvent::Load).await;
        let listings = bloc.state().listings().expect("a list").clone();

        let in_flight = ListState::Refreshing { listings };
        assert!(in_flight.is_busy());
        assert!(in_flight.listings().is_some());
        assert!(!in_flight.is_loaded());
    });
}

#[gpui::test]
fn a_failed_list_read_leaves_an_error_not_an_empty_list(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = ListBloc::new(UnavailablePatientRepository {
            detail: "the directory is not reachable",
        });
        bloc.dispatch(ListEvent::Load).await;
        let state = bloc.state().clone();
        assert!(
            state
                .failure()
                .is_some_and(|message| message.contains("not reachable"))
        );
        assert_eq!(state.listings(), None);
        assert!(!state.is_busy());
    });
}

#[gpui::test]
fn the_list_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let broken = ListDirectory::new(UnavailablePatientRepository { detail: "no route" });
        let error = broken.execute().await.unwrap_err();
        assert_eq!(
            error.message(),
            "The patient's record could not be read. no route"
        );

        let reading = ListDirectory::new(StoryDirectoryRepository);
        assert!(reading.execute().await.is_ok());
    });
}
