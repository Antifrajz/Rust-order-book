use price_key::{AskPrice, BidPrice, PriceKey};
use types::{
    CancelRequest, LevelInfo, ModifyRequest, OrdType, Order, OrderBookSide, OrderFill, OrderFills,
    OrderStatus, Side,
};

pub mod price_key;
pub mod types;

pub struct OrderBook {
    bid: OrderBookSide<BidPrice>,
    ask: OrderBookSide<AskPrice>,
}

// Public struct used as a result for order_book.info
#[derive(PartialEq, Eq, Debug)]
pub struct OrderBookInfo {
    pub bid: Vec<LevelInfo>,
    pub ask: Vec<LevelInfo>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bid: OrderBookSide::new(Side::Buy),
            ask: OrderBookSide::new(Side::Sell),
        }
    }

    pub fn info(self: &Self, depth: Option<usize>) -> OrderBookInfo {
        let d = depth.unwrap_or(5);
        let mut result = OrderBookInfo {
            bid: Vec::new(),
            ask: Vec::new(),
        };
        result.bid = self.bid.get_side_info(d);
        result.ask = self.ask.get_side_info(d);
        result
    }

    pub fn add_order(&mut self, order: &Order) -> Option<OrderFills> {
        match order.get_side() {
            Side::Buy => Self::insert_order(order, &mut self.bid, &mut self.ask),
            Side::Sell => Self::insert_order(order, &mut self.ask, &mut self.bid),
        }
    }

    fn insert_order<T, U>(
        order: &Order,
        side_container: &mut OrderBookSide<T>,
        opposite_side_container: &mut OrderBookSide<U>,
    ) -> Option<OrderFills>
    where
        T: PriceKey,
        U: PriceKey,
    {
        if order.ord_type == OrdType::Market {
            Self::insert_market_order(order, opposite_side_container)
        } else if opposite_side_container.is_order_fillable(order) {
            Self::insert_aggressive_order(order, side_container, opposite_side_container)
        } else {
            Self::insert_passive_order(order, side_container);
            None
        }
    }

    fn insert_passive_order<T>(order: &Order, side_container: &mut OrderBookSide<T>)
    where
        T: PriceKey,
    {
        side_container.insert_order(order);
    }

    fn insert_market_order<T>(
        order: &Order,
        opposite_side_container: &mut OrderBookSide<T>,
    ) -> Option<OrderFills>
    where
        T: PriceKey,
    {
        if opposite_side_container.is_order_fillable(order) {
            Some(Self::do_fills(order, opposite_side_container))
        } else {
            None
        }
    }
    fn insert_aggressive_order<T, U>(
        order: &Order,
        side_container: &mut OrderBookSide<T>,
        opposite_side_container: &mut OrderBookSide<U>,
    ) -> Option<OrderFills>
    where
        T: PriceKey,
        U: PriceKey,
    {
        let order_fills = Self::do_fills(order, opposite_side_container);
        if order_fills.remaining_quantity != 0 {
            let new_order = Order {
                quantity: order_fills.remaining_quantity,
                price: order.price,
                order_id: order.order_id,
                side: order.get_side(),
                ord_type: types::OrdType::Limit,
                order_status: OrderStatus::PartiallyFilled,
            };
            side_container.insert_order(&new_order);
            //provejrit ovo da se muva kako :D
        }

        Some(order_fills)
    }

    pub fn cancel_order<T: Clone>(
        cancel_request: &CancelRequest,
        side_container: &mut OrderBookSide<T>,
    ) -> bool
    where
        T: PriceKey,
    {
        let id_to_remove = cancel_request.id;
        let mut key_to_remove: Option<T> = None;
        let mut result: bool = false;

        for (key, level) in &mut side_container.levels {
            if let Some(pos) = level
                .orders
                .iter()
                .position(|&order| order.order_id == id_to_remove)
            {
                level.orders.remove(pos);

                if level.orders.is_empty() {
                    key_to_remove = Some(key.clone());
                }
                result = true;
                break;
            }
        }

        if let Some(key) = key_to_remove {
            side_container.levels.remove(&key);
        }

        return result;
    }

    pub fn cancel(&mut self, cancel_request: &CancelRequest) -> bool {
        match cancel_request.side {
            Side::Buy => OrderBook::cancel_order(cancel_request, &mut self.bid),
            Side::Sell => OrderBook::cancel_order(cancel_request, &mut self.ask),
        }
    }

    pub fn modify(&mut self, modify_request: &ModifyRequest) -> (bool, Option<OrderFills>) {
        match modify_request.side {
            Side::Buy => Self::modify_order(modify_request, &mut self.bid, &mut self.ask),
            Side::Sell => Self::modify_order(modify_request, &mut self.ask, &mut self.bid),
        }
    }

    fn modify_order<T, U>(
        modify_request: &ModifyRequest,
        side_container: &mut OrderBookSide<T>,
        opposite_side_container: &mut OrderBookSide<U>,
    ) -> (bool, Option<OrderFills>)
    where
        T: Clone + PriceKey,
        U: PriceKey,
    {
        let mut new_order: Option<Order> = None;
        let mut key_to_remove: Option<T> = None;
        for (key, level) in &mut side_container.levels {
            if let Some(pos) = level
                .orders
                .iter()
                .position(|&order| order.order_id == modify_request.id)
            {
                new_order = Some(level.orders[pos].clone());
                level.orders.remove(pos);

                if level.orders.is_empty() && new_order.unwrap().price != modify_request.price {
                    key_to_remove = Some(key.clone());
                }
            }
        }
        if let Some(key) = key_to_remove {
            side_container.levels.remove(&key);
        }

        if let Some(mut modified_order) = new_order {
            modified_order.price = modify_request.price;
            modified_order.quantity = modify_request.quantity;
            modified_order.ord_type = modify_request.ord_type;
            return (
                true,
                Self::insert_order(&modified_order, side_container, opposite_side_container),
            );
        } else {
            return (false, None);
        }
    }

    fn do_fills<T>(order: &Order, opposite_side_container: &mut OrderBookSide<T>) -> OrderFills
    where
        T: PriceKey,
    {
        let mut order_fills = OrderFills {
            fills: Vec::new(),
            executed_quantity: 0,
            remaining_quantity: order.get_quantity(),
        };
        while let Some(mut first_enetry) = opposite_side_container.levels.first_entry() {
            let price = order
                .price
                .or_else(|| Some(first_enetry.key().price().clone()))
                .unwrap();
            if order_fills.remaining_quantity == 0 || !first_enetry.key().fill_possible(price) {
                break;
            }

            while let Some(first_order) = first_enetry.get_mut().orders.front_mut() {
                if order_fills.remaining_quantity == 0 {
                    break;
                }
                if order_fills.remaining_quantity >= first_order.get_quantity() {
                    order_fills.fills.push(OrderFill {
                        id: first_order.order_id,
                        quantity: first_order.quantity,
                        price: first_order.price.unwrap(),
                    });
                    order_fills.executed_quantity += first_order.quantity;
                    order_fills.remaining_quantity -= first_order.quantity;
                    dbg!(&first_order);
                    first_enetry.get_mut().orders.pop_front();
                } else {
                    order_fills.fills.push(OrderFill {
                        id: first_order.order_id,
                        quantity: order_fills.remaining_quantity,
                        price: first_order.price.unwrap(),
                    });
                    dbg!(&first_order);
                    order_fills.executed_quantity += order_fills.remaining_quantity;
                    first_order.quantity -= order_fills.remaining_quantity;
                    first_order.order_status = OrderStatus::PartiallyFilled;
                    order_fills.remaining_quantity = 0;
                }
            }
            if first_enetry.get_mut().orders.is_empty() {
                opposite_side_container.levels.pop_first();
            }
        }

        order_fills
    }
}
