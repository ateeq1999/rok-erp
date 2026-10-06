//! The reports' state machine: what an event does to the state.

use gpui::TestAppContext;

use crate::features::reports::application::errors::ReportError;
use crate::features::reports::application::report_bloc::ReportBloc;
use crate::features::reports::application::report_event::ReportEvent;
use crate::features::reports::application::report_state::ReportState;
use crate::features::reports::application::report_use_cases::ReadReports;
use crate::features::reports::data::repository::RepositoryError;
use crate::features::reports::data::repository_impl::{
    StoryReportsRepository, UnavailableReportsRepository,
};

#[test]
fn a_new_bloc_has_read_nothing() {
    let bloc = ReportBloc::new(StoryReportsRepository);
    assert_eq!(bloc.state(), &ReportState::Initial);
    assert!(!bloc.state().is_busy());
}

#[gpui::test]
fn the_first_load_puts_the_reports_on_screen(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = ReportBloc::new(StoryReportsRepository);
        bloc.dispatch(ReportEvent::Load).await;
        let state = bloc.state().clone();
        assert!(state.is_loaded());
        assert!(!state.is_busy());
        let reports = state.reports().expect("reports on screen");
        assert_eq!(reports.reports.len(), 5);
        assert_eq!(
            reports.sales().expect("the board leads with sales").total,
            1_842_000
        );
    });
}

#[gpui::test]
fn a_retry_after_a_failure_recovers_the_reports(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut broken = ReportBloc::new(UnavailableReportsRepository {
            detail: "the reports store is not reachable",
        });
        broken.dispatch(ReportEvent::Load).await;
        assert!(broken.state().failure().is_some());

        let mut recovered = ReportBloc::new(StoryReportsRepository);
        recovered.dispatch(ReportEvent::Retry).await;
        assert!(recovered.state().reports().is_some());
    });
}

#[gpui::test]
fn the_use_case_maps_a_source_failure_into_the_applications_own(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let broken = ReadReports::new(UnavailableReportsRepository { detail: "no route" });
        let error = broken.execute().await.unwrap_err();
        assert_eq!(error.message(), "The reports could not be read. no route");

        let reading = ReadReports::new(StoryReportsRepository);
        assert!(reading.execute().await.is_ok());
    });
}

#[test]
fn a_shaped_error_is_its_own_message() {
    let error = RepositoryError::Unexpected("a total became a word".to_string());
    let mapped = ReportError::from(error);
    assert!(mapped.message().contains("shape this screen cannot read"));
}

#[gpui::test]
fn a_failed_read_leaves_an_error_not_empty_reports(cx: &mut TestAppContext) {
    cx.executor().block_test(async {
        let mut bloc = ReportBloc::new(UnavailableReportsRepository {
            detail: "the reports are not reachable",
        });
        bloc.dispatch(ReportEvent::Load).await;
        let state = bloc.state().clone();
        assert!(
            state
                .failure()
                .is_some_and(|message| message.contains("not reachable"))
        );
        assert_eq!(state.reports(), None);
        assert!(!state.is_busy());
    });
}
