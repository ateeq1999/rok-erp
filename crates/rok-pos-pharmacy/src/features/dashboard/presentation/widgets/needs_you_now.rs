//! What needs the pharmacist now.

use gpui::Stateful;
use rok_pos_shell::{Tone, tone};
use rok_ui::prelude::*;

use crate::features::dashboard::presentation::dashboard_screen::{TaskView, link_to, route};
use crate::features::dashboard::presentation::styles::DASHBOARD;

/// One row of the card.
fn row(index: usize, view: &TaskView, mode: ThemeMode, colors: &ThemeColors) -> Stateful<Div> {
    let palette = tone::colors(view.tone, mode);
    let action_color = if view.tone == Tone::Success {
        colors.muted_foreground
    } else {
        colors.primary
    };
    link_to(("dashboard-task", index), route(view.destination))
        .sx(&DASHBOARD.task)
        .child(
            div()
                .sx(sx![
                    &DASHBOARD.task_chip,
                    style! { background: {palette.background}, color: {palette.foreground} }
                ])
                .child(view.kind.clone()),
        )
        .child(
            div()
                .sx(&DASHBOARD.task_text)
                .child(div().sx(&DASHBOARD.task_title).child(view.title.clone()))
                .child(div().sx(&DASHBOARD.meta).child(view.detail.clone())),
        )
        .child(
            div()
                .sx(&DASHBOARD.task_action)
                .text_color(action_color)
                .child(view.action.clone()),
        )
}

/// The card: how much is open, then every row, finished work last.
pub(crate) fn card(tasks: &[TaskView], mode: ThemeMode, colors: &ThemeColors) -> Div {
    let open = tasks
        .iter()
        .filter(|task| task.tone != Tone::Success)
        .count();
    let done = tasks.len() - open;
    div()
        .sx(sx![&DASHBOARD.card, style! { grow: {1.25} }])
        .child(
            div()
                .sx(&DASHBOARD.card_head)
                .child(div().sx(&DASHBOARD.card_title).child("Needs you now"))
                .child(
                    div()
                        .sx(&DASHBOARD.meta)
                        .child(format!("{open} open \u{b7} {done} done today")),
                ),
        )
        .children(
            tasks
                .iter()
                .enumerate()
                .map(|(index, view)| row(index, view, mode, colors)),
        )
}
