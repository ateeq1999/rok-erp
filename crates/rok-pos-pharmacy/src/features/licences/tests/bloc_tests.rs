//! The folder's state machine: what an event does to the state.

use gpui::TestAppContext;

use crate::features::licences::application::errors::LicenceError;
use crate::features::licences::application::licence_bloc::LicenceBloc;
use crate::features::licences::application::licence_event::LicenceEvent;
use crate::features::licences::application::licence_state::LicenceState;
use crate::features::licences::application::licence_use_cases::ReadLicences;
use crate::features::licences::data::repository::RepositoryError;
use crate::features::licences::data::repository_impl::{
    StoryLicencesRepository, UnavailableLicencesRepository,
};

#[test]
fn a_new_bloc_has_read_nothing() {
    let bloc = LicenceBloc::new(StoryLicencesRepository);
    assert_eq!(bloc.state(), &LicenceState::Initial);
    assert!(!bloc.state().is_busy());
}

#[gpui::test]
fn the_first_load_puts_the_folder_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = LicenceBloc::new(StoryLicencesRepository);
        bloc.dispatch(LicenceEvent::Load).await;
        let state = bloc.state().clone();
        assert!(state.is_loaded());
        assert!(!state.is_busy());
        let licences = state.licences().expect("a folder on screen");
        assert_eq!(licences.licences.len(), 5);
        assert!(licences.renewal().is_some());
    });
}

#[gpui::test]
fn a_retry_after_a_failure_recovers_the_folder(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut broken = LicenceBloc::new(UnavailableLicencesRepository {
            detail: "the folder store is not reachable",
        });
        broken.dispatch(LicenceEvent::Load).await;
        assert!(broken.state().failure().is_some());

        let mut recovered = LicenceBloc::new(StoryLicencesRepository);
        recovered.dispatch(LicenceEvent::Retry).await;
        assert!(recovered.state().licences().is_some());
    });
}

#[gpui::test]
fn the_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let broken = ReadLicences::new(UnavailableLicencesRepository { detail: "no route" });
        let error = broken.execute().await.unwrap_err();
        assert_eq!(error.message(), "The folder could not be read. no route");

        let reading = ReadLicences::new(StoryLicencesRepository);
        assert!(reading.execute().await.is_ok());
    });
}

#[test]
fn a_shaped_error_is_its_own_message() {
    let error = RepositoryError::Unexpected("an expiry became a number".to_string());
    let mapped = LicenceError::from(error);
    assert!(mapped.message().contains("shape this screen cannot read"));
}

#[gpui::test]
fn a_failed_read_leaves_an_error_not_an_empty_folder(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = LicenceBloc::new(UnavailableLicencesRepository {
            detail: "the folder is not reachable",
        });
        bloc.dispatch(LicenceEvent::Load).await;
        let state = bloc.state().clone();
        assert!(
            state
                .failure()
                .is_some_and(|message| message.contains("not reachable"))
        );
        assert_eq!(state.licences(), None);
        assert!(!state.is_busy());
    });
}
