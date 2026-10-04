//! Numbers as the boards write them: grouped in threes, in Oxanium, so a column
//! of amounts lines up.

use rok_ui::prelude::*;

/// `"1,234,567"` from `1_234_567`, and `"-4,110"` from `-4_110`.
///
/// ```
/// # use rok_pos_shell::group_digits;
/// assert_eq!(group_digits(13_700), "13,700");
/// assert_eq!(group_digits(-4_110), "-4,110");
/// ```
#[must_use]
pub fn group_digits(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    group_digit_string(value.unsigned_abs().to_string().as_str(), sign)
}

/// Group the digits of an already-rendered unsigned number.
///
/// Split out so [`crate::money_text::format_money_precise`] can group a whole
/// amount without turning it back into an integer and losing its decimals.
#[must_use]
pub fn group_digit_string(digits: &str, sign: &str) -> String {
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3 + sign.len());
    grouped.push_str(sign);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

/// A number in the boards' numbers font, grouped and right-aligned.
///
/// A quantity, a stock count, a controlled-substance total.
#[component]
pub fn NumberText(
    value: i64,
    #[default(true)] grouped: bool,
    #[default] suffix: Option<SharedString>,
    cx: &mut Cx,
) -> impl IntoElement {
    let text = if grouped {
        group_digits(value)
    } else {
        value.to_string()
    };
    div()
        .sx(style! {
            font_family: mono,
            text_align: end,
            font: medium,
        })
        .child(text)
        .when_some(suffix, gpui::ParentElement::child)
}

#[cfg(test)]
mod tests {
    use super::{group_digit_string, group_digits};

    #[test]
    fn groups_in_threes_from_the_right() {
        assert_eq!(group_digits(0), "0");
        assert_eq!(group_digits(7), "7");
        assert_eq!(group_digits(999), "999");
        assert_eq!(group_digits(1_000), "1,000");
        assert_eq!(group_digits(13_700), "13,700");
        assert_eq!(group_digits(1_234_567), "1,234,567");
        assert_eq!(group_digits(1_000_000_000), "1,000,000,000");
    }

    #[test]
    fn keeps_the_sign_outside_the_grouping() {
        assert_eq!(group_digits(-4_110), "-4,110");
        assert_eq!(group_digits(-1_000), "-1,000");
        assert_eq!(group_digits(i64::MIN), "-9,223,372,036,854,775,808");
    }

    #[test]
    fn groups_a_rendered_number_with_a_sign() {
        assert_eq!(group_digit_string("4120", ""), "4,120");
        assert_eq!(group_digit_string("4120", "-"), "-4,120");
        assert_eq!(group_digit_string("4120", "+"), "+4,120");
    }
}
