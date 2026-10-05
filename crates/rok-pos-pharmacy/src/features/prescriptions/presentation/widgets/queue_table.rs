//! The queue table: one row per prescription, waiting first.

use gpui::Stateful;
use gpui::prelude::*;
use rok_pos_shell::Tone;
use rok_ui::prelude::*;

use crate::features::prescriptions::application::prescriptions_event::PrescriptionsEvent;
use crate::features::prescriptions::presentation::prescriptions_screen::Dispatch;
use crate::features::prescriptions::presentation::styles::QUEUE;
use crate::features::shared::board;

/// One row of the table, as the screen resolves it.
pub(crate) struct RowView {
    /// Its reference.
    pub reference: String,
    /// When it reached the pharmacy.
    pub received: String,
    /// Who it is for.
    pub patient: String,
    /// What was prescribed.
    pub medicines: String,
    /// Where it came from, as the board writes it.
    pub source: String,
    /// Where it has got to, as the board writes it.
    pub status: String,
    /// How the status chip is coloured.
    pub tone: Tone,
    /// What the pharmacist must not miss.
    pub flags: Vec<String>,
    /// Whether a flag is a stop rather than a warning.
    pub flag_is_alert: bool,
}

/// One row: the flags as chips, and a click that opens the prescription.
fn row(index: usize, view: &RowView, selected: bool, dispatch: &Dispatch) -> Stateful<Div> {
    let flags: Vec<AnyElement> = view
        .flags
        .iter()
        .map(|flag| {
            board::chip(
                flag.clone(),
                if view.flag_is_alert {
                    Tone::Danger
                } else {
                    Tone::Warning
                },
            )
            .into_any_element()
        })
        .collect();
    let select = dispatch.clone();
    let reference = view.reference.clone();
    let cell = div()
        .id(("prescription-row", index))
        .debug_selector(|| format!("prescription-row-{index}"))
        .tab_index(0)
        .on_click(move |_, window, cx| {
            select(
                PrescriptionsEvent::Select {
                    reference: reference.clone(),
                },
                window,
                cx,
            );
        })
        .on_key_down({
            let select = dispatch.clone();
            let reference = view.reference.clone();
            move |event, window, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    cx.stop_propagation();
                    select(
                        PrescriptionsEvent::Select {
                            reference: reference.clone(),
                        },
                        window,
                        cx,
                    );
                }
            }
        })
        .sx(board::line_style())
        .child(board::cell_fixed_stack(
            format!("{}\u{a0}\u{a0}{}", view.reference, view.received),
            view.patient.clone(),
        ))
        .child(board::cell_stack(
            view.medicines.clone(),
            view.source.clone(),
        ))
        .child(board::chip(view.status.clone(), view.tone))
        .child(board::chip_line(flags));
    if selected {
        cell.sx(&QUEUE.selected_row)
    } else {
        cell
    }
}

/// The table and its header.
pub(crate) fn card(
    range: &str,
    waiting: u32,
    oldest: &str,
    rows: &[RowView],
    selected: &str,
    dispatch: &Dispatch,
) -> Div {
    board::card(2.)
        .child(board::card_head(
            range.to_string(),
            format!("{waiting} waiting \u{b7} {oldest}"),
        ))
        .child(board::head(vec![
            board::cell_fixed("Prescription"),
            board::cell("Patient and medicines"),
            board::cell_fixed("Status"),
            board::cell_fixed("Flags"),
        ]))
        .children(
            rows.iter()
                .enumerate()
                .map(|(index, view)| row(index, view, view.reference == selected, dispatch)),
        )
}
