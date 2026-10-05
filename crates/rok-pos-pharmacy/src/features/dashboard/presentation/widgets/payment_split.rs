//! How patients paid, as a split bar and a legend.

use rok_pos_shell::theme::chart_series;
use rok_ui::prelude::*;

use crate::features::dashboard::presentation::dashboard_screen::PaymentView;
use crate::features::dashboard::presentation::styles::DASHBOARD;

/// The split bar and the legend under it.
pub(crate) fn card(payments: &[PaymentView], mode: ThemeMode) -> Div {
    let series = chart_series(mode);
    let segments = payments
        .iter()
        .zip(series)
        .map(|(payment, color)| div().h_full().w(relative(payment.fraction)).bg(color));
    div()
        .sx(sx![&DASHBOARD.card, style! { grow: {1.} }])
        .child(div().sx(&DASHBOARD.card_title).child("How patients paid"))
        .child(div().sx(&DASHBOARD.split).children(segments))
        .children(payments.iter().zip(series).map(|(payment, color)| {
            div()
                .sx(&DASHBOARD.legend)
                .child(div().sx(&DASHBOARD.dot).bg(color))
                .child(div().flex_grow().child(payment.label.clone()))
                .child(
                    div()
                        .sx(&DASHBOARD.meta)
                        .child(format!("{}%", payment.percent)),
                )
                .child(
                    div()
                        .sx(&DASHBOARD.legend_amount)
                        .child(payment.amount.clone()),
                )
        }))
        .child(
            div()
                .sx(&DASHBOARD.small)
                .child("Insurance part is billed in the October claim batch, not cash in hand."),
        )
}
