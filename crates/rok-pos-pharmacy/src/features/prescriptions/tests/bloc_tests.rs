//! The queue's state machine: what an event does to the state.

use gpui::TestAppContext;

use crate::features::prescriptions::application::prescriptions_bloc::PrescriptionsBloc;
use crate::features::prescriptions::application::prescriptions_event::PrescriptionsEvent;
use crate::features::prescriptions::application::prescriptions_state::PrescriptionsState;
use crate::features::prescriptions::application::prescriptions_use_cases::GetPrescriptions;
use crate::features::prescriptions::data::repository::RepositoryError;
use crate::features::prescriptions::data::repository_impl::{
    StoryPrescriptionRepository, UnavailablePrescriptionRepository,
};
use crate::features::prescriptions::data::story;

#[test]
fn a_new_bloc_has_read_nothing() {
    let bloc = PrescriptionsBloc::new(StoryPrescriptionRepository, "Mwenge", story::SELECTED);
    assert_eq!(bloc.state(), &PrescriptionsState::Initial);
    assert_eq!(bloc.branch_id(), "Mwenge");
    assert_eq!(bloc.selected(), story::SELECTED);
}

#[gpui::test]
fn the_first_load_puts_the_day_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc =
            PrescriptionsBloc::new(StoryPrescriptionRepository, "Mwenge", story::SELECTED);
        bloc.dispatch(PrescriptionsEvent::Load).await;
        let state = bloc.state().clone();
        assert!(state.is_loaded());
        let queue = state.queue().expect("a queue on screen");
        assert_eq!(queue.queue.len(), 10);
        assert_eq!(&*queue.selected, story::SELECTED);
    });
}

#[gpui::test]
fn a_refresh_keeps_the_queue_and_puts_a_new_one_back(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc =
            PrescriptionsBloc::new(StoryPrescriptionRepository, "Mwenge", story::SELECTED);
        bloc.dispatch(PrescriptionsEvent::Load).await;
        let first = bloc.state().queue().expect("a queue").clone();

        bloc.dispatch(PrescriptionsEvent::Refresh).await;
        assert!(bloc.state().is_loaded());
        assert_eq!(bloc.state().queue().expect("a queue again"), &first);
        assert!(PrescriptionsEvent::Refresh.keeps_the_queue());
    });
}

#[gpui::test]
fn a_refresh_in_flight_keeps_the_queue_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc =
            PrescriptionsBloc::new(StoryPrescriptionRepository, "Mwenge", story::SELECTED);
        bloc.dispatch(PrescriptionsEvent::Load).await;
        let queue = bloc.state().queue().expect("a queue").clone();

        let in_flight = PrescriptionsState::Refreshing { queue };
        assert!(in_flight.is_busy());
        assert!(in_flight.queue().is_some());
        assert!(!in_flight.is_loaded());
    });
}

#[gpui::test]
fn selecting_another_prescription_moves_the_rail(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc =
            PrescriptionsBloc::new(StoryPrescriptionRepository, "Mwenge", story::SELECTED);
        bloc.dispatch(PrescriptionsEvent::Load).await;

        bloc.dispatch(PrescriptionsEvent::Select {
            reference: "RX-2215".to_string(),
        })
        .await;
        assert_eq!(bloc.selected(), "RX-2215");
        let queue = bloc.state().queue().expect("a queue");
        assert_eq!(&*queue.selected, "RX-2215");
        assert!(
            PrescriptionsEvent::Select {
                reference: String::new()
            }
            .keeps_the_queue()
        );
    });
}

#[gpui::test]
fn a_failed_read_leaves_an_error_not_an_empty_queue(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = PrescriptionsBloc::new(
            UnavailablePrescriptionRepository {
                detail: "the branch database is not reachable",
            },
            "Mwenge",
            story::SELECTED,
        );
        bloc.dispatch(PrescriptionsEvent::Load).await;
        let state = bloc.state().clone();
        assert!(
            state
                .failure()
                .is_some_and(|message| message.contains("not reachable"))
        );
        assert_eq!(state.queue(), None);
        assert!(!state.is_busy());
    });
}

#[gpui::test]
fn a_retry_after_a_failure_recovers_the_queue(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut broken = PrescriptionsBloc::new(
            UnavailablePrescriptionRepository { detail: "no route" },
            "Mwenge",
            story::SELECTED,
        );
        broken.dispatch(PrescriptionsEvent::Load).await;
        assert!(broken.state().failure().is_some());

        let mut recovered =
            PrescriptionsBloc::new(StoryPrescriptionRepository, "Mwenge", story::SELECTED);
        recovered.dispatch(PrescriptionsEvent::Retry).await;
        assert!(recovered.state().is_loaded());
    });
}

#[gpui::test]
fn the_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let broken =
            GetPrescriptions::new(UnavailablePrescriptionRepository { detail: "no route" });
        let error = broken.execute("Mwenge", story::SELECTED).await.unwrap_err();
        assert_eq!(
            error.message(),
            "The prescription queue could not be read. no route"
        );

        let reading = GetPrescriptions::new(StoryPrescriptionRepository);
        assert!(reading.execute("Mwenge", story::SELECTED).await.is_ok());
    });
}

#[test]
fn a_shaped_error_is_its_own_message() {
    let error = RepositoryError::Unexpected("status became a number".to_string());
    assert!(error.message().contains("shape this screen cannot read"));
}
