//! The window: one frame, one route tree, and the pharmacy's theme.

use gpui::prelude::*;
use gpui::{Bounds, TitlebarOptions, WindowBounds, WindowOptions};
use rok_ui::prelude::*;

use pharmacy::routes;

/// How wide the window opens.
const WINDOW_WIDTH: gpui::Pixels = px(1440.);

/// How tall the window opens.
const WINDOW_HEIGHT: gpui::Pixels = px(900.);

struct Pharmacy;

impl Render for Pharmacy {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        routes::tree()
    }
}

fn main() {
    Application::new()
        .with_assets(rok_ui::Assets)
        .run(|cx: &mut App| {
            rok_ui::init(cx);
            rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
            rok_pos_shell::theme::install_theme(cx);
            let bounds = Bounds::centered(None, gpui::size(WINDOW_WIDTH, WINDOW_HEIGHT), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Afya Pharmacy".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    rok_pos_shell::theme::sync_with_system_appearance(window, cx);
                    cx.new(|_| Pharmacy)
                },
            )
            .expect("the window opens");
            cx.activate(true);
        });
}
