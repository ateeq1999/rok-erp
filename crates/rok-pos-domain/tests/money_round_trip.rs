//! Phase 0 spike 1: `Money` survives a round trip through a `numeric(18,2)`
//! column, as plan.md 0.7 asks.

use rok_db::{Db, raw};
use rok_pos_domain::Money;
use rust_decimal::Decimal;
use uuid::Uuid;

#[rok_db::test(sql = "tests/sales_schema.sql")]
async fn money_goes_into_the_column_and_comes_back_the_same(db: Db) {
    let amount = Money::from_shillings(13_700);

    let inserted = raw("insert into spike.sales (id, reference, amount) values (?, ?, ?)")
        .bind(Uuid::now_v7())
        .bind("RX-2210")
        .bind(amount)
        .execute(&db)
        .await
        .expect("a Money binds to a numeric column");
    assert_eq!(inserted, 1);

    let read: Decimal = raw("select amount from spike.sales where reference = ?")
        .bind("RX-2210")
        .scalar(&db)
        .await
        .expect("the amount reads back as a Decimal");
    assert_eq!(read, amount.decimal());
    assert_eq!(Money::from_decimal(read).unwrap(), amount);
}

#[rok_db::test(sql = "tests/sales_schema.sql")]
async fn a_money_column_keeps_its_two_decimal_places(db: Db) {
    let amount = Money::from_decimal(Decimal::new(4_120, 2)).unwrap();

    raw("insert into spike.sales (id, reference, amount) values (?, ?, ?)")
        .bind(Uuid::now_v7())
        .bind("RX-2211")
        .bind(amount)
        .execute(&db)
        .await
        .expect("a Money with cents binds");

    let read: Decimal = raw("select amount from spike.sales where reference = ?")
        .bind("RX-2211")
        .scalar(&db)
        .await
        .expect("the amount reads back");
    assert_eq!(
        read,
        Decimal::new(4_120, 2),
        "the column keeps the scale, and does not round it away"
    );
    assert_eq!(Money::from_decimal(read).unwrap(), amount);
}

#[rok_db::test(sql = "tests/sales_schema.sql")]
async fn an_absent_amount_is_a_null_and_not_a_zero(db: Db) {
    let inserted = raw(
        "insert into spike.sales (id, reference, amount, insurer_share) \
         values (?, ?, ?, ?)",
    )
    .bind(Uuid::now_v7())
    .bind("RX-2212")
    .bind(Money::from_shillings(13_700))
    .bind(None::<Money>)
    .execute(&db)
    .await
    .expect("an absent Money binds a NULL");
    assert_eq!(inserted, 1);

    let stored: Option<Decimal> = raw("select insurer_share from spike.sales where reference = ?")
        .bind("RX-2212")
        .scalar(&db)
        .await
        .expect("the row reads back");
    assert_eq!(stored, None, "no share is stored, rather than a zero");
}
