//! The four tiles across the top of the dashboard.

use gpui::Stateful;
use rok_ui::prelude::*;

use crate::features::dashboard::presentation::dashboard_screen::{
    TileView, link_to, palette, route,
};
use crate::features::dashboard::presentation::styles::DASHBOARD;

/// One tile: what it counts, the figure, and the line under it.
pub(crate) fn tile(index: usize, view: &TileView, mode: ThemeMode) -> Stateful<Div> {
    let color = view.tone.map(|tone| palette(tone, mode).foreground);
    link_to(("dashboard-tile", index), route(view.destination))
        .sx(&DASHBOARD.tile)
        .child(div().sx(&DASHBOARD.tile_label).child(view.label.clone()))
        .child(
            div()
                .sx(&DASHBOARD.tile_value)
                .when_some(color, gpui::Styled::text_color)
                .child(view.value.clone()),
        )
        .child(div().sx(&DASHBOARD.small).child(view.note.clone()))
}

/// The empty row the loading board draws where the tiles will be.
pub(crate) fn placeholder(_colors: ThemeColors) -> impl IntoElement {
    div().sx(&DASHBOARD.tiles).children(
        [
            "Sales today",
            "Prescriptions",
            "Expiring in 30 days",
            "Claims queried",
        ]
        .into_iter()
        .map(|label: &str| {
            div()
                .sx(&DASHBOARD.tile)
                .child(div().sx(&DASHBOARD.tile_label).child(label))
                .child(div().sx(&DASHBOARD.tile_value).child("\u{2014}"))
        }),
    )
}
