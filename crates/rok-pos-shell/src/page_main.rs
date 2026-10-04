//! The page a route draws under its top bar.
//!
//! Every board's `<main>` is padded `20px 28px`, and the pieces inside it are
//! four spacing units apart.

use rok_ui::prelude::*;

/// How far a page is padded, in pixels: 20 down, 28 across.
pub const PADDING_Y_PX: f32 = 20.;

/// How far a page is padded across, in pixels.
pub const PADDING_X_PX: f32 = 28.;

styles! {
    PAGE_MAIN = {
        root: {
            flex: 1,
            min_height: 0,
            display: flex,
            flex_direction: column,
            gap: 4,
            padding_y: PADDING_Y_PX / 4.,
            padding_x: PADDING_X_PX / 4.,
        },
    }
}

/// The page under a top bar.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::PageMain;
/// let page = PageMain::new().child(Button::new("new").label("New"));
/// ```
#[component]
pub fn PageMain(#[children] children: Vec<AnyElement>, #[sx] sx: Sx) -> impl IntoElement {
    div()
        .sx((&PAGE_MAIN.root, &sx))
        .id("page-main")
        .overflow_y_scroll()
        .children(children)
}

#[cfg(test)]
mod tests {
    use super::PageMain;
    use rok_ui::prelude::*;

    struct Page;

    impl Render for Page {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            PageMain::new().children([
                Button::new("a").label("Order medicines").into_any_element(),
                Button::new("b")
                    .label("Receive delivery")
                    .into_any_element(),
            ])
        }
    }

    #[gpui::test]
    fn draws_a_page_with_several_pieces(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Page);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
