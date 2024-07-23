use rust_decimal::prelude::*;

pub trait PriceKey
where
    Self: Ord + core::fmt::Debug,
{
    fn from_price(price: Decimal) -> Self;
    fn fill_possible(&self, price: Decimal) -> bool;
    fn price(&self) -> Decimal;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct AskPrice {
    price: Decimal,
}

impl PriceKey for AskPrice {
    fn from_price(price: Decimal) -> Self {
        return AskPrice { price };
    }

    fn fill_possible(&self, price: Decimal) -> bool {
        self.price <= price || price.is_zero()
    }

    fn price(&self) -> Decimal {
        self.price
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct BidPrice {
    price: Decimal,
}

impl PriceKey for BidPrice {
    fn from_price(price: Decimal) -> Self {
        return BidPrice { price };
    }

    fn fill_possible(&self, price: Decimal) -> bool {
        self.price >= price || price.is_zero()
    }

    fn price(&self) -> Decimal {
        self.price
    }
}

impl PartialOrd for BidPrice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.price.cmp(&self.price))
    }
}

impl Ord for BidPrice {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.price.cmp(&self.price)
    }
}
