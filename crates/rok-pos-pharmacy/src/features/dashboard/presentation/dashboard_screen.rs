//! What the dashboard draws, and for which state.
//!
//! The screen is one match over [`DashboardState`]. Each arm resolves the
//! figures the board needs through the domain's rules, formats them here, and
//! hands them to widgets that only draw. Nothing below this file derives a
//! figure.

use std::rc::Rc;

use gpui::ElementId;
use rok_pos_shell::{Tone, format_money, tone};
use rok_ui::prelude::*;

use super::styles::DASHBOARD;
use super::widgets::{
    branches_today, dashboard_tile, needs_you_now, payment_split, sales_by_hour, top_medicines,
};
use crate::features::dashboard::application::dashboard_event::DashboardEvent;
use crate::features::dashboard::application::dashboard_state::DashboardState;
use crate::features::dashboard::domain::calculations;
use crate::features::dashboard::domain::entities::Dashboard;
use crate::features::dashboard::domain::enums::{Destination, TaskSeverity};
use crate::features::dashboard::domain::measures::MeasureValue;

/// How the screen asks the `BLoC` to do something.
pub(crate) type Dispatch = Rc<dyn Fn(DashboardEvent, &mut Window, &mut App)>;

/// How tall the busiest hour's bar is, in pixels.
const TALLEST_BAR_PX: f32 = 100.;

/// One figure the tile widget draws.
pub(crate) struct TileView {
    /// What the tile counts.
    pub label: String,
    /// The figure.
    pub value: String,
    /// The line under it.
    pub note: String,
    /// How serious the figure looks.
    pub tone: Option<Tone>,
    /// Where the tile goes.
    pub destination: Destination,
}

/// One row of "Needs you now".
pub(crate) struct TaskView {
    /// The chip on the left.
    pub kind: String,
    /// How the chip is coloured.
    pub tone: Tone,
    /// What it is about.
    pub title: String,
    /// What to do about it.
    pub detail: String,
    /// The words on the right.
    pub action: String,
    /// Where the row goes.
    pub destination: Destination,
}

/// One bar of the sales chart.
pub(crate) struct HourView {
    /// The hour it starts.
    pub hour: String,
    /// How tall the bar is, in pixels.
    pub height: f32,
    /// Whether the hour is still going.
    pub in_progress: bool,
}

/// One row of the best sellers.
pub(crate) struct MedicineView {
    /// The medicine.
    pub name: String,
    /// How much of it.
    pub quantity: String,
    /// What it sold for.
    pub sales: String,
}

/// One line of the payment split.
pub(crate) struct PaymentView {
    /// The way it was paid.
    pub label: String,
    /// Its share of the day, as a whole percentage.
    pub percent: i64,
    /// Its share of the day as a fraction, for the split bar.
    pub fraction: f32,
    /// How much.
    pub amount: String,
}

/// One measure of the branch comparison.
pub(crate) struct BranchRowView {
    /// What is compared.
    pub label: String,
    /// This branch's figure.
    pub here: String,
    /// The other branch's figure.
    pub there: String,
}

/// Every figure the dashboard's widgets draw, already resolved.
pub(crate) struct Board {
    /// This branch's short name.
    pub branch: String,
    /// The other branch's short name.
    pub other_branch: String,
    /// The four tiles across the top.
    pub tiles: Vec<TileView>,
    /// The day's alerts.
    pub tasks: Vec<TaskView>,
    /// The sales chart's bars.
    pub hours: Vec<HourView>,
    /// The best sellers.
    pub top_medicines: Vec<MedicineView>,
    /// The payment split.
    pub payments: Vec<PaymentView>,
    /// The branch comparison.
    pub branches: Vec<BranchRowView>,
}

/// A figure as the screen writes it: money grouped, a count plain, a percent
/// signed.
pub(crate) fn measure_text(value: MeasureValue) -> String {
    match value {
        MeasureValue::Money(money) => format_money(money),
        MeasureValue::Count(count) => count.to_string(),
        MeasureValue::Percent(percent) => format!("{percent}%"),
    }
}

/// How a severity is coloured, which is the one place the domain's own words
/// become the shell's tones.
pub(crate) fn tone_for(severity: TaskSeverity) -> Tone {
    match severity {
        TaskSeverity::Danger => Tone::Danger,
        TaskSeverity::Warning => Tone::Warning,
        TaskSeverity::Info => Tone::Info,
        TaskSeverity::Success => Tone::Success,
    }
}

/// Where each destination opens, which no other layer knows.
pub(crate) fn route(destination: Destination) -> &'static str {
    match destination {
        Destination::Reports => "/reports",
        Destination::Prescriptions => "/prescriptions",
        Destination::PrescriptionCheck => "/prescriptions/RX-2214/check",
        Destination::Batches => "/batches",
        Destination::Claims => "/claims",
        Destination::Recall => "/recalls/RC-0047",
        Destination::Receive => "/receive/UZ-7781",
        Destination::Licences => "/licences",
        Destination::ControlledRegister => "/controlled-register",
    }
}

/// A box that opens `href` on a click, Enter or Space.
pub(crate) fn link_to(id: impl Into<ElementId>, href: &'static str) -> gpui::Stateful<Div> {
    div()
        .id(id)
        .tab_index(0)
        .on_click(move |_, _, cx| rok_ui::router::navigate(href, cx))
        .on_key_down(move |event, _, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                rok_ui::router::navigate(href, cx);
            }
        })
}

/// The whole board for a day's figures.
pub(crate) fn board(dashboard: &Dashboard) -> Board {
    let heights = calculations::bar_heights(dashboard, TALLEST_BAR_PX);
    let last = dashboard.hours.len().saturating_sub(1);
    Board {
        branch: dashboard.branch.to_string(),
        other_branch: dashboard.other_branch.name.to_string(),
        tiles: calculations::tiles(dashboard)
            .into_iter()
            .map(|tile| TileView {
                label: tile.label.to_string(),
                value: measure_text(tile.measure),
                note: match tile.note_figure {
                    Some(figure) => {
                        format!("{} \u{b7} {}", tile.note, measure_text(figure))
                    }
                    None => tile.note.to_string(),
                },
                tone: tile.severity.map(tone_for),
                destination: tile.destination,
            })
            .collect(),
        tasks: dashboard
            .tasks
            .iter()
            .map(|task| TaskView {
                kind: task.kind.label().to_string(),
                tone: tone_for(task.severity),
                title: task.title.to_string(),
                detail: task.detail.to_string(),
                action: task.action.to_string(),
                destination: task.destination,
            })
            .collect(),
        hours: dashboard
            .hours
            .iter()
            .zip(heights)
            .enumerate()
            .map(|(index, (hour, height))| HourView {
                hour: hour.hour.to_string(),
                height,
                in_progress: index == last,
            })
            .collect(),
        top_medicines: calculations::best_sellers(dashboard)
            .into_iter()
            .map(|medicine| MedicineView {
                name: medicine.name.to_string(),
                quantity: medicine.quantity.to_string(),
                sales: format_money(medicine.sales),
            })
            .collect(),
        payments: dashboard
            .payments
            .iter()
            .map(|payment| PaymentView {
                label: payment.label.to_string(),
                percent: calculations::payment_share(dashboard, payment.amount),
                fraction: share_fraction(payment.amount, calculations::sales_total(dashboard)),
                amount: format_money(payment.amount),
            })
            .collect(),
        branches: calculations::branch_measures(dashboard)
            .into_iter()
            .map(|measure| BranchRowView {
                label: measure.label.to_string(),
                here: measure_text(measure.here),
                there: measure_text(measure.there),
            })
            .collect(),
    }
}

/// `amount`'s share of the day as a fraction of the split bar; nothing of
/// nothing is no bar.
fn share_fraction(amount: rok_pos_domain::Money, total: rok_pos_domain::Money) -> f32 {
    let total = total.round_to_shillings();
    if total == 0 {
        0.
    } else {
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        {
            (amount.round_to_shillings() as f64 / total as f64) as f32
        }
    }
}

/// The board while the day's figures are on their way.
fn loading(cx: &Cx) -> Div {
    let colors = cx.theme().colors.clone();
    div()
        .sx(&DASHBOARD.root)
        .child(
            div()
                .sx(&DASHBOARD.notice)
                .child(
                    div()
                        .sx(&DASHBOARD.notice_title)
                        .child("Loading today's figures"),
                )
                .child(
                    div()
                        .sx(&DASHBOARD.notice_body)
                        .child("Sales, prescriptions, expiring stock and the day's alerts."),
                ),
        )
        .child(dashboard_tile::placeholder(colors))
}

/// The board when the figures could not be read.
fn failed(message: &str, dispatch: &Dispatch, cx: &Cx) -> Div {
    let _ = cx;
    let retry = dispatch.clone();
    let retry_button = Button::new("dashboard-retry")
        .label("Try again")
        .on_click(move |_, window, cx| retry(DashboardEvent::Retry, window, cx));
    div().sx(&DASHBOARD.root).child(
        div()
            .sx(&DASHBOARD.notice)
            .child(
                div()
                    .sx(&DASHBOARD.notice_title)
                    .child("Today's figures could not be read"),
            )
            .child(div().sx(&DASHBOARD.notice_body).child(message.to_string()))
            .child(retry_button),
    )
}

/// The board for a day that is on screen.
fn content(board: &Board, refreshing: bool, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    let mode = cx.theme().mode;
    let colors = cx.theme().colors.clone();
    let refresh = dispatch.clone();
    let refresh_button = Button::new("dashboard-refresh")
        .label(if refreshing { "Refreshing" } else { "Refresh" })
        .on_click(move |_, window, cx| refresh(DashboardEvent::Refresh, window, cx));
    div()
        .sx(&DASHBOARD.root)
        .child(
            div().sx(&DASHBOARD.tiles).children(
                board
                    .tiles
                    .iter()
                    .enumerate()
                    .map(|(index, tile)| dashboard_tile::tile(index, tile, mode)),
            ),
        )
        .child(
            div()
                .sx(&DASHBOARD.row)
                .child(needs_you_now::card(&board.tasks, mode, &colors))
                .child(sales_by_hour::card(&board.hours, mode, &colors)),
        )
        .child(
            div()
                .sx(&DASHBOARD.row)
                .child(top_medicines::card(&board.top_medicines, &colors))
                .child(payment_split::card(&board.payments, mode))
                .child(branches_today::card(
                    &board.branches,
                    &board.branch,
                    &board.other_branch,
                )),
        )
        .child(
            div().sx(&DASHBOARD.row).child(refresh_button).child(
                link_to("dashboard-reports", route(Destination::Reports))
                    .sx(&DASHBOARD.link)
                    .child("Open full reports"),
            ),
        )
}

/// The dashboard for the state it is in.
pub(crate) fn draw(state: &DashboardState, dispatch: &Dispatch, cx: &mut Cx) -> Div {
    match state {
        DashboardState::Initial | DashboardState::Loading => loading(cx),
        DashboardState::Refreshing { dashboard } => content(&board(dashboard), true, dispatch, cx),
        DashboardState::Loaded { dashboard } => content(&board(dashboard), false, dispatch, cx),
        DashboardState::Error { message } => failed(message, dispatch, cx),
    }
}

/// The tone colours, for the widgets that need a palette.
pub(crate) fn palette(tone: Tone, mode: ThemeMode) -> tone::ToneColors {
    tone::colors(tone, mode)
}
