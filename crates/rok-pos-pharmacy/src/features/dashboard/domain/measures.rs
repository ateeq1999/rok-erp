//! A figure the screen cannot know the shape of.
//!
//! A widget that is handed `"38%"` has to take the string apart again to draw a
//! percentage. Handing it a [`MeasureValue`] keeps the number a number until the
//! moment it becomes text.

use rok_pos_domain::Money;

/// A figure the screen draws as itself, not as something to take apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasureValue {
    /// An amount in shillings.
    Money(Money),
    /// How many of something.
    Count(u32),
    /// A whole percentage.
    Percent(i64),
}

impl MeasureValue {
    /// The number behind the figure, for a screen that has to compare or sort.
    #[must_use]
    pub fn amount(self) -> i64 {
        match self {
            MeasureValue::Money(money) => {
                if money.is_negative() {
                    -money.round_to_shillings()
                } else {
                    money.round_to_shillings()
                }
            }
            MeasureValue::Count(count) => i64::from(count),
            MeasureValue::Percent(percent) => percent,
        }
    }
}
