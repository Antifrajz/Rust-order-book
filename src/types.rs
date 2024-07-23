use std::collections::{BTreeMap, VecDeque};

use rust_decimal::prelude::*;

use crate::price_key::PriceKey;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OrdType {
    Market,
    Limit,
}

#[derive(Copy, Clone, Debug)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Canceled,
}
#[derive(Copy, Clone, Debug)]
pub struct Order {
    pub order_id: u64,
    pub quantity: u64,
    pub side: Side,
    pub order_status: OrderStatus,
    pub price: Option<Decimal>,
    pub ord_type: OrdType,
}

// Public struct used as an entry for oder book info
#[derive(PartialEq, Eq, Debug)]
pub struct LevelInfo {
    pub count: u64,
    pub quantity: u64,
    pub price: Decimal,
}

pub struct OrderBookSide<T> {
    side: Side,
    pub levels: BTreeMap<T, Level>,
}

pub struct OrderFill {
    pub id: u64,
    pub quantity: u64,
    pub price: Decimal,
}

pub struct OrderFills {
    pub fills: Vec<OrderFill>,
    pub executed_quantity: u64,
    pub remaining_quantity: u64,
}

pub struct Level {
    pub orders: VecDeque<Order>,
    pub price_point: Decimal,
}

#[derive(PartialEq, Eq, Debug)]
pub struct CancelRequest {
    pub id: u64,
    pub side: Side,
}

#[derive(Copy, Clone, Debug)]
pub struct ModifyRequest {
    pub id: u64,
    pub side: Side,
    pub price: Option<Decimal>,
    pub quantity: u64,
    pub ord_type: OrdType,
}

impl<T> OrderBookSide<T>
where
    T: PriceKey,
{
    pub fn new(side: Side) -> Self {
        OrderBookSide {
            side: side,
            levels: BTreeMap::new(),
        }
    }

    pub fn insert_order(&mut self, order: &Order) {
        let key = T::from_price(order.price.unwrap());
        let entry = self.levels.get_mut(&key);
        match entry {
            Some(level) => level.orders.push_back(order.clone()),
            None => {
                let mut level = Level {
                    price_point: order.price.unwrap(),
                    orders: VecDeque::new(),
                };
                level.orders.push_back(order.clone());
                self.levels.insert(key, level);
            }
        }
    }

    pub fn is_order_fillable(&mut self, order: &Order) -> bool {
        if let Some(first_entry) = self.levels.first_entry() {
            if let Some(price) = order.price {
                first_entry.key().fill_possible(price)
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn get_side_info(&self, depth: usize) -> Vec<LevelInfo> {
        let mut result = Vec::new();
        for (count, (_key, value)) in self.levels.iter().enumerate() {
            if depth == count {
                break;
            }
            result.push(value.info());
        }
        result
    }
}

impl Order {
    pub fn get_side(&self) -> Side {
        self.side
    }

    pub fn get_quantity(&self) -> u64 {
        self.quantity
    }
}

impl Level {
    pub fn info(&self) -> LevelInfo {
        let total_qty = self.orders.iter().fold(0, |acc, el| acc + el.quantity);
        LevelInfo {
            count: self.orders.iter().count().to_u64().unwrap(),
            quantity: total_qty,
            price: self.price_point,
        }
    }
}
