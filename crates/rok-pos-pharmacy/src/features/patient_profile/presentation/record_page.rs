//! The patient record page, and one of the ways into this feature.
//!
//! The page holds the `BLoC`, dispatches its first load once, and draws whatever
//! state the `BLoC` is in. It is the composition point, so it is the one place
//! that names a source; the screen and the widgets never see one.

use rok_ui::prelude::*;

use super::record_screen;
use crate::features::patient_profile::application::record_bloc::RecordBloc;
use crate::features::patient_profile::application::record_event::RecordEvent;
use crate::features::patient_profile::data::repository_impl::StoryPatientRepository;

/// The patient the record opens on until the route carries a real one.
const PATIENT_ID: &str = "P-1042";

/// The patient's record.
#[component]
pub fn PatientProfilePage(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let state = cx.use_state(|| RecordBloc::new(StoryPatientRepository, PATIENT_ID));
    let loaded = cx.use_state(|| false);

    if !loaded.get(cx.app) {
        loaded.set(true, cx.app);
        let mut working = state.get(cx.app);
        let handle = state.clone();
        cx.spawn(async move |cx| {
            working.dispatch(RecordEvent::Load).await;
            let _ = cx.update(|cx| handle.set(working, cx));
        })
        .detach();
    }

    let bloc = state.get(cx.app);
    div().sx(sx).child(record_screen::draw(bloc.state(), cx))
}
