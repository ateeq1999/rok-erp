//! The medicines that sold most today.

use rok_ui::prelude::*;

use crate::features::dashboard::presentation::dashboard_screen::MedicineView;
use crate::features::dashboard::presentation::styles::DASHBOARD;

/// A right-aligned figure in the numbers font.
fn number_cell(text: impl Into<SharedString>) -> Div {
    div().sx(&DASHBOARD.number_column).child(text.into())
}

/// The table's header row.
fn head() -> Div {
    div()
        .sx(&DASHBOARD.table_head)
        .child(div().sx(&DASHBOARD.first_column).child("Medicine"))
        .child(div().sx(&DASHBOARD.number_head).child("Sold"))
        .child(div().sx(&DASHBOARD.number_head).child("Sales"))
}

/// The table, most money first.
pub(crate) fn card(medicines: &[MedicineView], colors: &ThemeColors) -> Div {
    div()
        .sx(sx![&DASHBOARD.card, style! { grow: {1.1} }])
        .child(div().sx(&DASHBOARD.card_title).child("Top medicines today"))
        .child(head())
        .children(medicines.iter().map(|medicine| {
            div()
                .sx(&DASHBOARD.table_row)
                .child(
                    div()
                        .sx(&DASHBOARD.first_column)
                        .child(medicine.name.clone()),
                )
                .child(number_cell(medicine.quantity.clone()).text_color(colors.muted_foreground))
                .child(number_cell(medicine.sales.clone()).font_weight(FontWeight::SEMIBOLD))
        }))
}
