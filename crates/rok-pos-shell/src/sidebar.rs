//! The left navigation column, as the `PharmacySidebar` board draws it: the
//! rok POS mark, the business and branch, the one big action, groups of
//! destinations with their counts, and who is signed in.
//!
//! The shell draws the column; the app says what goes in it. [`NavGroup`] and
//! [`NavItem`] are plain data, so `apps/pharmacy` can list its own
//! destinations and a supplier app can list its own.

use rok_ui::prelude::*;
use rok_ui::router::{is_active, navigate, preload};

use crate::theme::{ACCENT, ROK_MARK, hsla};
use crate::tone::{self, Tone};

/// How wide the column is, in pixels.
pub const WIDTH_PX: f32 = 240.;

/// How wide the column is in spacing units, which is how the styles read it.
pub const WIDTH: f32 = WIDTH_PX / 4.;

/// How tall a destination row is, in pixels.
pub const ROW_HEIGHT_PX: f32 = 34.;

/// The product's name beside its mark.
pub const PRODUCT_NAME: &str = "rok POS";

/// A count beside a destination: "6" prescriptions waiting, "1" recall open.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavBadge {
    /// What the badge says.
    pub text: SharedString,
    /// How urgent it is: [`Tone::Brand`] to inform, [`Tone::Warning`] or
    /// [`Tone::Danger`] when something is late or blocked.
    pub tone: Tone,
}

/// One destination in the sidebar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavItem {
    /// The words on the row.
    label: SharedString,
    /// The path it opens, as the router spells it: `/prescriptions`.
    href: SharedString,
    /// Count as current only on this exact path, not on paths below it.
    exact: bool,
    /// The count beside the words, when there is one.
    badge: Option<NavBadge>,
}

impl NavItem {
    /// A row for `label`.
    #[must_use]
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: SharedString::default(),
            exact: false,
            badge: None,
        }
    }

    /// The path this row opens. Default: none.
    #[must_use]
    pub fn href(mut self, href: impl Into<SharedString>) -> Self {
        self.href = href.into();
        self
    }

    /// Mark this row current only on its own path. Default: `false`.
    ///
    /// Worth it for `/prescriptions`, so `/prescriptions/42/check` does not light
    /// up both rows.
    #[must_use]
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = exact;
        self
    }

    /// A count beside the words. Default: none.
    #[must_use]
    pub fn badge(mut self, text: impl Into<SharedString>, tone: Tone) -> Self {
        self.badge = Some(NavBadge {
            text: text.into(),
            tone,
        });
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

    /// Whether this row counts as current only on its own path.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.exact
    }

    /// The count beside the words, when there is one.
    #[must_use]
    pub const fn badge_value(&self) -> Option<&NavBadge> {
        self.badge.as_ref()
    }
}

/// A labelled set of rows: "Dispensary", "Medicines & stock", "Money & rules".
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
    /// What they do here: "Pharmacist in charge".
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

/// The one big button under the business: "Open dispensary till".
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SidebarAction {
    /// The words on the button.
    pub label: SharedString,
    /// The path it opens.
    pub href: SharedString,
}

impl SidebarAction {
    /// A button that opens `href`.
    #[must_use]
    pub fn new(label: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
        }
    }
}

/// The letters on a business's square: the first letter of its first two words.
///
/// ```
/// # use rok_pos_shell::sidebar::initials;
/// assert_eq!(initials("Afya Pharmacy"), "AP");
/// assert_eq!(initials("uzima"), "U");
/// ```
#[must_use]
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .flat_map(char::to_uppercase)
        .collect()
}

styles! {
    SIDEBAR = {
        root: {
            display: flex,
            flex_direction: column,
            gap: 3,
            shrink: 0,
            width: {px(WIDTH_PX)},
            height: full,
            padding_y: 4,
            padding_x: 3.5,
            background: card,
            color: foreground,
            border_right: 1,
            border_color: border,
            font_family: sans,
            text: {14.},
        },
        brand: { display: flex, flex_direction: row, align: center, gap: 2.5, padding_x: 1.5 },
        mark: { position: relative, size: 7.5, shrink: 0, background: {hsla(ROK_MARK)} },
        product: { grow: 1, font_family: mono, text: {20.}, font: bold },
        icon_button: {
            display: flex,
            align: center,
            justify: center,
            size: 9,
            shrink: 0,
            color: muted_foreground,
            cursor: pointer,
        },
        business: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 2.5,
            padding_y: 2,
            padding_x: 2.5,
            border: 1,
            border_color: border,
            background: background,
            cursor: pointer,
        },
        business_square: {
            display: flex,
            align: center,
            justify: center,
            size: 8,
            shrink: 0,
            background: primary,
            color: primary_foreground,
            font_family: mono,
            font: bold,
        },
        names: { display: flex, flex_direction: column, grow: 1, min_width: 0 },
        name: { font: semibold, truncate: true },
        detail: { text: {12.}, color: muted_foreground, truncate: true },
        action: {
            display: flex,
            align: center,
            justify: center,
            height: 10.5,
            shrink: 0,
            background: primary,
            color: primary_foreground,
            text: {15.},
            font: semibold,
            cursor: pointer,
            hover: { background: primary/90 },
        },
        list: {
            grow: 1,
            min_height: 0,
            display: flex,
            flex_direction: column,
            gap: 3,
        },
        group: { display: flex, flex_direction: column, gap: {px(1.)} },
        group_label: {
            padding_x: 2.5,
            padding_bottom: 1,
            text: {11.},
            font: semibold,
            color: muted_foreground,
        },
        row: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 2.5,
            height: {px(ROW_HEIGHT_PX)},
            shrink: 0,
            padding_x: 2.5,
            cursor: pointer,
        },
        row_label: { grow: 1, truncate: true },
        badge: { padding_x: {px(7.)}, padding_y: {px(1.)}, text: {12.}, font: semibold },
        user: {
            display: flex,
            flex_direction: row,
            align: center,
            gap: 2.5,
            padding_top: 2.5,
            padding_x: 2,
            border_top: 1,
            border_color: border,
        },
        avatar: {
            display: flex,
            align: center,
            justify: center,
            size: 9,
            shrink: 0,
            radius: full,
            background: accent,
            color: accent_foreground,
            font: semibold,
        },
    }
}

/// A clickable box that also answers Enter and Space, so a keyboard reaches
/// everything a mouse does.
fn pressable(
    id: impl Into<ElementId>,
    on_press: impl Fn(&mut Window, &mut App) + 'static,
) -> gpui::Stateful<Div> {
    let on_press = std::rc::Rc::new(on_press);
    let keys = on_press.clone();
    div()
        .id(id)
        .tab_index(0)
        .on_click(move |_, window, cx| on_press(window, cx))
        .on_key_down(move |event, window, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                keys(window, cx);
            }
        })
}

/// Call `handler` when there is one.
fn fire(handler: Option<&EventHandler<()>>, window: &mut Window, cx: &mut App) {
    if let Some(handler) = handler {
        handler(&(), window, cx);
    }
}

/// The rok POS mark: an ember square with three bars, each fainter than the last.
fn mark() -> impl IntoElement {
    let bar = |left: f32, top: f32, width: f32, opacity: f32| {
        div()
            .absolute()
            .left(px(left))
            .top(px(top))
            .w(px(width))
            .h(px(4.))
            .bg(gpui::white().opacity(opacity))
    };
    div()
        .sx(&SIDEBAR.mark)
        .child(bar(7., 8., 16., 1.))
        .child(bar(11., 13., 12., 0.8))
        .child(bar(7., 18., 16., 0.6))
}

/// Four outlined squares: the way to every app this business has installed.
fn all_apps_glyph(color: Hsla) -> impl IntoElement {
    let square = || div().size(px(7.)).border_1().border_color(color);
    div()
        .flex()
        .flex_col()
        .gap(px(3.))
        .child(div().flex().gap(px(3.)).child(square()).child(square()))
        .child(div().flex().gap(px(3.)).child(square()).child(square()))
}

fn nav_row(
    item: &NavItem,
    active: bool,
    mode: ThemeMode,
    colors: &ThemeColors,
) -> impl IntoElement {
    let neutral = tone::colors(Tone::Neutral, mode);
    let (background, foreground) = if active {
        (colors.accent, colors.accent_foreground)
    } else {
        (colors.accent.alpha(0.), neutral.foreground)
    };
    let href = item.href.clone();
    let hover = item.href.clone();
    pressable(item.href.clone(), move |_, cx| navigate(href.clone(), cx))
        .on_hover(move |hovered, _, cx| {
            if *hovered && !hover.is_empty() {
                preload(&hover, cx);
            }
        })
        .sx(sx![
            &SIDEBAR.row,
            style! {
                background: {background},
                color: {foreground},
                hover: { background: {colors.accent} },
            },
            active.then_some(style! { font: semibold }),
        ])
        .child(div().sx(&SIDEBAR.row_label).child(item.label.clone()))
        .when_some(item.badge.clone(), |row, badge| {
            let palette = tone::colors(badge.tone, mode);
            row.child(
                div()
                    .sx(sx![
                        &SIDEBAR.badge,
                        style! { background: {palette.background}, color: {palette.foreground} },
                    ])
                    .child(badge.text),
            )
        })
}

/// The left column: product, business, the big action, destinations, and the
/// signed-in user.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_shell::{NavGroup, NavItem, Sidebar, SidebarAction, Tone};
/// let sidebar = Sidebar::new("Afya Pharmacy", "Mwenge branch")
///     .action(SidebarAction::new("Open dispensary till", "/till"))
///     .groups(vec![NavGroup::new("Dispensary", [
///         NavItem::new("Dashboard").href("/").exact(true),
///         NavItem::new("Prescriptions").href("/prescriptions").badge("6", Tone::Brand),
///     ])]);
/// ```
#[component]
pub fn Sidebar(
    business: SharedString,
    branch: SharedString,
    #[default] action: Option<SidebarAction>,
    #[default] groups: Vec<NavGroup>,
    #[default] user: Option<UserChip>,
    #[default] on_all_apps: Option<EventHandler<()>>,
    #[default] on_switch_business: Option<EventHandler<()>>,
    #[default] on_sign_out: Option<EventHandler<()>>,
    #[sx] sx: Sx,
    cx: &mut Cx,
) -> impl IntoElement {
    let colors = cx.theme().colors.clone();
    let mode = cx.theme().mode;
    let rendered: Vec<AnyElement> = groups
        .iter()
        .map(|group| {
            let rows: Vec<AnyElement> = group
                .items()
                .iter()
                .map(|item| {
                    let active = is_active(&item.href, item.exact, cx);
                    nav_row(item, active, mode, &colors).into_any_element()
                })
                .collect();
            div()
                .sx(&SIDEBAR.group)
                .when(!group.label.is_empty(), |column| {
                    column.child(
                        div()
                            .sx(&SIDEBAR.group_label)
                            .child(group.label.to_uppercase()),
                    )
                })
                .children(rows)
                .into_any_element()
        })
        .collect();

    div()
        .sx((&SIDEBAR.root, &sx))
        .child(
            div()
                .sx(&SIDEBAR.brand)
                .child(mark())
                .child(div().sx(&SIDEBAR.product).child(PRODUCT_NAME))
                .child(
                    pressable("sidebar-all-apps", move |window, cx| {
                        fire(on_all_apps.as_ref(), window, cx);
                    })
                    .sx(sx![
                        &SIDEBAR.icon_button,
                        style! { border: 1, border_color: border }
                    ])
                    .child(all_apps_glyph(colors.foreground)),
                ),
        )
        .child(
            pressable("sidebar-business", move |window, cx| {
                fire(on_switch_business.as_ref(), window, cx);
            })
            .sx(&SIDEBAR.business)
            .child(
                div()
                    .sx(&SIDEBAR.business_square)
                    .child(initials(&business)),
            )
            .child(
                div()
                    .sx(&SIDEBAR.names)
                    .child(div().sx(&SIDEBAR.name).child(business))
                    .child(div().sx(&SIDEBAR.detail).child(branch)),
            )
            .child(
                Icon::new(IconName::ChevronsUpDown)
                    .size(px(16.))
                    .color(colors.muted_foreground),
            ),
        )
        .when_some(action, |column, action| {
            let href = action.href.clone();
            column.child(
                pressable("sidebar-action", move |_, cx| navigate(href.clone(), cx))
                    .sx(&SIDEBAR.action)
                    .child(action.label),
            )
        })
        .child(
            div()
                .sx(&SIDEBAR.list)
                .id("sidebar-list")
                .overflow_y_scroll()
                .children(rendered),
        )
        .when_some(user, |column, user| {
            column.child(
                div()
                    .sx(&SIDEBAR.user)
                    .child(div().sx(&SIDEBAR.avatar).child(user.initials))
                    .child(
                        div()
                            .sx(&SIDEBAR.names)
                            .child(div().sx(&SIDEBAR.name).child(user.name))
                            .child(div().sx(&SIDEBAR.detail).child(user.role)),
                    )
                    .child(
                        pressable("sidebar-sign-out", move |window, cx| {
                            fire(on_sign_out.as_ref(), window, cx);
                        })
                        .sx(&SIDEBAR.icon_button)
                        .child(
                            Icon::new(IconName::LogOut)
                                .size(px(18.))
                                .color(colors.muted_foreground),
                        ),
                    ),
            )
        })
}

/// The teal the business square and the current row are drawn in, as one value,
/// so an app that draws its own shell can match it.
#[must_use]
pub fn brand_color() -> Hsla {
    hsla(ACCENT)
}

#[cfg(test)]
mod tests {
    use super::{NavGroup, NavItem, Sidebar, SidebarAction, UserChip, brand_color, initials};
    use crate::theme::{ACCENT, hsla};
    use crate::tone::Tone;
    use rok_ui::prelude::*;

    #[test]
    fn an_item_keeps_its_words_path_and_badge() {
        let item = NavItem::new("Prescriptions")
            .href("/prescriptions")
            .exact(true)
            .badge("6", Tone::Brand);
        assert_eq!(item.label(), "Prescriptions");
        assert_eq!(item.target(), "/prescriptions");
        assert!(item.is_exact());
        let badge = item.badge_value().expect("a badge");
        assert_eq!(badge.text, "6");
        assert_eq!(badge.tone, Tone::Brand);
        assert!(NavItem::new("Patients").badge_value().is_none());
    }

    #[test]
    fn a_group_keeps_its_rows_in_order() {
        let group = NavGroup::new(
            "Dispensary",
            [
                NavItem::new("Dashboard").href("/"),
                NavItem::new("Prescriptions").href("/prescriptions"),
            ],
        );
        assert_eq!(group.label(), "Dispensary");
        let labels: Vec<&SharedString> = group.items().iter().map(NavItem::label).collect();
        assert_eq!(labels, ["Dashboard", "Prescriptions"]);
    }

    #[test]
    fn a_business_square_carries_two_initials_at_most() {
        assert_eq!(initials("Afya Pharmacy"), "AP");
        assert_eq!(initials("Uzima Pharmaceuticals Limited"), "UP");
        assert_eq!(initials("afya"), "A");
        assert_eq!(initials("  "), "");
    }

    #[test]
    fn the_brand_colour_is_the_boards_teal() {
        assert_eq!(brand_color(), hsla(ACCENT));
    }

    struct Frame;

    impl Render for Frame {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            Sidebar::new("Afya Pharmacy", "Mwenge branch")
                .action(SidebarAction::new("Open dispensary till", "/till"))
                .groups(vec![NavGroup::new(
                    "Dispensary",
                    [
                        NavItem::new("Dashboard").href("/").exact(true),
                        NavItem::new("Prescriptions")
                            .href("/prescriptions")
                            .badge("6", Tone::Brand),
                        NavItem::new("Recalls")
                            .href("/recalls")
                            .badge("1", Tone::Danger),
                    ],
                )])
                .user(UserChip::new("GN", "Grace N.", "Pharmacist in charge"))
                .on_sign_out(|(), _, _| {})
        }
    }

    #[gpui::test]
    fn draws_with_groups_badges_and_a_user(cx: &mut gpui::TestAppContext) {
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
