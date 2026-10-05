//! Sales by hour, with the hour in progress drawn lighter.

use gpui::prelude::*;
use rok_pos_shell::theme::chart_in_progress;
use rok_ui::prelude::*;

use crate::features::dashboard::presentation::dashboard_screen::HourView;
use crate::features::dashboard::presentation::styles::DASHBOARD;

/// The chart and its hour labels.
pub(crate) fn card(hours: &[HourView], mode: ThemeMode, colors: &ThemeColors) -> Div {
    let bars = hours.iter().map(|hour| {
        let fill = if hour.in_progress {
            chart_in_progress(mode)
        } else {
            colors.primary
        };
        div()
            .sx(&DASHBOARD.bar_column)
            .child(div().sx(&DASHBOARD.bar_figure).child(hour.hour.clone()))
            .child(div().w_full().h(px(hour.height)).bg(fill))
    });
    div()
        .sx(sx![&DASHBOARD.card, style! { grow: {1.} }])
        .child(
            div()
                .sx(&DASHBOARD.card_head)
                .child(div().sx(&DASHBOARD.card_title).child("Sales by hour"))
                .child(div().sx(&DASHBOARD.meta).child("TZS \u{b7} today so far")),
        )
        .child(div().sx(&DASHBOARD.bars).children(bars))
        .child(
            div().sx(&DASHBOARD.hour_labels).children(
                hours
                    .iter()
                    .map(|hour| div().sx(&DASHBOARD.hour_label).child(hour.hour.clone())),
            ),
        )
        .child(
            div()
                .sx(&DASHBOARD.small)
                .child("The last bar is the hour in progress (lighter)."),
        )
}
