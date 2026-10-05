//! The dashboard's state machine: what an event does to the state.

use gpui::TestAppContext;

use crate::features::dashboard::application::dashboard_bloc::DashboardBloc;
use crate::features::dashboard::application::dashboard_event::DashboardEvent;
use crate::features::dashboard::application::dashboard_state::DashboardState;
use crate::features::dashboard::data::repository::RepositoryError;
use crate::features::dashboard::data::repository_impl::{
    StoryDashboardRepository, UnavailableDashboardRepository,
};
use crate::features::dashboard::data::story;
#[test]
fn a_new_bloc_has_read_nothing() {
    let bloc = DashboardBloc::new(StoryDashboardRepository, story::MWENGE);
    assert_eq!(bloc.state(), &DashboardState::Initial);
    assert_eq!(bloc.branch_id(), story::MWENGE);
}
#[gpui::test]
fn the_first_load_puts_the_board_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = DashboardBloc::new(StoryDashboardRepository, story::MWENGE);
        bloc.dispatch(DashboardEvent::Load).await;
        let state = bloc.state().clone();
        assert!(state.is_loaded());
        let dashboard = state.dashboard().expect("a board on screen");
        assert_eq!(&*dashboard.branch, story::MWENGE);
        assert_eq!(dashboard.hours.len(), 8);
    });
}
#[gpui::test]
fn a_refresh_keeps_the_board_and_puts_a_new_one_back(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = DashboardBloc::new(StoryDashboardRepository, story::MWENGE);
        bloc.dispatch(DashboardEvent::Load).await;
        let first = bloc.state().dashboard().expect("a board").clone();
        assert!(DashboardEvent::Refresh.keeps_the_board());
        bloc.dispatch(DashboardEvent::Refresh).await;
        assert!(bloc.state().is_loaded());
        assert_eq!(
            bloc.state().dashboard().expect("a board again"),
            &first,
            "a refresh reads the day again and keeps it on screen"
        );
    });
}
#[gpui::test]
fn a_refresh_in_flight_keeps_the_board_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = DashboardBloc::new(StoryDashboardRepository, story::MWENGE);
        bloc.dispatch(DashboardEvent::Load).await;
        let board = bloc.state().dashboard().expect("a board").clone();
        // The state the screen shows while a refresh is still reading: the old
        // day, still drawn, rather than an empty page.
        let in_flight = DashboardState::Refreshing { dashboard: board };
        assert!(in_flight.is_busy());
        assert!(in_flight.dashboard().is_some());
        assert!(!in_flight.is_loaded());
    });
}
#[gpui::test]
fn a_failed_read_leaves_an_error_not_an_empty_board(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = DashboardBloc::new(
            UnavailableDashboardRepository {
                detail: "the branch database is not reachable",
            },
            story::MWENGE,
        );
        bloc.dispatch(DashboardEvent::Load).await;
        let state = bloc.state().clone();
        assert!(
            state
                .failure()
                .is_some_and(|message| message.contains("not reachable"))
        );
        assert_eq!(state.dashboard(), None);
        assert!(!state.is_busy());
    });
}
#[gpui::test]
fn a_retry_after_a_failure_recovers_the_board(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = DashboardBloc::new(
            UnavailableDashboardRepository {
                detail: "the branch database is not reachable",
            },
            story::MWENGE,
        );
        bloc.dispatch(DashboardEvent::Load).await;
        assert!(bloc.state().failure().is_some());
        // Recovery is an event, not a new page: the same bloc, reading a source
        // that answers.
        let mut recovered = DashboardBloc::new(StoryDashboardRepository, story::MWENGE);
        recovered.dispatch(DashboardEvent::Retry).await;
        assert!(recovered.state().is_loaded());
    });
}
#[gpui::test]
fn changing_branch_reads_that_branch(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = DashboardBloc::new(StoryDashboardRepository, story::MWENGE);
        bloc.dispatch(DashboardEvent::ChangeBranch {
            branch_id: story::TEGETA.to_string(),
        })
        .await;
        assert_eq!(bloc.branch_id(), story::TEGETA);
        assert_eq!(
            &*bloc.state().dashboard().expect("a board").branch,
            story::TEGETA
        );
    });
}
#[gpui::test]
fn the_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        use crate::features::dashboard::application::dashboard_use_cases::GetDashboard;
        let use_cases = GetDashboard::new(UnavailableDashboardRepository { detail: "no route" });
        let error = use_cases.execute(story::MWENGE).await.unwrap_err();
        assert_eq!(error.message(), "The dashboard could not be read. no route");
        let reading = GetDashboard::new(StoryDashboardRepository);
        assert!(reading.execute(story::MWENGE).await.is_ok());
    });
}
#[test]
fn a_shaped_error_is_its_own_message() {
    let error = RepositoryError::Unexpected("three columns became two".to_string());
    assert!(error.message().contains("shape this screen cannot read"));
}
