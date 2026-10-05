//! The prescription queue page, and one of the ways into this feature.

use std::rc::Rc;

use rok_ui::prelude::*;

use super::prescriptions_screen::{self, Dispatch};
use crate::features::prescriptions::application::prescriptions_bloc::PrescriptionsBloc;
use crate::features::prescriptions::application::prescriptions_event::PrescriptionsEvent;
use crate::features::prescriptions::data::repository_impl::StoryPrescriptionRepository;
use crate::features::prescriptions::data::story;

/// Send `event` to the bloc, and write back what it made of it.
///
/// The bloc is copied out of the state, read on a copy of itself, and written
/// back in one update, because the entity cannot be borrowed across the read.
/// The task is detached, so dropping the page cancels the read.
fn dispatch(
    state: &State<PrescriptionsBloc<StoryPrescriptionRepository>>,
    event: PrescriptionsEvent,
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

/// The branch the queue is read for until the session replaces it.
const BRANCH: &str = "Mwenge";

/// The prescription the board opens on.
const SELECTED: &str = story::SELECTED;

/// Today's prescriptions.
#[component]
pub fn PrescriptionsPage(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let state =
        cx.use_state(|| PrescriptionsBloc::new(StoryPrescriptionRepository, BRANCH, SELECTED));
    let loaded = cx.use_state(|| false);

    if !loaded.get(cx.app) {
        loaded.set(true, cx.app);
        dispatch(&state, PrescriptionsEvent::Load, cx.app);
    }

    let bloc = state.get(cx.app);
    let send: Dispatch = Rc::new({
        let state = state.clone();
        move |event, _window, cx| dispatch(&state, event, cx)
    });
    div()
        .sx(sx)
        .child(prescriptions_screen::draw(bloc.state(), &send, cx))
}
