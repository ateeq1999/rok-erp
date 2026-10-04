//! Why a value could not be built or combined.

use rust_decimal::Decimal;
use thiserror::Error;

/// A value the domain refuses to accept.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DomainError {
    /// A money value carried more than the two decimal places `numeric(18,2)` stores.
    #[error("{value} has more precision than two decimal places")]
    TooPrecise {
        /// The value that was rejected.
        value: Decimal,
    },
    /// An operation left the range a money value can hold.
    #[error("{left} and {right} cannot be combined")]
    OutOfRange {
        /// The value on the left of the operation.
        left: Decimal,
        /// The value on the right of the operation.
        right: Decimal,
    },
}
