//! The reports page, and the way into this feature.
//!
//! The page holds the `BLoC`, dispatches its first load once, and draws
//! whatever state the `BLoC` is in. It is the composition point, so it is the
//! one place that names a source; the screen never sees one.

use std::rc::Rc;

use rok_ui::prelude::*;

use super::report_screen::{self, Dispatch};
use crate::features::reports::application::report_bloc::ReportBloc;
use crate::features::reports::application::report_event::ReportEvent;
use crate::features::reports::data::repository_impl::StoryReportsRepository;

/// Send `event` to the bloc, and write back what it made of it.
///
/// The bloc is copied out of the state, read on a copy of itself, and written
/// back in one update, because the reports cannot be borrowed across the read.
/// The task is detached, so dropping the page cancels the read.
fn dispatch(state: &State<ReportBloc<StoryReportsRepository>>, event: ReportEvent, cx: &mut App) {
    let mut working = state.get(cx);
    let handle = state.clone();
    cx.spawn(async move |cx| {
        working.dispatch(event).await;
        let _ = cx.update(|cx| handle.set(working, cx));
    })
    .detach();
}

/// The pharmacy's reports: what sold, what it left, what it lost, and what is
/// still owed.
#[component]
pub fn ReportsPage(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let state = cx.use_state(|| ReportBloc::new(StoryReportsRepository));
    let loaded = cx.use_state(|| false);

    if !loaded.get(cx.app) {
        loaded.set(true, cx.app);
        dispatch(&state, ReportEvent::Load, cx.app);
    }

    let bloc = state.get(cx.app);
    let send: Dispatch = Rc::new({
        let state = state.clone();
        move |event, _window, cx| dispatch(&state, event, cx)
    });

    div()
        .sx(sx)
        .child(report_screen::draw(bloc.state(), &send, cx))
}
