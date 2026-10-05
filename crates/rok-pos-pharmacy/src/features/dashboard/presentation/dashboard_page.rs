//! The dashboard page, and the only way into this feature from outside it.
//!
//! The page holds the `BLoC`, dispatches its first load once, and draws whatever
//! state the `BLoC` is in. It is the composition point, so it is the one place
//! that names a source; the screen and the widgets never see one.

use std::rc::Rc;

use rok_ui::prelude::*;

use super::dashboard_screen::{self, Dispatch};
use crate::features::dashboard::application::dashboard_bloc::DashboardBloc;
use crate::features::dashboard::application::dashboard_event::DashboardEvent;
use crate::features::dashboard::data::repository_impl::StoryDashboardRepository;
use crate::features::dashboard::data::story;

/// Send `event` to the bloc, and write back what it made of it.
///
/// The bloc is copied out of the state, read on a copy of itself, and written
/// back in one update, because the entity cannot be borrowed across the read.
/// The task is detached, so dropping the page cancels the read.
fn dispatch(
    state: &State<DashboardBloc<StoryDashboardRepository>>,
    event: DashboardEvent,
    cx: &mut App,
) {
    let mut working = state.get(cx);
    let handle = state.clone();
    cx.spawn(async move |cx| {
        working.dispatch(event).await;
        let _ = cx.update(|cx| handle.set(working, cx));
    })
    .detach();
}

/// The dashboard, as `/` frames it.
#[component]
pub fn DashboardPage(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let state = cx.use_state(|| DashboardBloc::new(StoryDashboardRepository, story::MWENGE));
    let loaded = cx.use_state(|| false);

    if !loaded.get(cx.app) {
        loaded.set(true, cx.app);
        dispatch(&state, DashboardEvent::Load, cx.app);
    }

    let bloc = state.get(cx.app);
    let send: Dispatch = Rc::new({
        let state = state.clone();
        move |event, _, cx| dispatch(&state, event, cx)
    });

    div()
        .sx(sx)
        .child(dashboard_screen::draw(bloc.state(), &send, cx))
}
