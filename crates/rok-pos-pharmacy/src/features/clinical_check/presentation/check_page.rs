//! The clinical check page, and one of the ways into this feature.

use std::rc::Rc;

use rok_ui::prelude::*;

use super::check_screen::{self, Dispatch};
use crate::features::clinical_check::application::check_bloc::CheckBloc;
use crate::features::clinical_check::application::check_event::CheckEvent;
use crate::features::clinical_check::data::repository_impl::StoryClinicalCheckRepository;
use crate::features::clinical_check::data::story;

/// Send `event` to the bloc, and write back what it made of it.
///
/// The bloc is copied out of the state, read on a copy of itself, and written
/// back in one update, because the entity cannot be borrowed across the read.
/// The task is detached, so dropping the page cancels the read.
fn dispatch(
    state: &State<CheckBloc<StoryClinicalCheckRepository>>,
    event: CheckEvent,
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

/// The branch the check is read for until the session replaces it.
const BRANCH: &str = "Mwenge";

/// The prescription the board opens on.
const PRESCRIPTION: &str = story::RX_2214;

/// The pharmacist's check of one prescription.
#[component]
pub fn ClinicalCheckPage(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let state = cx.use_state(|| CheckBloc::new(StoryClinicalCheckRepository, BRANCH, PRESCRIPTION));
    let loaded = cx.use_state(|| false);

    if !loaded.get(cx.app) {
        loaded.set(true, cx.app);
        dispatch(&state, CheckEvent::Load, cx.app);
    }

    let bloc = state.get(cx.app);
    let send: Dispatch = Rc::new({
        let state = state.clone();
        move |event, _window, cx| dispatch(&state, event, cx)
    });

    div()
        .sx(sx)
        .child(check_screen::draw(bloc.state(), &send, cx))
}
