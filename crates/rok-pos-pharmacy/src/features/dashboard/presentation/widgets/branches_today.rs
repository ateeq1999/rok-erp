//! This branch against the other one, on five measures.

use rok_ui::prelude::*;

use crate::features::dashboard::domain::enums::Destination;
use crate::features::dashboard::presentation::dashboard_screen::{BranchRowView, link_to, route};
use crate::features::dashboard::presentation::styles::DASHBOARD;

/// A right-aligned figure in the numbers font.
fn number_cell(text: impl Into<SharedString>) -> Div {
    div().sx(&DASHBOARD.number_column).child(text.into())
}

/// The comparison table and the link to the reports behind it.
pub(crate) fn card(rows: &[BranchRowView], branch: &str, other_branch: &str) -> Div {
    let head = |first: &str, second: &str, third: &str| {
        div()
            .sx(&DASHBOARD.table_head)
            .child(div().sx(&DASHBOARD.first_column).child(first.to_string()))
            .child(div().sx(&DASHBOARD.number_head).child(second.to_string()))
            .child(div().sx(&DASHBOARD.number_head).child(third.to_string()))
    };
    div()
        .sx(sx![&DASHBOARD.card, style! { grow: {1.} }])
        .child(div().sx(&DASHBOARD.card_title).child("Branches today"))
        .child(head("Measure", branch, other_branch))
        .children(rows.iter().map(|row| {
            div()
                .sx(&DASHBOARD.table_row)
                .child(div().sx(&DASHBOARD.first_column).child(row.label.clone()))
                .child(number_cell(row.here.clone()).font_weight(FontWeight::SEMIBOLD))
                .child(number_cell(row.there.clone()))
        }))
        .child(
            link_to("dashboard-reports", route(Destination::Reports))
                .sx(&DASHBOARD.link)
                .child("Open full reports"),
        )
}
