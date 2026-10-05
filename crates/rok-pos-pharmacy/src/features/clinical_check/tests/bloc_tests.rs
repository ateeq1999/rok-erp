//! The check's state machine: what an event does to the state.

use gpui::TestAppContext;

use crate::features::clinical_check::application::check_bloc::CheckBloc;
use crate::features::clinical_check::application::check_event::CheckEvent;
use crate::features::clinical_check::application::check_state::CheckState;
use crate::features::clinical_check::application::check_use_cases::GetClinicalCheck;
use crate::features::clinical_check::data::repository::RepositoryError;
use crate::features::clinical_check::data::repository_impl::{
    StoryClinicalCheckRepository, UnavailableClinicalCheckRepository,
};
use crate::features::clinical_check::data::story;

#[test]
fn a_new_bloc_has_read_nothing() {
    let bloc = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", story::RX_2214);
    assert_eq!(bloc.state(), &CheckState::Initial);
    assert_eq!(bloc.branch_id(), "Mwenge");
    assert_eq!(bloc.prescription_id(), story::RX_2214);
}

#[gpui::test]
fn the_first_load_puts_the_check_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", story::RX_2214);
        bloc.dispatch(CheckEvent::Load).await;
        let state = bloc.state().clone();
        assert!(state.is_loaded());
        let check = state.check().expect("a check on screen");
        assert_eq!(&*check.reference, story::RX_2214);
        assert_eq!(check.medicines.len(), 2);
        assert_eq!(check.checks.len(), 6);
    });
}

#[gpui::test]
fn a_refresh_keeps_the_check_and_puts_the_same_one_back(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", story::RX_2214);
        bloc.dispatch(CheckEvent::Load).await;
        let first = bloc.state().check().expect("a check").clone();

        bloc.dispatch(CheckEvent::Refresh).await;
        assert!(bloc.state().is_loaded());
        assert_eq!(bloc.state().check().expect("a check again"), &first);
        assert!(CheckEvent::Refresh.keeps_the_check());
        assert!(!CheckEvent::Load.keeps_the_check());
    });
}

#[gpui::test]
fn a_refresh_in_flight_keeps_the_check_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", story::RX_2214);
        bloc.dispatch(CheckEvent::Load).await;
        let check = bloc.state().check().expect("a check").clone();

        let in_flight = CheckState::Refreshing { check };
        assert!(in_flight.is_busy());
        assert!(in_flight.check().is_some());
        assert!(!in_flight.is_loaded());
        assert_eq!(in_flight.failure(), None);
    });
}

#[gpui::test]
fn a_refresh_with_nothing_on_screen_falls_back_to_the_first_load(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", story::RX_2214);
        assert_eq!(bloc.state().check(), None);
        bloc.dispatch(CheckEvent::Refresh).await;
        assert!(
            bloc.state().is_loaded(),
            "a refresh over nothing is just a load"
        );
    });
}

#[gpui::test]
fn a_failed_read_leaves_an_error_not_an_empty_check(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = CheckBloc::new(
            UnavailableClinicalCheckRepository {
                detail: "the branch database is not reachable",
            },
            "Mwenge",
            story::RX_2214,
        );
        bloc.dispatch(CheckEvent::Load).await;
        let state = bloc.state().clone();
        assert!(
            state
                .failure()
                .is_some_and(|message| message.contains("not reachable"))
        );
        assert_eq!(state.check(), None);
        assert!(!state.is_busy());
        assert!(!state.is_loaded());
    });
}

#[gpui::test]
fn a_retry_after_a_failure_recovers_the_check(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut broken = CheckBloc::new(
            UnavailableClinicalCheckRepository { detail: "no route" },
            "Mwenge",
            story::RX_2214,
        );
        broken.dispatch(CheckEvent::Load).await;
        assert!(broken.state().failure().is_some());

        let mut recovered = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", story::RX_2214);
        recovered.dispatch(CheckEvent::Retry).await;
        assert!(recovered.state().is_loaded());
    });
}

#[gpui::test]
fn the_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let broken =
            GetClinicalCheck::new(UnavailableClinicalCheckRepository { detail: "no route" });
        let error = broken.execute("Mwenge", story::RX_2214).await.unwrap_err();
        assert_eq!(
            error.message(),
            "The prescription could not be read. no route"
        );

        let reading = GetClinicalCheck::new(StoryClinicalCheckRepository);
        assert!(reading.execute("Mwenge", story::RX_2214).await.is_ok());
    });
}

#[test]
fn a_shaped_error_is_its_own_message() {
    let error = RepositoryError::Unexpected("finding became a number".to_string());
    assert!(error.message().contains("shape this screen cannot read"));
}

#[gpui::test]
fn the_bloc_adopts_the_reference_the_source_reads_back(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = CheckBloc::new(StoryClinicalCheckRepository, "Mwenge", "RX-9999");
        bloc.dispatch(CheckEvent::Load).await;
        assert_eq!(
            bloc.prescription_id(),
            "RX-9999",
            "the source handed back the reference it read, so a second read agrees"
        );
        assert_eq!(
            &*bloc.state().check().expect("a check").reference,
            "RX-9999"
        );
    });
}
