//! Amounts as a till, a receipt and a dashboard tile show them.

use rok_pos_domain::Money;
use rok_ui::prelude::*;

use crate::digits::group_digit_string;
use crate::tone::Tone;

/// What an amount says after the figure: `TZS`.
pub const CURRENCY: &str = "TZS";

/// `"13,700"`: whole shillings, which is how every board quotes a price.
///
/// ```
/// # use rok_pos_domain::Money;
/// # use rok_pos_shell::format_money;
/// assert_eq!(format_money(Money::from_shillings(13_700)), "13,700");
/// ```
#[must_use]
pub fn format_money(money: Money) -> String {
    crate::digits::group_digits(money.round_to_shillings())
}

/// `"TZS 13,700"`: with the currency, for anything a patient reads.
///
/// ```
/// # use rok_pos_domain::Money;
/// # use rok_pos_shell::format_amount;
/// assert_eq!(format_amount(Money::from_shillings(13_700)), "TZS 13,700");
/// ```
#[must_use]
pub fn format_amount(money: Money) -> String {
    format!("{CURRENCY} {}", format_money(money))
}

/// `"13,700.50"`: the two decimals the `numeric(18,2)` column keeps, for a line
/// total where the pi matter.
#[must_use]
pub fn format_money_precise(money: Money) -> String {
    let rendered = format!("{:.2}", money.decimal());
    match rendered.split_once('.') {
        Some((whole, pi)) => {
            let (sign, digits) = whole
                .strip_prefix('-')
                .map_or(("", whole), |rest| ("-", rest));
            format!("{}.{}", group_digit_string(digits, sign), pi)
        }
        None => rendered,
    }
}

/// The tone an amount wears: a refund or credit note is a danger, everything
/// else is the theme's own text.
#[must_use]
pub fn tone_for(money: Money) -> Option<Tone> {
    money.is_negative().then_some(Tone::Danger)
}

/// An amount in the boards' numbers font, right-aligned so a column lines up.
///
/// ```
/// # use rok_ui::prelude::*;
/// # use rok_pos_domain::Money;
/// # use rok_pos_shell::MoneyText;
/// # let _ = MoneyText::new(Money::from_shillings(13_700)).with_currency(true);
/// ```
#[component]
pub fn MoneyText(
    money: Money,
    #[default] with_currency: bool,
    #[default] precise: bool,
    #[default] tone: Option<Tone>,
    cx: &mut Cx,
) -> impl IntoElement {
    let text = match (with_currency, precise) {
        (true, true) => format!("{CURRENCY} {}", format_money_precise(money)),
        (true, false) => format_amount(money),
        (false, true) => format_money_precise(money),
        (false, false) => format_money(money),
    };
    let tone = tone.or_else(|| tone_for(money));
    let color = tone.map(|tone| crate::tone::colors(tone, cx.theme().mode).foreground);
    div()
        .sx(style! {
            font_family: mono,
            text_align: end,
            font: medium,
            color: {color.unwrap_or(cx.theme().colors.foreground)},
        })
        .child(text)
}

#[cfg(test)]
mod tests {
    use super::{format_amount, format_money, format_money_precise, tone_for};
    use crate::tone::Tone;
    use rok_pos_domain::Money;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn quotes_whole_shillings_with_a_thousands_separator() {
        assert_eq!(format_money(Money::from_shillings(0)), "0");
        assert_eq!(format_money(Money::from_shillings(950)), "950");
        assert_eq!(format_money(Money::from_shillings(13_700)), "13,700");
        assert_eq!(format_money(Money::from_shillings(-4_110)), "-4,110");
    }

    #[test]
    fn names_the_currency_when_a_patient_reads_it() {
        assert_eq!(format_amount(Money::from_shillings(13_700)), "TZS 13,700");
        assert_eq!(format_amount(Money::from_shillings(-500)), "TZS -500");
    }

    #[test]
    fn keeps_both_decimals_where_the_pi_matter() {
        let money = Money::from_decimal(Decimal::from_str("13700.50").unwrap()).unwrap();
        assert_eq!(format_money_precise(money), "13,700.50");
        let credit = Money::from_decimal(Decimal::from_str("-4120.05").unwrap()).unwrap();
        assert_eq!(format_money_precise(credit), "-4,120.05");
    }

    #[test]
    fn a_credit_wears_the_danger_tone() {
        assert_eq!(tone_for(Money::from_shillings(500)), None);
        assert_eq!(tone_for(Money::from_shillings(0)), None);
        assert_eq!(tone_for(Money::from_shillings(-500)), Some(Tone::Danger));
    }
}
