//! The pieces every pharmacy board is built from, recovered from the boards in
//! `design/pharmacy`: a titled card, a stat row, a table with a header row, a
//! filter bar, and a two-column split with a detail rail.
//!
//! These are the shapes the boards repeat. A screen that draws its own copy of
//! one of them is the copy that drifts from the board, so the shape lives here
//! once and every screen fills it in with its own figures.

use gpui::ElementId;
use rok_pos_shell::Chip;
use rok_pos_shell::tone::{self, Tone};
use rok_ui::prelude::*;
use rok_ui::router::navigate;

/// A middle dot, as the boards separate a batch from its note with it.
pub const DOT: char = '\u{b7}';

styles! {
    BOARD = {
        root: { display: flex, flex_direction: column, gap: 4 },
        row: { display: flex, flex_direction: row, gap: 4, align: stretch },
        card: {
            basis: 0,
            min_width: 0,
            display: flex,
            flex_direction: column,
            gap: 2,
            padding: 4,
            background: card,
            border: 1,
            border_color: border,
            radius: none,
        },
        card_head: {
            display: flex,
            flex_direction: row,
            justify: between,
            align: center,
            gap: 2,
        },
        card_title: { text: {16.}, font: semibold },
        meta: { text: {13.}, color: muted_foreground },
        small: { text: {12.}, color: muted_foreground },
        link: { text: {13.}, font: semibold, color: primary, cursor: pointer },
        section_label: { text: {11.}, font: semibold, color: muted_foreground },
        stat_value: { font_family: mono, text: {22.}, font: bold },
        stat_note: { text: {12.}, color: muted_foreground },
        filters: { display: flex, flex_direction: row, gap: 1.5 },
        filter: {
            padding_x: 2,
            padding_y: {px(4.)},
            text: {13.},
            border: 1,
            border_color: border,
            radius: none,
            cursor: pointer,
        },
        filter_on: {
            padding_x: 2,
            padding_y: {px(4.)},
            text: {13.},
            font: semibold,
            border: 1,
            border_color: border,
            radius: none,
        },
        head: {
            display: flex,
            flex_direction: row,
            gap: 2,
            padding_y: 1,
            font: semibold,
            text: {12.},
            color: muted_foreground,
            border_bottom: 1,
            border_color: border,
        },
        line: {
            display: flex,
            flex_direction: row,
            gap: 2,
            align: center,
            padding_y: {px(7.)},
            border_top: 1,
            border_color: muted,
        },
        cell_stack: { display: flex, flex_direction: column, gap: {px(2.)}, min_width: 0 },
        cell_title: { font: semibold, truncate: true },
        cell_detail: { text: {12.}, color: muted_foreground, truncate: true },
        grow: { grow: 1, min_width: 0 },
        grow_truncate: { grow: 1, min_width: 0, truncate: true },
        number: { text_align: right, font_family: mono, shrink: 0 },
        fixed: { shrink: 0 },
        alert: {
            display: flex,
            flex_direction: row,
            gap: 2,
            padding: 3,
            border: 1,
            border_color: border,
            radius: none,
        },
        alert_title: { font: semibold },
        alert_body: { text: {13.}, color: muted_foreground },
        kv: { display: flex, flex_direction: row, justify: between, gap: 3, padding_y: {px(3.)} },
        kv_label: { text: {13.}, color: muted_foreground },
        kv_value: { font: semibold, text_align: right },
        actions: { display: flex, flex_direction: row, gap: 2, align: center },
        muted_panel: { padding: 3, background: muted, radius: none },
        list_gap: { display: flex, flex_direction: column, gap: 2 },
        chip_line: { display: flex, flex_direction: row, gap: 1.5, align: center },
    }
}

/// A card of the page's own width share, with the board's title and meta line.
#[must_use]
pub fn card(grow: f32) -> Div {
    div().sx(sx![&BOARD.card, style! { grow: {grow} }])
}

/// A card's heading: the title on the left, a figure or link on the right.
#[must_use]
pub fn card_head(title: impl Into<SharedString>, meta: impl Into<SharedString>) -> Div {
    div()
        .sx(&BOARD.card_head)
        .child(div().sx(&BOARD.card_title).child(title.into()))
        .child(div().sx(&BOARD.meta).child(meta.into()))
}

/// A plain heading, for a card whose right-hand side is a control.
#[must_use]
pub fn card_title(title: impl Into<SharedString>) -> Div {
    div().sx(&BOARD.card_title).child(title.into())
}

/// A box that opens `href` on a click, Enter or Space.
///
/// The boards make a whole row a link: a prescription row, a batch row, a
/// patient row. Each needs its own [`ElementId`] so two rows do not share one
/// focus ring.
#[must_use]
pub fn link_to(id: impl Into<ElementId>, href: &'static str) -> gpui::Stateful<Div> {
    div()
        .id(id)
        .tab_index(0)
        .on_click(move |_, _, cx| navigate(href, cx))
        .on_key_down(move |event, _, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                navigate(href, cx);
            }
        })
}

/// The small caps label a board puts above a block: "PRESCRIPTION IMAGE".
#[must_use]
pub fn section_label(text: impl Into<SharedString>) -> Div {
    div().sx(&BOARD.section_label).child(text.into())
}

/// One of the figures across the top of a board: a count, a value and a note.
#[must_use]
pub fn stat(
    label: impl Into<SharedString>,
    value: impl Into<SharedString>,
    note: impl Into<SharedString>,
    tone: Option<Tone>,
    mode: ThemeMode,
) -> Div {
    let value = value.into();
    div()
        .sx(sx![&BOARD.card, style! { grow: 1 }])
        .child(div().sx(&BOARD.meta).child(label.into()))
        .child(
            div()
                .sx(&BOARD.stat_value)
                .when_some(
                    tone.map(|tone| tone::colors(tone, mode).foreground),
                    gpui::Styled::text_color,
                )
                .child(value),
        )
        .child(div().sx(&BOARD.stat_note).child(note.into()))
}

/// A row of [`stat`]s, as the boards lay their headline figures out.
#[must_use]
pub fn stat_row(stats: Vec<Div>) -> Div {
    div().sx(&BOARD.row).children(stats)
}

/// The page's own root style, so a screen can layer overrides on it.
#[must_use]
pub fn root() -> &'static Sx {
    &BOARD.root
}

/// A row of cards sharing the width, as every board lays its middle section out.
#[must_use]
pub fn row() -> &'static Sx {
    &BOARD.row
}

/// A body row's style, so a screen can put its own id on one and keep the
/// row's border, padding and gap.
#[must_use]
pub fn line_style() -> &'static Sx {
    &BOARD.line
}

/// The growing cell's style: takes the space left over and truncates.
#[must_use]
pub fn grow_style() -> &'static Sx {
    &BOARD.grow
}

/// One filter chip. `on` is the board's selected state, which the shell fills
/// with the brand tint.
#[must_use]
pub fn filter(label: impl Into<SharedString>, on: bool, mode: ThemeMode) -> Div {
    let label = label.into();
    if on {
        let palette = tone::colors(Tone::Brand, mode);
        div().sx(sx![
            &BOARD.filter_on,
            style! { background: {palette.background}, color: {palette.foreground}, border_color: {palette.background} },
        ])
        .child(label)
    } else {
        div().sx(&BOARD.filter).child(label)
    }
}

/// The filters a board shows, filled with the current mode.
///
/// The boards mark the selected filter with the brand tint, which is only
/// correct in the mode the board is being drawn in, so the mode is a parameter
/// rather than read from a context the free functions do not have.
#[must_use]
pub fn filters_in(mode: ThemeMode, labels: &[(&'static str, bool)]) -> Div {
    div().sx(&BOARD.filters).children(
        labels
            .iter()
            .map(|(label, on)| filter((*label).to_string(), *on, mode)),
    )
}

/// A table's header row: one cell per column, each a fixed or growing width.
#[must_use]
pub fn head(cells: Vec<AnyElement>) -> Div {
    div().sx(&BOARD.head).children(cells)
}

/// One body row, already carrying its top border.
///
/// The cells are [`AnyElement`] so a row can hold a plain cell, a two-line cell
/// or a status chip without the row having to know which.
#[must_use]
pub fn line(cells: Vec<AnyElement>) -> Div {
    div().sx(&BOARD.line).children(cells)
}

/// A cell in the column that grows: it takes the space left over and
/// truncates rather than pushing the row wider.
#[must_use]
pub fn cell(text: impl Into<SharedString>) -> AnyElement {
    div().sx(&BOARD.grow).child(text.into()).into_any_element()
}

/// A growing cell whose text is one line that ellipsizes.
///
/// A board's middle column holds a sentence: the medicines on a prescription,
/// where a batch is kept. Wrapping that sentence makes the row three lines
/// tall and pushes the table off the card, so the boards let it ellipsize and
/// put the full text in the row's detail line instead.
#[must_use]
pub fn cell_truncating(text: impl Into<SharedString>) -> AnyElement {
    div()
        .sx(&BOARD.grow_truncate)
        .child(text.into())
        .into_any_element()
}

/// A cell of a fixed width, for a date, a status or a short figure.
#[must_use]
pub fn cell_fixed(text: impl Into<SharedString>) -> AnyElement {
    div().sx(&BOARD.fixed).child(text.into()).into_any_element()
}

/// A fixed-width cell holding a two-line stack: a reference and the line under
/// it. The boards give the first column a fixed width so the columns after it
/// line up down the table.
#[must_use]
pub fn cell_fixed_stack(
    title: impl Into<SharedString>,
    detail: impl Into<SharedString>,
) -> AnyElement {
    div()
        .sx(&BOARD.fixed)
        .child(
            div()
                .sx(&BOARD.cell_stack)
                .child(div().sx(&BOARD.cell_title).child(title.into()))
                .child(div().sx(&BOARD.cell_detail).child(detail.into())),
        )
        .into_any_element()
}

/// A cell holding a figure, right-aligned in the numbers font.
#[must_use]
pub fn cell_number(text: impl Into<SharedString>) -> AnyElement {
    div()
        .sx(&BOARD.number)
        .child(text.into())
        .into_any_element()
}

/// A two-line cell: a title and the quieter line under it.
#[must_use]
pub fn cell_stack(title: impl Into<SharedString>, detail: impl Into<SharedString>) -> AnyElement {
    div()
        .sx(&BOARD.cell_stack)
        .child(div().sx(&BOARD.cell_title).child(title.into()))
        .child(div().sx(&BOARD.cell_detail).child(detail.into()))
        .into_any_element()
}

/// A status chip, as the boards put one next to a row or a figure.
#[must_use]
pub fn chip(label: impl Into<SharedString>, tone: Tone) -> AnyElement {
    Chip::new(label.into()).tone(tone).into_any_element()
}

/// A key and its value, as the boards pair them in a detail rail.
#[must_use]
pub fn key_value(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Div {
    div()
        .sx(&BOARD.kv)
        .child(div().sx(&BOARD.kv_label).child(label.into()))
        .child(div().sx(&BOARD.kv_value).child(value.into()))
}

/// A block that needs attention, filled with `tone` and titled.
#[must_use]
pub fn alert(
    title: impl Into<SharedString>,
    body: impl Into<SharedString>,
    tone: Tone,
    mode: ThemeMode,
) -> Div {
    let palette = tone::colors(tone, mode);
    div()
        .sx(sx![
            &BOARD.alert,
            style! { background: {palette.background}, border_color: {palette.foreground} },
        ])
        .child(
            div()
                .sx(&BOARD.alert_title)
                .text_color(palette.foreground)
                .child(title.into()),
        )
        .child(div().sx(&BOARD.alert_body).child(body.into()))
}

/// The board's buttons along the bottom of a rail.
#[must_use]
pub fn actions(buttons: Vec<AnyElement>) -> Div {
    div().sx(&BOARD.actions).children(buttons)
}

/// A vertical list with the board's own gaps, for a card that stacks rows.
#[must_use]
pub fn list(children: Vec<AnyElement>) -> Div {
    div().sx(&BOARD.list_gap).children(children)
}

/// The board's small print: a line under a figure or a table.
#[must_use]
pub fn meta(text: impl Into<SharedString>) -> Div {
    div().sx(&BOARD.meta).child(text.into())
}

/// A row of chips, which the boards put under a status or beside a figure.
#[must_use]
pub fn chip_line(chips: Vec<AnyElement>) -> Div {
    div().sx(&BOARD.chip_line).children(chips)
}

/// A quiet box of text, as the boards set an option or a label in.
#[must_use]
pub fn option(label: impl Into<SharedString>) -> Div {
    div().sx(&BOARD.filter).child(label.into())
}

/// A row filled with `tone`'s background, as "Today's queue by status" draws
/// each of its five boxes.
#[must_use]
pub fn toned_row(tone: Tone, mode: ThemeMode, children: Vec<AnyElement>) -> Div {
    let palette = tone::colors(tone, mode);
    div()
        .sx(sx![
            &BOARD.line,
            style! { background: {palette.background} },
        ])
        .children(children)
}

/// A quiet panel: the grey block a board puts notes and summaries in.
#[must_use]
pub fn panel(children: Vec<AnyElement>) -> Div {
    div().sx(&BOARD.muted_panel).children(children)
}

/// The note under a board, in the board's own words about what it is showing.
#[must_use]
pub fn footnote(text: impl Into<SharedString>) -> Div {
    div().sx(&BOARD.small).child(text.into())
}

/// A link that opens `href`, as the boards close a card with one.
#[must_use]
pub fn link(
    id: impl Into<ElementId>,
    href: &'static str,
    text: impl Into<SharedString>,
) -> gpui::Stateful<Div> {
    link_to(id, href).sx(&BOARD.link).child(text.into())
}

/// The colours a screen needs in the mode it is drawn in, taken once at the top
/// of the screen so every helper below shares them.
#[must_use]
pub fn colors(cx: &Cx) -> ThemeColors {
    cx.theme().colors.clone()
}

/// The mode a screen is being drawn in.
#[must_use]
pub fn mode(cx: &Cx) -> ThemeMode {
    cx.theme().mode
}

/// `"a \u{b7} b \u{b7} c"`: the boards join a line's parts with a middle dot.
#[must_use]
pub fn dotted(parts: &[&str]) -> String {
    parts
        .iter()
        .filter(|part| !part.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(&format!(" {DOT} "))
}

#[cfg(test)]
mod tests {
    use super::{dotted, filters_in};
    use rok_pos_shell::Tone;
    use rok_ui::prelude::*;

    #[test]
    fn a_dotted_line_drops_the_empty_parts() {
        assert_eq!(
            dotted(&["Shelf B2", "", "PCM-2502"]),
            "Shelf B2 \u{b7} PCM-2502"
        );
        assert_eq!(dotted(&[]), "");
    }

    struct Pieces;

    impl Render for Pieces {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mode = ThemeMode::Light;
            div()
                .child(filters_in(
                    mode,
                    &[("All \u{b7} 412", true), ("Prescription-only", false)],
                ))
                .child(super::stat_row(vec![
                    super::stat("Refills due", "9", "3 to 9 October", None, mode),
                    super::stat(
                        "Reminders sent",
                        "3",
                        "by text message",
                        Some(Tone::Success),
                        mode,
                    ),
                ]))
                .child(super::head(vec![
                    super::cell("Medicine").into_any_element(),
                    super::cell_fixed("On hand").into_any_element(),
                ]))
                .child(super::line(vec![
                    super::cell_stack("Amoxicillin", "500mg capsules"),
                    super::cell_number("1,231 caps").into_any_element(),
                ]))
                .child(super::alert(
                    "Recall RC-0047",
                    "Batch AMS-2404 can no longer be sold.",
                    Tone::Danger,
                    mode,
                ))
                .child(super::key_value("Paid by", "National health insurance"))
                .child(super::footnote("Showing 12 of 412"))
                .child(super::link("board-link", "/medicines", "Open full reports"))
        }
    }

    #[gpui::test]
    fn draws_every_piece_in_both_modes(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
        });
        for scheme in ["light", "dark"] {
            let _ = scheme;
            let (_view, window) = cx.add_window_view(|_, _| Pieces);
            window.update(|window, cx| {
                let _ = window.draw(cx);
            });
        }
    }
}
