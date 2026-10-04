//! Generates the route tree from the files under `src/routes`.

fn main() {
    rok_ui_build::routes("src/routes")
        .generate()
        .expect("valid route files");
}
