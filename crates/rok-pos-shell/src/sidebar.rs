//! The left navigation column: the brand, groups of destinations, and who is
//! signed in.
//!
//! The shell draws the column; the app says what goes in it. [`NavGroup`] and
//! [`NavItem`] are plain data, so `apps/rok-pharmacy` can list its own
//! destinations and a supplier app can list its own.

use rok_ui::prelude::*;
use rok_ui::router::{is_active, navigate, preload};

use crate::theme::{ACCENT, ACCENT_STRONG, ACCENT_TINT, hsla};

/// How wide the column is, in pixels.
pub const WIDTH_PX: f32 = 240.;

/// How wide the column is in spacing units, which is how the styles read it.
pub const WIDTH: f32 = WIDTH_PX / 4.;

/// How tall a destination row is, in spacing units (36 pixels).
pub const ROW_HEIGHT: f32 = 9.;

/// One destination in the sidebar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavItem {
    /// The words on the row.
    label: SharedString,
    /// The glyph beside them.
    icon: IconName,
    /// The path it opens, as the router spells it: `/prescriptions`.
    href: SharedString,
    /// Count as current only on this exact path, not on paths below it.
    exact: bool,
}

impl NavItem {
    /// A row for `label` with `icon` beside it.
    #[must_use]
    pub fn new(label: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            label: label.into(),
            icon,
            href: SharedString::default(),
            exact: false,
        }
    }

    /// The path this row opens.
    #[must_use]
    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = href.into();
        self
    }

    /// Mark this row current only on its own path.
    ///
    /// Worth it for `/prescriptions`, so `/prescriptions/42/check` does not light
    /// up both rows.
    #[must_use]
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = exact;
        self
    }

    /// The words on the row.
    #[must_use]
    pub fn label(&self) -> &SharedString {
        &self.label
    }

    /// The path this row opens.
    #[must_use]
    pub fn target(&self) -> &SharedString {
        &self.href
    }

    /// The glyph beside the words.
    #[must_use]
    pub const fn icon(&self) -> IconName {
        self.icon
    }

    /// Whether this row counts as current only on its own path.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.exact
    }
}

/// A labelled set of rows: "Dispensing", "Inventory", "Records", "Admin".
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NavGroup {
    label: SharedString,
    items: Vec<NavItem>,
}

impl NavGroup {
    /// A group headed by `label`, holding `items`.
    #[must_use]
    pub fn new(label: impl Into<SharedString>, items: impl IntoIterator<Item = NavItem>) -> Self {
        Self {
            label: label.into(),
            items: items.into_iter().collect(),
        }
    }

    /// The heading above the rows.
    #[must_use]
    pub fn label(&self) -> &SharedString {
        &self.label
    }

    /// The rows in this group, in the order they are drawn.
    #[must_use]
    pub fn items(&self) -> &[NavItem] {
        &self.items
    }
}

/// Who is signed in, drawn at the foot of the column.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UserChip {
    /// Up to two letters, as on an avatar.
    pub initials: SharedString,
    /// The person's name.
    pub name: SharedString,
    /// What they do here: "Pharmacist".
    pub role: SharedString,
}

impl UserChip {
    /// A chip for one person.
    #[must_use]
    pub fn new(
        initials: impl Into<SharedString>,
        name: impl Into<SharedString>,
        role: impl Into<SharedString>,
    ) -> Self {
        Self {
            initials: initials.into(),
            name: name.into(),
            role: role.into(),
        }
    }
}

styles! {
    SIDEBAR = {
        root: {
            display: flex,
            flex_direction: column,
            shrink: 0,
            height: full,
            background: card,
            color: foreground,
            border_right: 1,
            border_color: border,
            font_family: sans,
            text: base,
        },
        brand: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 3,
            padding: 4,
            border_bottom: 1,
            border_color: border,
        },
        mark: {
            display: flex,
            align: center,
            justify: center,
            size: 8,
            background: primary,
            color: primary_foreground,
            font: bold,
        },
        brand_name: { font: semibold, color: foreground },
        brand_tagline: { text: xs, color: muted_foreground },
        list: {
            flex: 1,
            display: flex,
            flex_direction: column,
            gap: 4,
            padding: 3,
        },
        group_label: {
            padding_x: 3,
            padding_y: 1,
            text: xs,
            font: semibold,
            color: muted_foreground,
        },
        row: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 3,
            height: ROW_HEIGHT,
            padding_x: 3,
            radius: none,
            cursor: pointer,
        },
        user: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 3,
            padding: 3,
            border_top: 1,
            border_color: border,
        },
        avatar: {
            display: flex,
            align: center,
            justify: center,
            size: 8,
            background: accent,
            color: accent_foreground,
            font: semibold,
            text: xs,
            radius: full,
        },
        user_name: { font: medium, color: foreground },
        user_role: { text: xs, color: muted_foreground },
    }
}

/// The colours a destination row is drawn in.
struct RowColors {
    background: gpui::Hsla,
    foreground: gpui::Hsla,
    hover: gpui::Hsla,
}

/// The colours for a row that is (or is not) the current page.
fn row_colors(active: bool, colors: &ThemeColors) -> RowColors {
    if active {
        RowColors {
            background: hsla(ACCENT_TINT),
            foreground: hsla(ACCENT_STRONG),
            hover: hsla(ACCENT_TINT),
        }
    } else {
        RowColors {
            background: hsla(ACCENT_TINT).alpha(0.),
            foreground: colors.muted_foreground,
            hover: hsla(ACCENT_TINT).alpha(0.5),
        }
    }
}

fn nav_row(item: &NavItem, active: bool, colors: &ThemeColors) -> impl IntoElement {
    let palette = row_colors(active, colors);
    let click = item.href.clone();
    let keys = item.href.clone();
    let hover = item.href.clone();
    div()
        .id(item.href.clone())
        .tab_index(0)
        .on_click(move |_, _, cx| navigate(click.clone(), cx))
        .on_key_down(move |event, _, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                navigate(keys.clone(), cx);
            }
        })
        .on_hover(move |hovered, _, cx| {
            if *hovered && !hover.is_empty() {
                preload(&hover, cx);
            }
        })
        .sx(sx![
            &SIDEBAR.row,
            style! {
                background: {palette.background},
                color: {palette.foreground},
                hover: { background: {palette.hover} },
            },
            active.then_some(style! { font: semibold }),
        ])
        .child(Icon::new(item.icon).size(px(16.)).color(palette.foreground))
        .child(item.label.clone())
}

fn nav_group(label: &SharedString, items: Vec<impl IntoElement>) -> impl IntoElement {
    div()
        .when(!label.is_empty(), |group| {
            group.child(div().sx(&SIDEBAR.group_label).child(label.to_uppercase()))
        })
        .children(items)
}

/// The left column: brand, destinations, and the signed-in user.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::{NavGroup, NavItem, Sidebar};
/// let sidebar = Sidebar::new("Afya Pharmacy", "Dispensing & inventory").groups(vec![
///     NavGroup::new("Dispensing", [
///         NavItem::new("Overview", IconName::Home).href("/"),
///         NavItem::new("Prescriptions", IconName::FileText).href("/prescriptions").exact(true),
///     ]),
/// ]);
/// ```
#[component]
pub fn Sidebar(
    brand: SharedString,
    tagline: SharedString,
    #[default] groups: Vec<NavGroup>,
    #[default] user: Option<UserChip>,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let mut rendered = Vec::new();
    for group in &groups {
        let mut rows = Vec::new();
        for item in group.items() {
            let active = is_active(&item.href, item.exact, cx);
            rows.push(nav_row(item, active, &colors));
        }
        rendered.push(nav_group(&group.label, rows));
    }
    div()
        .sx((&SIDEBAR.root, &sx))
        .child(
            div()
                .sx(&SIDEBAR.brand)
                .child(div().sx(&SIDEBAR.mark).child("A"))
                .child(
                    div()
                        .child(div().sx(&SIDEBAR.brand_name).child(brand))
                        .child(div().sx(&SIDEBAR.brand_tagline).child(tagline)),
                ),
        )
        .child(
            div()
                .sx(&SIDEBAR.list)
                .id("sidebar-list")
                .overflow_y_scroll()
                .children(rendered),
        )
        .when_some(user, |sidebar, user| {
            sidebar.child(
                div()
                    .sx(&SIDEBAR.user)
                    .child(div().sx(&SIDEBAR.avatar).child(user.initials))
                    .child(
                        div()
                            .child(div().sx(&SIDEBAR.user_name).child(user.name))
                            .child(div().sx(&SIDEBAR.user_role).child(user.role)),
                    ),
            )
        })
}

/// The teal the brand mark and the current row are drawn in, as one value, so an
/// app that draws its own shell can match it.
#[must_use]
pub fn brand_color() -> gpui::Hsla {
    hsla(ACCENT)
}

#[cfg(test)]
mod tests {
    use super::{NavGroup, NavItem, Sidebar, UserChip, brand_color};
    use crate::theme::{ACCENT, hsla};
    use rok_ui::prelude::*;

    #[test]
    fn an_item_keeps_its_words_icon_and_path() {
        let item = NavItem::new("Prescriptions", IconName::FileText)
            .href("/prescriptions")
            .exact(true);
        assert_eq!(item.label(), "Prescriptions");
        assert_eq!(item.target(), "/prescriptions");
        assert_eq!(item.icon(), IconName::FileText);
        assert!(item.is_exact());
    }

    #[test]
    fn a_group_keeps_its_rows_in_order() {
        let group = NavGroup::new(
            "Dispensing",
            [
                NavItem::new("Overview", IconName::Home).href("/"),
                NavItem::new("Prescriptions", IconName::FileText).href("/prescriptions"),
            ],
        );
        assert_eq!(group.label(), "Dispensing");
        let labels: Vec<&SharedString> = group.items().iter().map(NavItem::label).collect();
        assert_eq!(labels, ["Overview", "Prescriptions"]);
    }

    #[test]
    fn the_brand_colour_is_the_boards_teal() {
        assert_eq!(brand_color(), hsla(ACCENT));
    }

    struct Frame;

    impl Render for Frame {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Sidebar::new("Afya Pharmacy", "Dispensing & inventory")
                .groups(vec![NavGroup::new(
                    "Dispensing",
                    [
                        NavItem::new("Overview", IconName::Home).href("/"),
                        NavItem::new("Prescriptions", IconName::FileText)
                            .href("/prescriptions")
                            .exact(true),
                    ],
                )])
                .user(UserChip::new("AT", "Amani T.", "Pharmacist"))
        }
    }

    #[gpui::test]
    fn draws_with_groups_and_a_user(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            crate::fonts::install(cx).expect("the bundled fonts load");
        });
        let (_view, window) = cx.add_window_view(|_, _| Frame);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
    }
}
