use crate::model::{Price, SIDE_ASK, SIDE_BID, Side};

pub const INTERNAL_BID_SENTINEL: Price = 0;
pub const INTERNAL_ASK_SENTINEL: Price = Price::MAX;

pub const DATABENTO_SENTINEL: i64 = i64::MAX;
pub const DATABENTO_PRICE_DIVISOR: Price = 10_000_000;

pub fn from_databento_to_price(side: Side, databento_price: &i64) -> Price {
    if databento_price == &DATABENTO_SENTINEL {
        return match side {
            SIDE_BID => INTERNAL_BID_SENTINEL,
            SIDE_ASK => INTERNAL_ASK_SENTINEL,
            _ => unreachable!(),
        };
    }

    databento_price / DATABENTO_PRICE_DIVISOR
}
