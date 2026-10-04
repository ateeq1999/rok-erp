//! The two column types the compliance tables share that are not money.
//!
//! A temperature and a quantity are `numeric` columns, and a bare `Decimal`
//! cannot be a [`rok_db`] column value in a crate that does not own it: only
//! the type's own crate may implement sqlx's traits for it. A newtype is the
//! escape hatch, stored exactly like the `Decimal` inside it, so the column
//! keeps every decimal place the pharmacy recorded.

use rok_db::DbNewtype;
use rust_decimal::Decimal;

/// A temperature in degrees Celsius, from a `numeric(5,2)` column: the fridge
/// logs and the cold-chain check on a delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbNewtype)]
pub struct Temperature(Decimal);

impl Temperature {
    /// The reading as stored, for comparisons and for the in-range rule.
    #[must_use]
    pub const fn decimal(self) -> Decimal {
        self.0
    }

    /// A reading taken at the bench or out of the fridge.
    #[must_use]
    pub const fn new(value: Decimal) -> Self {
        Self(value)
    }
}

/// A quantity in units, from a `numeric(18,3)` column: how many bottles a
/// recall took out of the quarantine box, say, where a part of a unit is
/// meaningful for weights and volumes but the count itself is whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, DbNewtype)]
pub struct Quantity(Decimal);

impl Quantity {
    /// The amount as stored.
    #[must_use]
    pub const fn decimal(self) -> Decimal {
        self.0
    }

    /// A counted amount.
    #[must_use]
    pub const fn new(value: Decimal) -> Self {
        Self(value)
    }
}
