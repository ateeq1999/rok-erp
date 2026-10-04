//! How [`Money`] is stored in a `numeric(18,2)` column.
//!
//! The database holds money as `numeric(18,2)`, so the plumbing here is
//! delegation: [`sqlx::Encode`] writes the [`Decimal`] inside, [`sqlx::Decode`]
//! hands the column to [`Money::from_decimal`], and the SQL type is `NUMERIC`.
//! [`rok_db::impl_value!`] then makes `Money` a column value, so it binds in a
//! filter, a `set` and a `?` placeholder, and reads back from a row.
//!
//! The two decimal places are kept rather than rounded away: a value the column
//! cannot hold is reported, not quietly trimmed.
//!
//! This lives beside [`Money`] rather than in `rok-pos-database` because sqlx's
//! traits are foreign to this workspace, and only the crate that owns the type
//! may implement them.

use rust_decimal::Decimal;
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};

use crate::Money;

impl Type<Postgres> for Money {
    fn type_info() -> PgTypeInfo {
        <Decimal as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <Decimal as Type<Postgres>>::compatible(ty)
    }
}

impl Encode<'_, Postgres> for Money {
    fn encode_by_ref(&self, buffer: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <Decimal as Encode<'_, Postgres>>::encode_by_ref(&self.decimal(), buffer)
    }
}

impl<'r> Decode<'r, Postgres> for Money {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        <Decimal as Decode<'r, Postgres>>::decode(value)
            .and_then(|decimal| Money::from_decimal(decimal).map_err(|error| Box::new(error) as _))
    }
}

rok_db::impl_value!(Money);
