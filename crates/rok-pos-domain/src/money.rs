//! An amount of Tanzanian shillings.
//!
//! The database stores money as `numeric(18,2)`, so a [`Money`] keeps at most
//! two decimal places and refuses anything else rather than rounding it away.
//! Till screens show whole shillings; [`Money::round_to_shillings`] is what
//! they draw, and the fractional part stays in the value until then.

use std::ops::{Add, Neg, Sub};

use rust_decimal::{Decimal, RoundingStrategy, prelude::ToPrimitive};

use crate::error::DomainError;

/// An amount of money, at most two decimal places.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(Decimal);

impl Money {
    /// No money at all: the starting point of a basket, an invoice or a claim.
    #[must_use]
    pub const fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    /// An amount given in whole shillings, as every figure on the boards is.
    ///
    /// ```
    /// use rok_pos_domain::Money;
    ///
    /// assert_eq!(Money::from_shillings(13_700).decimal().to_string(), "13700");
    /// ```
    #[must_use]
    pub fn from_shillings(shillings: i64) -> Self {
        Self(Decimal::new(shillings, 0))
    }

    /// An amount read from the database.
    ///
    /// # Errors
    ///
    /// Fails when the value carries more than two decimal places, which means
    /// the column it came from is not the `numeric(18,2)` this type expects.
    pub fn from_decimal(value: Decimal) -> Result<Self, DomainError> {
        if value.round_dp(2) == value {
            Ok(Self(value))
        } else {
            Err(DomainError::TooPrecise { value })
        }
    }

    /// The value as stored.
    #[must_use]
    pub const fn decimal(self) -> Decimal {
        self.0
    }

    /// The amount as whole shillings, rounding a half shilling away from zero.
    ///
    /// Prices are quoted in shillings, so this is what a till, a receipt or a
    /// dashboard tile shows. A `numeric(18,2)` amount always fits in an `i64`;
    /// `i64::MAX` stands in for a value that somehow does not.
    #[must_use]
    pub fn round_to_shillings(self) -> i64 {
        self.0
            .round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero)
            .to_i64()
            .unwrap_or(i64::MAX)
    }

    /// `true` when there is nothing to record or claim.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// `true` when the amount is below zero, as a refund or a credit note is.
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.0.is_sign_negative() && !self.0.is_zero()
    }

    /// The sum of two amounts.
    ///
    /// # Errors
    ///
    /// Fails when the sum leaves the range a money value can hold.
    pub fn checked_add(self, other: Self) -> Result<Self, DomainError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(DomainError::OutOfRange {
                left: self.0,
                right: other.0,
            })
    }

    /// The difference between two amounts, which may be negative.
    ///
    /// # Errors
    ///
    /// Fails when the difference leaves the range a money value can hold.
    pub fn checked_sub(self, other: Self) -> Result<Self, DomainError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(DomainError::OutOfRange {
                left: self.0,
                right: other.0,
            })
    }

    /// The amount scaled by a rate, for example a margin percentage.
    ///
    /// # Errors
    ///
    /// Fails when the result leaves the range a money value can hold, or when
    /// scaling leaves more than two decimal places behind.
    pub fn checked_mul(self, rate: Decimal) -> Result<Self, DomainError> {
        let scaled = self.0.checked_mul(rate).ok_or(DomainError::OutOfRange {
            left: self.0,
            right: rate,
        })?;
        Self::from_decimal(scaled)
    }

    /// The sum of a column of amounts, for example a day's sales or the value
    /// of the batches expiring this month.
    ///
    /// # Errors
    ///
    /// Fails when the total leaves the range a money value can hold. It is
    /// reported rather than dropped, because a wrong total on a receipt is
    /// worse than a screen that says it could not be added up.
    pub fn total(amounts: impl IntoIterator<Item = Self>) -> Result<Self, DomainError> {
        amounts
            .into_iter()
            .try_fold(Self::zero(), Self::checked_add)
    }
}

impl Add for Money {
    type Output = Result<Self, DomainError>;

    /// Falls back to [`Money::checked_add`].
    fn add(self, other: Self) -> Self::Output {
        self.checked_add(other)
    }
}

impl Sub for Money {
    type Output = Result<Self, DomainError>;

    /// Falls back to [`Money::checked_sub`].
    fn sub(self, other: Self) -> Self::Output {
        self.checked_sub(other)
    }
}

impl Neg for Money {
    type Output = Self;

    /// The same amount owed instead of owed back.
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl From<i64> for Money {
    /// Whole shillings, as on the boards.
    fn from(shillings: i64) -> Self {
        Self::from_shillings(shillings)
    }
}

#[cfg(test)]
mod tests {
    use super::{DomainError, Money};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn decimal(text: &str) -> Decimal {
        Decimal::from_str(text).unwrap()
    }

    #[test]
    fn keeps_two_decimal_places_from_the_database() {
        let value = decimal("4120.50");
        assert_eq!(Money::from_decimal(value).unwrap().decimal(), value);
    }

    #[test]
    fn refuses_more_precision_than_the_column_stores() {
        let value = decimal("4120.505");
        assert_eq!(
            Money::from_decimal(value),
            Err(DomainError::TooPrecise { value })
        );
    }

    #[test]
    fn shows_whole_shillings_and_rounds_a_half_away_from_zero() {
        assert_eq!(
            Money::from_decimal(decimal("13700.00"))
                .unwrap()
                .round_to_shillings(),
            13_700
        );
        assert_eq!(
            Money::from_decimal(decimal("2.50"))
                .unwrap()
                .round_to_shillings(),
            3
        );
        assert_eq!(
            Money::from_decimal(decimal("2.49"))
                .unwrap()
                .round_to_shillings(),
            2
        );
        assert_eq!(Money::from_shillings(-3).round_to_shillings(), -3);
    }

    #[test]
    fn adds_and_subtracts_through_the_operators() {
        let total = Money::from_shillings(13_700);
        let insurer = Money::from_shillings(9_590);
        assert_eq!(total - insurer, Ok(Money::from_shillings(4_110)));
        assert_eq!(insurer + Money::from_shillings(4_110), Ok(total));
    }

    #[test]
    fn totals_a_column_the_way_a_dashboard_tile_does() {
        let sales = [
            Money::from_shillings(32_000),
            Money::from_shillings(46_200),
            Money::from_shillings(50_400),
        ];
        assert_eq!(Money::total(sales), Ok(Money::from_shillings(128_600)));
        assert_eq!(Money::total([]), Ok(Money::zero()));
    }

    #[test]
    fn reports_a_refund_as_negative_and_zero_as_neither() {
        assert!(Money::from_shillings(-500).is_negative());
        assert!(Money::zero().is_zero());
        assert!(!Money::zero().is_negative());
    }

    #[test]
    fn scales_by_a_rate_and_refuses_a_lossy_one() {
        let cost = Money::from_decimal(decimal("400.00")).unwrap();
        assert_eq!(
            cost.checked_mul(decimal("1.25")),
            Ok(Money::from_decimal(decimal("500.00")).unwrap())
        );
        // 3.33 * 0.3333 is 1.109889, which the money type cannot hold.
        let odd = Money::from_decimal(decimal("3.33")).unwrap();
        assert!(odd.checked_mul(decimal("0.3333")).is_err());
    }

    #[test]
    fn negating_gives_the_amount_owed_back() {
        assert_eq!(-Money::from_shillings(4_110), Money::from_shillings(-4_110));
    }
}
