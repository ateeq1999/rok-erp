//! Batch lines as a stock table and a dispensing ticket show them.

use rok_ui::prelude::*;

use crate::tone::Tone;

/// What stands between a batch code and its expiry on a board: `" | "`.
pub const SEPARATOR: &str = " | ";

/// `"B-2408-117 | Exp 08/2027"`: a batch code with the month and year it
/// expires, or just the code when no expiry is known.
///
/// ```
/// # use rok_pos_shell::format_batch_line;
/// assert_eq!(format_batch_line("B-2408-117", Some("08/2027")), "B-2408-117 | Exp 08/2027");
/// assert_eq!(format_batch_line("B-2408-117", None), "B-2408-117");
/// ```
#[must_use]
pub fn format_batch_line(code: &str, expiry: Option<&str>) -> String {
    match expiry {
        Some(expiry) => format!("{code}{SEPARATOR}Exp {expiry}"),
        None => code.to_string(),
    }
}

/// `"08/2027"` from `"2027-08-31"`: the month and year a board shows, with no
/// day for a dispenser to misread.
///
/// A value that is not an ISO date is handed back unchanged rather than
/// half-parsed.
///
/// ```
/// # use rok_pos_shell::format_expiry;
/// assert_eq!(format_expiry("2027-08-31"), "08/2027");
/// assert_eq!(format_expiry("soon"), "soon");
/// ```
#[must_use]
pub fn format_expiry(iso_date: &str) -> String {
    let digits = |part: &str, len: usize| {
        part.len() == len && part.chars().all(|char| char.is_ascii_digit())
    };
    match iso_date.split('-').collect::<Vec<_>>().as_slice() {
        [year, month, day] if digits(year, 4) && digits(month, 2) && digits(day, 2) => {
            format!("{month}/{year}")
        }
        _ => iso_date.to_string(),
    }
}

/// A batch code, and its expiry when there is one, in the numbers font.
///
/// `tone` is how urgent the batch looks: [`Tone::Warning`] inside a month of
/// expiry, [`Tone::Danger`] once it has passed.
#[component]
pub fn BatchCodeText(
    code: SharedString,
    #[default] expiry: Option<SharedString>,
    #[default] tone: Option<Tone>,
    cx: &mut Cx,
) -> impl IntoElement {
    let text = format_batch_line(&code, expiry.as_ref().map(|expiry| expiry.as_ref() as &str));
    let color = tone.map(|tone| crate::tone::colors(tone, cx.theme().mode).foreground);
    div()
        .sx(style! {
            font_family: mono,
            font: medium,
            color: {color.unwrap_or(cx.theme().colors.foreground)},
        })
        .child(text)
}

#[cfg(test)]
mod tests {
    use super::{format_batch_line, format_expiry};

    #[test]
    fn writes_the_code_and_its_expiry() {
        assert_eq!(
            format_batch_line("B-2408-117", Some("08/2027")),
            "B-2408-117 | Exp 08/2027"
        );
        assert_eq!(format_batch_line("B-2408-117", None), "B-2408-117");
    }

    #[test]
    fn shortens_an_iso_date_to_a_month_and_year() {
        assert_eq!(format_expiry("2027-08-31"), "08/2027");
        assert_eq!(format_expiry("2026-01-01"), "01/2026");
        assert_eq!(format_expiry("2026-12-09"), "12/2026");
    }

    #[test]
    fn hands_back_anything_that_is_not_an_iso_date() {
        assert_eq!(format_expiry("2027-8-31"), "2027-8-31");
        assert_eq!(format_expiry("2027-08"), "2027-08");
        assert_eq!(format_expiry("Aug 2027"), "Aug 2027");
        assert_eq!(format_expiry(""), "");
        assert_eq!(format_expiry("20a7-08-31"), "20a7-08-31");
    }
}
