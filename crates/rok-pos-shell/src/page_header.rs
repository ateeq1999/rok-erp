//! The heading every page opens with: the title, what it is for, and the
//! actions that belong to it.

use rok_ui::prelude::*;

styles! {
    PAGE_HEADER = {
        root: {
            display: flex,
            flex_direction: row,
            align: start,
            justify: between,
            gap: 4,
        },
        titles: { display: flex, flex_direction: column, gap: 1 },
        title: { font: bold, text: xl, color: foreground },
        description: { text: sm, color: muted_foreground },
        actions: { display: flex, flex_direction: row, align: center, gap: 2 },
    }
}

/// A page's title, its one line of purpose, and its actions.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::PageHeader;
/// # fn draw() {
/// let header = PageHeader::new("Prescription queue", "Signed and awaiting check");
/// let with_actions = PageHeader::new("Prescription queue", "")
///     .actions(vec![
///         Button::new("new-prescription").label("New").variant(ButtonVariant::Primary).into_any_element(),
///     ]);
/// # }
/// ```
#[component]
pub fn PageHeader(
    title: SharedString,
    description: SharedString,
    #[default] actions: Vec<AnyElement>,
    #[sx] sx: Sx,
) -> impl IntoElement {
    div()
        .sx((&PAGE_HEADER.root, &sx))
        .child(
            div()
                .sx(&PAGE_HEADER.titles)
                .child(div().sx(&PAGE_HEADER.title).child(title))
                .when(!description.is_empty(), |titles| {
                    titles.child(div().sx(&PAGE_HEADER.description).child(description))
                }),
        )
        .when(!actions.is_empty(), |header| {
            header.child(div().sx(&PAGE_HEADER.actions).children(actions))
        })
}

#[cfg(test)]
mod tests {
    use super::PageHeader;
    use rok_ui::prelude::*;

    struct Header;

    impl Render for Header {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children([
                PageHeader::new("Prescription queue", "Signed and awaiting check")
                    .actions(vec![
                        Button::new("new-prescription")
                            .label("New prescription")
                            .variant(ButtonVariant::Primary)
                            .into_any_element(),
                    ])
                    .into_any_element(),
                PageHeader::new("Till", "").into_any_element(),
            ])
        }
    }

    #[gpui::test]
    fn draws_with_actions_and_on_its_own(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Header);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
