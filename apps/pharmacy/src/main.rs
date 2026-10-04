//! The window: one frame, one route tree, and the pharmacy's theme.
//!
//! `pharmacy --install` never opens a window: it installs every module
//! into the database `.env` or the environment names, loads the story seed
//! and reports what it did. That is phase 1's one command.

use gpui::prelude::*;
use gpui::{Bounds, TitlebarOptions, WindowBounds, WindowOptions};
use rok_ui::prelude::*;

use pharmacy::routes;
use rok_pos_database::{
    ModuleInstaller, default_modules_directory, default_story_seed_path, load_afya_story,
};

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

/// Run `pharmacy --install`: every module in dependency order, then the
/// story seed, on the database `DATABASE_URL` names.
///
/// The seed is what makes the boards show Afya's story rather than an empty
/// pharmacy, so it is loaded by default; `--no-seed` installs the schema only,
/// which is what a real pharmacy's first run wants.
fn install(arguments: &[String]) {
    let seed = !arguments.iter().any(|argument| argument == "--no-seed");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("the runtime starts");

    runtime.block_on(async {
        let database = rok_db::Db::connect_env()
            .await
            .expect("DATABASE_URL names a database the installer can reach");
        let installer = ModuleInstaller::load(&default_modules_directory())
            .expect("every module.toml reads and orders itself");

        let expected: usize = installer.modules().iter().map(|m| m.migrations.len()).sum();
        let report = installer
            .install(&database)
            .await
            .expect("every pending migration applies");

        for module in &report.modules {
            if module.applied.is_empty() {
                println!(
                    "  {} up to date ({} already applied)",
                    module.module_key, module.already_applied
                );
            } else {
                for migration in &module.applied {
                    println!("  {} {migration}", module.module_key);
                }
            }
        }
        println!(
            "installed {} modules, {} of {} migrations applied",
            report.modules.len(),
            report.total_applied(),
            expected
        );

        if seed {
            load_afya_story(&database)
                .await
                .expect("the story seed loads");
            println!("loaded {}", default_story_seed_path().display());
        }
        println!("install: ok");
    });
}

/// Read `.env` into the environment before anything asks for it.
///
/// dotenvy looks in the working directory and then its parents, and a name
/// already set in the environment wins over the file, so
/// `DATABASE_URL=... pharmacy --install` still overrides it. A missing `.env`
/// is not an error: the environment on its own is enough.
fn load_environment() {
    if let Ok(path) = dotenvy::dotenv() {
        println!("using {}", path.display());
    }
}

fn main() {
    load_environment();
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.iter().any(|argument| argument == "--install") {
        install(&arguments);
        return;
    }

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
