//! The language the pharmacy speaks, and the direction it draws in.
//!
//! English draws left to right; Arabic draws right to left, through rok-ui's
//! `Direction`, so every flex row, sidebar and table mirrors. Switching the
//! language switches the locale the `t!` macro reads and the direction every
//! screen is laid out in, in one place.
//!
//! The button lives in the top bar beside the appearance toggle, because a
//! language the user cannot find is a language they cannot use.

use gpui::prelude::*;
use rok_ui::bidi;
use rok_ui::components::direction::{self, TextDirection};
use rok_ui::prelude::*;

use rust_i18n::t;

/// The languages the pharmacy speaks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Language {
    /// English, drawn left to right.
    #[default]
    English,
    /// Arabic, drawn right to left.
    Arabic,
}

/// The language, where a window-wide setting lives.
struct LanguageSetting(Language);

impl gpui::Global for LanguageSetting {}

impl Language {
    /// The language the app draws in right now.
    #[must_use]
    pub fn current(cx: &App) -> Self {
        cx.try_global::<LanguageSetting>()
            .map_or(Self::default(), |setting| setting.0)
    }

    /// Make `self` the language, the locale and the text direction, and redraw
    /// everything in it.
    pub fn set(self, cx: &mut App) {
        cx.set_global(LanguageSetting(self));
        rust_i18n::set_locale(self.locale());
        direction::set_text_direction(self.direction(), cx);
    }

    /// Switch to the other language.
    pub fn toggle(cx: &mut App) {
        Self::current(cx).next().set(cx);
    }

    /// The BCP 47 code `t!` reads.
    #[must_use]
    pub const fn locale(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Arabic => "ar",
        }
    }

    /// The name of the language, in itself: the toggle never has to be
    /// translated for the reader to find their language.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::English => t!("language.english").to_string(),
            Self::Arabic => t!("language.arabic").to_string(),
        }
    }

    /// The other language.
    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::English => Self::Arabic,
            Self::Arabic => Self::English,
        }
    }

    /// The direction the language draws in.
    #[must_use]
    pub const fn direction(self) -> TextDirection {
        match self {
            Self::English => TextDirection::Ltr,
            Self::Arabic => TextDirection::Rtl,
        }
    }

    /// The language a BCP 47 code draws in: `"ar"`, `"ar-TZ"`, `"en"`.
    #[must_use]
    pub fn from_locale(code: &str) -> Self {
        if code.starts_with("ar") {
            Self::Arabic
        } else {
            Self::English
        }
    }
}

styles! {
    LOCALE = {
        root: {
            display: flex,
            flex_direction: row,
            align: center,
            justify: center,
            gap: 1.5,
            height: 10,
            shrink: 0,
            padding_x: 2.5,
            border: 1,
            border_color: border,
            background: card,
            cursor: pointer,
            hover: { background: muted },
        },
        word: { text: {13.}, font: medium },
    }
}

/// The language button the frame puts in the top bar, beside the appearance
/// toggle. One click switches to the other language and mirrors every screen.
#[component]
fn LanguageToggle(#[sx] sx: Sx, cx: &mut Cx) -> impl IntoElement {
    let current = Language::current(cx.app);
    let next = current.next();
    div()
        .id("top-bar-language")
        .debug_selector(|| "top-bar-language".to_string())
        .tab_index(0)
        .sx((&LOCALE.root, sx))
        .on_click(move |_, _, cx| next.set(cx))
        .on_key_down(move |event, _, cx| {
            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                cx.stop_propagation();
                next.set(cx);
            }
        })
        .child(
            div()
                .sx(&LOCALE.word)
                .child(bidi::display_text(current.label(), current.direction())),
        )
}

/// The language toggle, as the frame's top bar action.
#[must_use]
pub fn toggle() -> AnyElement {
    LanguageToggle::new().into_any_element()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Language;
    use rok_ui::components::direction::{self, TextDirection};
    use rok_ui::prelude::*;
    use rust_i18n::t;

    /// Every key `en.yml` declares, read from the crate's own `locales`
    /// directory, so this test cannot lie about the file it checks.
    fn english_keys() -> BTreeSet<String> {
        locale_keys(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/en.yml"))
    }

    /// The key set of a two-space-indented locale file. Fails loudly on
    /// anything else, so a format change cannot silently hollow the parity
    /// test out.
    fn locale_keys(path: &str) -> BTreeSet<String> {
        let source =
            std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"));
        let mut keys = BTreeSet::new();
        let mut branch: Vec<&str> = Vec::new();
        for line in source.lines() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            assert!(
                indent % 2 == 0,
                "{path}: odd indentation, the locale files use two spaces\n    {line}"
            );
            let depth = indent / 2;
            branch.truncate(depth);
            let rest = line.trim_start();
            if rest == "---" {
                continue;
            }
            assert!(
                !rest.starts_with("- "),
                "{path}: lists are not locale keys\n    {line}"
            );
            let (name, value) = rest
                .split_once(':')
                .unwrap_or_else(|| panic!("{path}: not a `key: value` line\n    {line}"));
            let name = name.trim();
            assert!(
                !name.is_empty(),
                "{path}: a key is missing its name\n    {line}"
            );
            let value = value.trim();
            branch.push(name);
            if value.is_empty() {
                // A nested mapping: the key continues on the deeper lines.
                continue;
            }
            keys.insert(branch.join("."));
        }
        assert!(
            !keys.is_empty(),
            "{path}: no keys were read; the format changed"
        );
        keys
    }

    /// A key present in English but not Arabic would fall back to English in
    /// the middle of an Arabic screen; a key present in Arabic but not
    /// English would break the same way when the fallback locale speaks. Both
    /// files must therefore declare the same keys.
    #[test]
    fn english_and_arabic_declare_the_same_keys() {
        let english = english_keys();
        let arabic = locale_keys(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/ar.yml"));
        let missing_from_arabic: Vec<_> = english.difference(&arabic).collect();
        assert!(
            missing_from_arabic.is_empty(),
            "ar.yml is missing keys that en.yml declares (they would fall back to English): {missing_from_arabic:?}"
        );
        let missing_from_english: Vec<_> = arabic.difference(&english).collect();
        assert!(
            missing_from_english.is_empty(),
            "en.yml is missing keys that ar.yml declares: {missing_from_english:?}"
        );
        assert!(!english.is_empty());
    }

    #[test]
    fn arabic_and_english_say_the_same_things_in_both_languages() {
        for key in [
            "nav.dashboard",
            "nav.prescriptions",
            "nav.patients",
            "patient.list.title",
            "patient.record.insurance",
            "common.retry",
        ] {
            let english = t!(key).to_string();
            let arabic = t!(key, locale = "ar").to_string();
            assert!(!english.is_empty(), "{key}");
            assert!(!arabic.is_empty(), "{key}");
            assert_ne!(english, arabic, "{key} is not translated");
            assert!(
                arabic
                    .chars()
                    .any(|letter| ('\u{0600}'..='\u{06FF}').contains(&letter)),
                "{key} has no Arabic letters: {arabic}"
            );
        }
    }

    #[test]
    fn the_language_sets_the_direction_the_way_its_locale_spells_it() {
        assert_eq!(Language::from_locale("ar"), Language::Arabic);
        assert_eq!(Language::from_locale("ar-TZ"), Language::Arabic);
        assert_eq!(Language::from_locale("en"), Language::English);
        assert_eq!(Language::English.direction(), TextDirection::Ltr);
        assert_eq!(Language::Arabic.direction(), TextDirection::Rtl);
    }

    /// One page, for tests that press the language button.
    struct Page;

    impl Render for Page {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            crate::locale::toggle()
        }
    }

    #[gpui::test]
    fn pressing_the_button_switches_the_language_and_mirrors(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            rok_ui::init(cx);
            rok_pos_shell::fonts::install(cx).expect("the bundled fonts load");
            rok_ui::fonts::NOTO_SANS_ARABIC
                .register(cx)
                .expect("the Arabic font loads");
        });
        let (_view, window) = cx.add_window_view(|_, _| Page);
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
        assert_eq!(direction::current_direction(), TextDirection::Ltr);

        let bounds = window
            .debug_bounds("top-bar-language")
            .expect("the language button drew");
        window.simulate_click(bounds.center(), gpui::Modifiers::none());
        window.run_until_parked();
        window.update(|_, _cx| {
            assert_eq!(direction::current_direction(), TextDirection::Rtl);
            assert_eq!(&*rust_i18n::locale(), "ar");
        });

        // Leave the process the way every other test expects it.
        window.update(|window, cx| {
            Language::English.set(cx);
            let _ = window.draw(cx);
        });
        assert_eq!(&*rust_i18n::locale(), "en");
    }
}
