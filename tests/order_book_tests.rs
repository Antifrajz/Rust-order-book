use order_book;
use order_book::{types::*, OrderBook};
use rand::Rng;
use rust_decimal::prelude::*;
#[test]
fn test_adding_passive_orders() {
    let mut order_book = OrderBook::new();
    let _4995 = Decimal::from_str("49.95").unwrap();
    let _4996 = Decimal::from_str("49.96").unwrap();
    let _4998 = Decimal::from_str("49.98").unwrap();
    let _5005 = Decimal::from_str("50.05").unwrap();
    let _5003 = Decimal::from_str("50.03").unwrap();
    let _5002 = Decimal::from_str("50.02").unwrap();

    // Adding first order

    let order = Order {
        price: Some(_4995),
        quantity: 3000,
        order_id: 0,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |            Ask
    // (1) 3000@49.95      |  None
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 1);
    assert_eq!(book_info.ask.iter().count(), 0);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );

    // Next order

    let order = Order {
        price: Some(_5005),
        quantity: 2700,
        order_id: 1,
        side: Side::Sell,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (1) 3000@49.95      |  (1) 2700@50.05
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 1);
    assert_eq!(book_info.ask.iter().count(), 1);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );

    // Next order

    let order = Order {
        price: Some(_4998),
        quantity: 5500,
        order_id: 2,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (1) 5500@49.98      |  (1) 2700@50.05
    // (1) 3000@49.95      |
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 2);
    assert_eq!(book_info.ask.iter().count(), 1);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 1,
            price: _4998,
            quantity: 5500
        }
    );
    assert_eq!(
        book_info.bid[1],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );

    // Next order

    let order = Order {
        price: Some(_5002),
        quantity: 1300,
        order_id: 3,
        side: Side::Sell,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (1) 5500@49.98      |  (1) 1300@50.02
    // (1) 3000@49.95      |  (1) 2700@50.05
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 2);
    assert_eq!(book_info.ask.iter().count(), 2);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 1,
            price: _4998,
            quantity: 5500
        }
    );
    assert_eq!(
        book_info.bid[1],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5002,
            quantity: 1300
        }
    );
    assert_eq!(
        book_info.ask[1],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );

    // Next order

    let order = Order {
        price: Some(_5003),
        quantity: 3900,
        order_id: 4,
        side: Side::Sell,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (1) 5500@49.98      |  (1) 1300@50.02
    // (1) 3000@49.95      |  (1) 3900@50.03
    //                     |  (1) 2700@50.05
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 2);
    assert_eq!(book_info.ask.iter().count(), 3);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 1,
            price: _4998,
            quantity: 5500
        }
    );
    assert_eq!(
        book_info.bid[1],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5002,
            quantity: 1300
        }
    );
    assert_eq!(
        book_info.ask[1],
        LevelInfo {
            count: 1,
            price: _5003,
            quantity: 3900
        }
    );
    assert_eq!(
        book_info.ask[2],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );

    // Next order

    let order = Order {
        price: Some(_4996),
        quantity: 2000,
        order_id: 5,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (1) 5500@49.98      |  (1) 1300@50.02
    // (1) 2000@49.96      |  (1) 3900@50.03
    // (1) 3000@49.95      |  (1) 2700@50.05
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 3);
    assert_eq!(book_info.ask.iter().count(), 3);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 1,
            price: _4998,
            quantity: 5500
        }
    );
    assert_eq!(
        book_info.bid[1],
        LevelInfo {
            count: 1,
            price: _4996,
            quantity: 2000
        }
    );
    assert_eq!(
        book_info.bid[2],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5002,
            quantity: 1300
        }
    );
    assert_eq!(
        book_info.ask[1],
        LevelInfo {
            count: 1,
            price: _5003,
            quantity: 3900
        }
    );
    assert_eq!(
        book_info.ask[2],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );

    // Next order

    let order = Order {
        price: Some(_4998),
        quantity: 2700,
        order_id: 6,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (2) 8200@49.98      |  (1) 1300@50.02
    // (1) 2000@49.96      |  (1) 3900@50.03
    // (1) 3000@49.95      |  (1) 2700@50.05
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 3);
    assert_eq!(book_info.ask.iter().count(), 3);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 2,
            price: _4998,
            quantity: 8200
        }
    );
    assert_eq!(
        book_info.bid[1],
        LevelInfo {
            count: 1,
            price: _4996,
            quantity: 2000
        }
    );
    assert_eq!(
        book_info.bid[2],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5002,
            quantity: 1300
        }
    );
    assert_eq!(
        book_info.ask[1],
        LevelInfo {
            count: 1,
            price: _5003,
            quantity: 3900
        }
    );
    assert_eq!(
        book_info.ask[2],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );

    // Next order

    let order = Order {
        price: Some(_5003),
        quantity: 4800,
        order_id: 7,
        side: Side::Sell,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result = order_book.add_order(&order);
    //       Bid           |        Ask
    // (2) 8200@49.98      |  (1) 1300@50.02
    // (1) 2000@49.96      |  (2) 8700@50.03
    // (1) 3000@49.95      |  (1) 2700@50.05
    //
    assert!(add_result.is_none());
    let book_info = order_book.info(None);
    assert_eq!(book_info.bid.iter().count(), 3);
    assert_eq!(book_info.ask.iter().count(), 3);
    assert_eq!(
        book_info.bid[0],
        LevelInfo {
            count: 2,
            price: _4998,
            quantity: 8200
        }
    );
    assert_eq!(
        book_info.bid[1],
        LevelInfo {
            count: 1,
            price: _4996,
            quantity: 2000
        }
    );
    assert_eq!(
        book_info.bid[2],
        LevelInfo {
            count: 1,
            price: _4995,
            quantity: 3000
        }
    );
    assert_eq!(
        book_info.ask[0],
        LevelInfo {
            count: 1,
            price: _5002,
            quantity: 1300
        }
    );
    assert_eq!(
        book_info.ask[1],
        LevelInfo {
            count: 2,
            price: _5003,
            quantity: 8700
        }
    );
    assert_eq!(
        book_info.ask[2],
        LevelInfo {
            count: 1,
            price: _5005,
            quantity: 2700
        }
    );
}

// Function to generate randomized orders
fn generate_orders(
    side: Side,
    id_start: u64,
    num_orders: u64,
    total_quantity: u64,
    price: Decimal,
) -> Vec<Order> {
    let mut orders = vec![];
    let mut remaining_quantity = total_quantity;

    for i in 0..num_orders {
        let id = id_start + i;
        let quantity = if i == num_orders - 1 {
            remaining_quantity
        } else {
            let max_quantity = (remaining_quantity - (num_orders - i - 1) * 100) / 100;
            let min_quantity = 1;
            let quantity = rand::thread_rng().gen_range(min_quantity..max_quantity + 1) * 100;
            remaining_quantity -= quantity;
            quantity
        };

        let order = Order {
            quantity,
            price: Some(price.clone()),
            order_id: id,
            side: side.clone(),
            order_status: OrderStatus::New,
            ord_type: OrdType::Limit,
        };
        orders.push(order);
    }

    orders
}

//       Bid           |        Ask
// (7) 36200@49.98     |  (4) 6900@50.02
// (8) 39000@49.96     |  (6) 10100@50.03
// (11) 56300@49.95    |  (8) 11800@50.05
//
fn create_order_book() -> (Vec<Order>, Vec<Order>, OrderBook) {
    let mut result = OrderBook::new();
    let mut add_orders = |orders: &Vec<Order>| {
        for order in orders {
            result.add_order(&order);
        }
    };
    let mut bid_orders = Vec::new();
    let orders_4998 = generate_orders(Side::Buy, 1, 7, 36200, Decimal::new(4998, 2));
    add_orders(&orders_4998);
    bid_orders.extend(orders_4998);
    let orders_4996 = generate_orders(Side::Buy, 8, 8, 39000, Decimal::new(4996, 2));
    add_orders(&orders_4996);
    bid_orders.extend(orders_4996);
    let orders_4995 = generate_orders(Side::Buy, 16, 11, 56300, Decimal::new(4995, 2));
    add_orders(&orders_4995);
    bid_orders.extend(orders_4995);
    let mut ask_orders = Vec::new();
    let orders_5002 = generate_orders(Side::Sell, 27, 4, 6900, Decimal::new(5002, 2));
    add_orders(&orders_5002);
    ask_orders.extend(orders_5002);
    let orders_5003 = generate_orders(Side::Sell, 31, 6, 10100, Decimal::new(5003, 2));
    add_orders(&orders_5003);
    ask_orders.extend(orders_5003);
    let orders_5005 = generate_orders(Side::Sell, 37, 8, 11800, Decimal::new(5005, 2));
    add_orders(&orders_5005);
    ask_orders.extend(orders_5005);

    (bid_orders, ask_orders, result)
}

#[test]
fn test_adding_aggressive_orders() {
    let (bid_orders, ask_orders, mut book) = create_order_book();

    // Execute half of first ask order
    let book_info0 = book.info(None);
    assert_eq!(book_info0.bid.iter().count(), 3);
    assert_eq!(book_info0.ask.iter().count(), 3);
    let half_qty = ask_orders[0].quantity / 2;
    let aggressive_order = Order {
        quantity: half_qty,
        price: Some(Decimal::new(5002, 2)),
        order_id: 46,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result_opt = book.add_order(&aggressive_order);
    assert!(add_result_opt.is_some());
    let add_result = add_result_opt.unwrap();
    assert_eq!(add_result.fills.iter().count(), 1);
    assert_eq!(add_result.fills[0].price, ask_orders[0].price.unwrap());
    assert_eq!(add_result.fills[0].id, ask_orders[0].order_id);
    assert_eq!(add_result.fills[0].quantity, half_qty);
    assert_eq!(add_result.executed_quantity, half_qty);
    assert_eq!(add_result.remaining_quantity, 0);
    let book_info1 = book.info(None);
    assert_eq!(book_info0.bid.iter().count(), 3);
    assert_eq!(book_info0.ask.iter().count(), 3);
    assert_eq!(book_info0.bid[0], book_info1.bid[0]);
    assert_eq!(book_info0.bid[1], book_info1.bid[1]);
    assert_eq!(book_info0.bid[2], book_info1.bid[2]);
    assert_eq!(book_info0.ask[2], book_info1.ask[2]);
    assert_eq!(book_info0.ask[1], book_info1.ask[1]);
    assert_eq!(book_info0.ask[0].price, book_info1.ask[0].price);
    assert_eq!(book_info0.ask[0].count, book_info1.ask[0].count);
    assert_eq!(
        book_info0.ask[0].quantity - half_qty,
        book_info1.ask[0].quantity,
    );

    // Execute other half of first ask order
    let aggressive_order = Order {
        quantity: half_qty,
        price: Some(Decimal::new(5002, 2)),
        order_id: 47,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result_opt = book.add_order(&aggressive_order);
    assert!(add_result_opt.is_some());
    let add_result = add_result_opt.unwrap();
    assert_eq!(add_result.fills.iter().count(), 1);
    assert_eq!(add_result.fills[0].price, ask_orders[0].price.unwrap());
    assert_eq!(add_result.fills[0].id, ask_orders[0].order_id);
    assert_eq!(add_result.fills[0].quantity, half_qty);

    assert_eq!(add_result.executed_quantity, half_qty);
    assert_eq!(add_result.remaining_quantity, 0);
    let book_info2 = book.info(None);
    assert_eq!(book_info2.bid.iter().count(), 3);
    assert_eq!(book_info2.ask.iter().count(), 3);
    assert_eq!(book_info1.bid[0], book_info2.bid[0]);
    assert_eq!(book_info1.bid[1], book_info2.bid[1]);
    assert_eq!(book_info1.bid[2], book_info2.bid[2]);
    assert_eq!(book_info1.ask[2], book_info2.ask[2]);
    assert_eq!(book_info1.ask[1], book_info2.ask[1]);
    assert_eq!(book_info1.ask[0].price, book_info2.ask[0].price);
    assert_eq!(book_info1.ask[0].count - 1, book_info2.ask[0].count);
    assert_eq!(
        book_info1.ask[0].quantity - half_qty,
        book_info2.ask[0].quantity
    );

    // Execute entire first level of ask side and create new order on bid side
    let aggressive_order = Order {
        quantity: book_info2.ask[0].quantity + 1000,
        price: Some(Decimal::new(5002, 2)),
        order_id: 48,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result_opt = book.add_order(&aggressive_order);
    assert!(add_result_opt.is_some());
    let add_result = add_result_opt.unwrap();
    assert_eq!(add_result.fills.iter().count(), 3);
    for i in 0..3 {
        assert_eq!(add_result.fills[i].price, ask_orders[i + 1].price.unwrap());
        assert_eq!(add_result.fills[i].id, ask_orders[i + 1].order_id);
        assert_eq!(add_result.fills[i].quantity, ask_orders[i + 1].quantity);
    }
    assert_eq!(add_result.remaining_quantity, 1000);
    assert_eq!(add_result.executed_quantity, book_info2.ask[0].quantity);
    let book_info3 = book.info(None);
    assert_eq!(book_info3.bid.iter().count(), 4);
    assert_eq!(book_info3.ask.iter().count(), 2);
    assert_eq!(
        LevelInfo {
            count: 1,
            quantity: 1000,
            price: Decimal::new(5002, 2)
        },
        book_info3.bid[0]
    );
    assert_eq!(book_info2.bid[0], book_info3.bid[1]);
    assert_eq!(book_info2.bid[1], book_info3.bid[2]);
    assert_eq!(book_info2.bid[2], book_info3.bid[3]);
    assert_eq!(book_info2.ask[2], book_info3.ask[1]);
    assert_eq!(book_info2.ask[1], book_info3.ask[0]);

    // Execute entire second level (after last aggressive order it is now first level) with price
    // 50.04. Expecting the fills to contain original prices of orders (50.03) and leave again,
    //    something on the book.
    let aggressive_order = Order {
        quantity: book_info3.ask[0].quantity + 500,
        price: Some(Decimal::new(5004, 2)),
        order_id: 49,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result_opt = book.add_order(&aggressive_order);
    assert!(add_result_opt.is_some());
    let add_result = add_result_opt.unwrap();
    assert_eq!(add_result.fills.iter().count(), 6);
    for i in 0..6 {
        assert_eq!(add_result.fills[i].price, ask_orders[i + 4].price.unwrap());
        assert_eq!(add_result.fills[i].id, ask_orders[i + 4].order_id);
        assert_eq!(add_result.fills[i].quantity, ask_orders[i + 4].quantity);
    }

    assert_eq!(add_result.remaining_quantity, 500);
    assert_eq!(add_result.executed_quantity, book_info3.ask[0].quantity);
    let book_info4 = book.info(None);
    assert_eq!(book_info4.bid.iter().count(), 5);
    assert_eq!(book_info4.ask.iter().count(), 1);
    assert_eq!(
        LevelInfo {
            count: 1,
            quantity: 500,
            price: Decimal::new(5004, 2)
        },
        book_info4.bid[0]
    );
    assert_eq!(book_info3.bid[0], book_info4.bid[1]);
    assert_eq!(book_info3.bid[1], book_info4.bid[2]);
    assert_eq!(book_info3.bid[2], book_info4.bid[3]);
    assert_eq!(book_info3.bid[3], book_info4.bid[4]);
    assert_eq!(book_info3.ask[1], book_info4.ask[0]);

    // Test complete execution of ask side and leave something on the book
    let aggressive_order = Order {
        quantity: book_info4.ask[0].quantity + 100,
        price: Some(Decimal::new(5050, 2)),
        order_id: 50,
        side: Side::Buy,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result_opt = book.add_order(&aggressive_order);
    assert!(add_result_opt.is_some());
    let add_result = add_result_opt.unwrap();
    assert_eq!(add_result.fills.iter().count(), 8);
    for i in 0..8 {
        assert_eq!(add_result.fills[i].price, ask_orders[i + 10].price.unwrap());
        assert_eq!(add_result.fills[i].id, ask_orders[i + 10].order_id);
        assert_eq!(add_result.fills[i].quantity, ask_orders[i + 10].quantity);
    }
    assert_eq!(add_result.remaining_quantity, 100);
    assert_eq!(add_result.executed_quantity, book_info4.ask[0].quantity);
    let book_info5 = book.info(Some(6));
    assert_eq!(book_info5.bid.iter().count(), 6);
    assert!(book_info5.ask.is_empty());
    assert_eq!(
        LevelInfo {
            count: 1,
            quantity: 100,
            price: Decimal::new(5050, 2)
        },
        book_info5.bid[0]
    );
    assert_eq!(book_info4.bid[0], book_info5.bid[1]);
    assert_eq!(book_info4.bid[1], book_info5.bid[2]);
    assert_eq!(book_info4.bid[2], book_info5.bid[3]);
    assert_eq!(book_info4.bid[3], book_info5.bid[4]);
    assert_eq!(book_info4.bid[4], book_info5.bid[5]);

    // Test complete execution of bid side and leave something on the book
    let total_bid_qty = book_info5.bid.iter().fold(0, |x, el| x + el.quantity);
    let aggressive_order = Order {
        quantity: total_bid_qty + 700,
        price: Some(Decimal::new(4500, 2)),
        order_id: 51,
        side: Side::Sell,
        order_status: OrderStatus::New,
        ord_type: OrdType::Limit,
    };
    let add_result_opt = book.add_order(&aggressive_order);
    assert!(add_result_opt.is_some());
    let add_result = add_result_opt.unwrap();
    assert_eq!(add_result.fills.iter().count(), 29);
    assert_eq!(add_result.fills[0].price, Decimal::new(5050, 2));
    assert_eq!(add_result.fills[0].id, 50);
    assert_eq!(add_result.fills[0].quantity, 100);
    assert_eq!(add_result.fills[1].price, Decimal::new(5004, 2));
    assert_eq!(add_result.fills[1].id, 49);
    assert_eq!(add_result.fills[1].quantity, 500);
    assert_eq!(add_result.fills[2].price, Decimal::new(5002, 2));
    assert_eq!(add_result.fills[2].id, 48);
    assert_eq!(add_result.fills[2].quantity, 1000);
    for i in 0..26 {
        assert_eq!(add_result.fills[i + 3].price, bid_orders[i].price.unwrap());
        assert_eq!(add_result.fills[i + 3].id, bid_orders[i].order_id);
        assert_eq!(add_result.fills[i + 3].quantity, bid_orders[i].quantity);
    }
    assert_eq!(add_result.remaining_quantity, 700);
    assert_eq!(add_result.executed_quantity, total_bid_qty);
    let book_info6 = book.info(None);
    assert!(book_info6.bid.is_empty());
    assert_eq!(book_info6.ask.iter().count(), 1);
    assert_eq!(book_info6.ask[0].quantity, 700);
    assert_eq!(book_info6.ask[0].price, aggressive_order.price.unwrap());
    assert_eq!(book_info6.ask[0].count, 1);
}

#[test]
fn test_cancelling_orders() {
    let (bid_orders, ask_orders, mut book) = create_order_book();

    // test that cancel request will reduce level's quantity and number of orders
    let book_info = book.info(None);
    let bid_level1_orig_qty = book_info.bid[0].quantity;
    let bid_level1_orig_orders_no = book_info.bid[0].count;
    let bid_order1_qty = bid_orders[0].quantity;

    let cancel_request = CancelRequest {
        id: bid_orders[0].order_id,
        side: Side::Buy,
    };

    assert!(book.cancel(&cancel_request));

    let book_info = book.info(None);
    let bid_level1_new_qty = book_info.bid[0].quantity;
    let bid_level1_new_orders_no = book_info.bid[0].count;

    assert_eq!(bid_level1_new_qty, bid_level1_orig_qty - bid_order1_qty);
    assert_eq!(bid_level1_new_orders_no, bid_level1_orig_orders_no - 1);

    // test that multiple cancel requests will remove level
    for i in 0..4 {
        let cancel_request = CancelRequest {
            id: ask_orders[i].order_id,
            side: Side::Sell,
        };
        assert!(book.cancel(&cancel_request));
    }

    let book_info = book.info(None);
    assert_eq!(book_info.ask.len(), 2);
    assert_eq!(book_info.bid.len(), 3);

    assert_eq!(book_info.ask[0].price, Decimal::new(5003, 2));
    assert_eq!(book_info.ask[1].price, Decimal::new(5005, 2));

    let book_info0 = book.info(None);

    // test that cancel request with non existing id will fail
    let cancel_request = CancelRequest {
        id: 999,
        side: Side::Sell,
    };
    assert!(!book.cancel(&cancel_request));
    let book_info1 = book.info(None);
    assert_eq!(book_info0, book_info1);

    // test that cancel request with wrong side will fail
    let cancel_request = CancelRequest {
        id: ask_orders[4].order_id,
        side: Side::Buy,
    };
    assert!(!book.cancel(&cancel_request));
    let book_info3 = book.info(None);
    assert_eq!(book_info0, book_info3);

    // test clearing book by sending cancel requests
    for i in 1..26 {
        let cancel_request = CancelRequest {
            id: bid_orders[i].order_id,
            side: Side::Buy,
        };
        assert!(book.cancel(&cancel_request));
    }

    for i in 4..18 {
        let cancel_request = CancelRequest {
            id: ask_orders[i].order_id,
            side: Side::Sell,
        };
        assert!(book.cancel(&cancel_request));
    }

    let probably_empty_book_info = book.info(None);
    assert!(probably_empty_book_info.bid.is_empty());
    assert!(probably_empty_book_info.ask.is_empty());

    // test cancel request on empty book
    let cancel_request = CancelRequest {
        id: bid_orders[2].order_id,
        side: Side::Buy,
    };
    assert!(!book.cancel(&cancel_request));

    let probably_empty_book_info2 = book.info(None);
    assert_eq!(probably_empty_book_info, probably_empty_book_info2);
}

#[test]
fn test_modifying_orders() {
    let (bid_orders, ask_orders, mut book) = create_order_book();

    // test that modify request will icrease level's quantity and won't change number of orders
    let book_info = book.info(None);
    let bid_level1_orig_qty = book_info.bid[0].quantity;
    let bid_level1_orig_orders_no = book_info.bid[0].count;
    let bid_order1_qty = bid_orders[0].quantity;

    let modify_request = ModifyRequest {
        id: bid_orders[0].order_id,
        side: Side::Buy,
        price: bid_orders[0].price,
        quantity: bid_orders[0].quantity + 500,
        ord_type: OrdType::Limit,
    };

    let modify_result = book.modify(&modify_request);

    assert!(modify_result.0);
    assert!(modify_result.1.is_none());

    let book_info = book.info(None);
    let bid_level1_new_qty = book_info.bid[0].quantity;
    let bid_level1_new_orders_no = book_info.bid[0].count;

    assert_eq!(bid_level1_new_qty, bid_level1_orig_qty + 500);
    assert_eq!(bid_level1_new_orders_no, bid_level1_orig_orders_no);

    //test that modify request will decrease level's quantity and won't change number of orders
    let book_info = book.info(None);
    let bid_level2_orig_qty = book_info.bid[1].quantity;
    let bid_level2_orig_orders_no = book_info.bid[1].count;
    let bid_order1_qty = bid_orders[7].quantity;

    let modify_request = ModifyRequest {
        id: bid_orders[7].order_id,
        side: Side::Buy,
        price: bid_orders[7].price,
        quantity: bid_orders[7].quantity - 200,
        ord_type: OrdType::Limit,
    };

    let modify_result = book.modify(&modify_request);
    assert!(modify_result.0);
    assert!(modify_result.1.is_none());

    let book_info = book.info(None);
    let bid_level2_new_qty = book_info.bid[1].quantity;
    let bid_level2_new_orders_no = book_info.bid[1].count;

    assert_eq!(bid_level2_new_qty, bid_level2_orig_qty - 200);
    assert_eq!(bid_level2_new_orders_no, bid_level2_orig_orders_no);

    //test that modify request will modify order price to a more passive price without execution
    let book_info = book.info(None);
    let bid_level1_orig_qty = book_info.bid[0].quantity;
    let bid_level1_orig_orders_no = book_info.bid[0].count;
    let bid_level2_orig_qty = book_info.bid[1].quantity;
    let bid_level2_orig_orders_no = book_info.bid[1].count;
    let bid_order1_qty = bid_orders[1].quantity;

    let modify_request = ModifyRequest {
        id: bid_orders[1].order_id,
        side: Side::Buy,
        price: bid_orders[7].price,
        quantity: bid_orders[1].quantity + 300,
        ord_type: OrdType::Limit,
    };

    let modify_result = book.modify(&modify_request);
    assert!(modify_result.0);
    assert!(modify_result.1.is_none());

    let book_info = book.info(None);
    let bid_level1_new_qty = book_info.bid[0].quantity;
    let bid_level1_new_orders_no = book_info.bid[0].count;
    let bid_level2_new_qty = book_info.bid[1].quantity;
    let bid_level2_new_orders_no = book_info.bid[1].count;

    assert_eq!(bid_level1_new_qty, bid_level1_orig_qty - bid_order1_qty);
    assert_eq!(bid_level1_new_orders_no, bid_level1_orig_orders_no - 1);
    assert_eq!(
        bid_level2_new_qty,
        bid_level2_orig_qty + bid_order1_qty + 300
    );
    assert_eq!(bid_level2_new_orders_no, bid_level2_orig_orders_no + 1);

    //test that modify request will modify order price to a more aggresive price with execution
    let book_info = book.info(None);
    let ask_level1_orig_qty = book_info.ask[0].quantity;
    let ask_level1_orig_orders_no = book_info.ask[0].count;
    let bid_level2_orig_qty = book_info.bid[1].quantity;
    let bid_level2_orig_orders_no = book_info.bid[1].count;
    let bid_order1_qty = bid_orders[9].quantity;
    let ask_order_price = ask_orders[0].price;

    let modify_request = ModifyRequest {
        id: bid_orders[9].order_id,
        side: Side::Buy,
        price: ask_order_price,
        quantity: ask_level1_orig_qty - 100,
        ord_type: OrdType::Limit,
    };

    let modify_result = book.modify(&modify_request);
    assert!(modify_result.0);
    assert!(modify_result.1.is_some());

    let book_info = book.info(None);
    let ask_level1_new_qty = book_info.ask[0].quantity;
    let ask_level1_new_orders_no = book_info.ask[0].count;
    let bid_level2_new_qty = book_info.bid[1].quantity;
    let bid_level2_new_orders_no = book_info.bid[1].count;

    assert_eq!(ask_level1_new_qty, 100);
    assert_ne!(ask_level1_new_orders_no, ask_level1_orig_orders_no);
    assert_eq!(bid_level2_new_qty, bid_level2_orig_qty - bid_order1_qty);
    assert_eq!(bid_level2_new_orders_no, bid_level2_orig_orders_no - 1);
}
