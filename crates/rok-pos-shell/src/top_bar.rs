//! The bar across the top: the page's heading, the line under it, and whatever a
//! screen puts on the right.
//!
//! Every board opens with `<PosTopBar heading=.. subheading=..>` at 64 pixels
//! tall, full width, above a `main` padded `20px 28px`.

use rok_ui::prelude::*;

/// How tall the top bar is on the boards, in pixels.
pub const HEIGHT_PX: f32 = 64.;

/// How tall the top bar is in spacing units, which is how the styles read it.
pub const HEIGHT: f32 = HEIGHT_PX / 4.;

styles! {
    TOP_BAR = {
        root: {
            display: flex,
            flex_direction: row,
            align: center,
            justify: between,
            gap: 4,
            height: HEIGHT,
            shrink: 0,
            padding_x: 7,
            background: card,
            color: foreground,
            border_bottom: 1,
            border_color: border,
            font_family: sans,
            text: sm,
        },
        titles: { display: flex, flex_direction: column, gap: 0.5 },
        heading: { font: semibold, text: base, color: foreground },
        subheading: { text: xs, color: muted_foreground },
        actions: { display: flex, flex_direction: row, align: center, gap: 3 },
    }
}

/// The bar across the top of a page.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::TopBar;
/// let bar = TopBar::new("Prescriptions", "Afya Pharmacy - Mwenge branch - Friday 2 October");
/// ```
#[component]
pub fn TopBar(
    heading: SharedString,
    subheading: SharedString,
    #[children] children: Vec<AnyElement>,
    #[sx] sx: Sx,
) -> impl IntoElement {
    div()
        .sx((&TOP_BAR.root, &sx))
        .child(
            div()
                .sx(&TOP_BAR.titles)
                .child(div().sx(&TOP_BAR.heading).child(heading))
                .when(!subheading.is_empty(), |titles| {
                    titles.child(div().sx(&TOP_BAR.subheading).child(subheading))
                }),
        )
        .when(!children.is_empty(), |bar| {
            bar.child(div().sx(&TOP_BAR.actions).children(children))
        })
}

#[cfg(test)]
mod tests {
    use super::TopBar;
    use rok_ui::prelude::*;

    struct Bar;

    impl Render for Bar {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children([
                TopBar::new(
                    "Dashboard",
                    "Afya Pharmacy - Mwenge branch - Fri 2 Oct 2026, 15:30",
                )
                .into_any_element(),
                TopBar::new("Prescriptions", "")
                    .child(Button::new("new").label("New"))
                    .into_any_element(),
            ])
        }
    }

    #[gpui::test]
    fn draws_with_a_subheading_and_with_actions(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Bar);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
