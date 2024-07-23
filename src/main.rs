use chrono::NaiveDateTime;
use env_logger;
use futures_util::StreamExt;
use log::{error, info};
use serde::Deserialize;
use tokio_tungstenite::connect_async;

#[derive(Deserialize, Debug)]
struct TradeMessage {
    s: String, // Symbol
    p: String, // Price
    q: String, // Quantity
    E: u64,    // Event time
}

async fn handle_trades(json_message: &str) {
    let trade: TradeMessage = serde_json::from_str(json_message).unwrap();
    let date_time = NaiveDateTime::from_timestamp((trade.E / 1000) as i64, 0);
    println!("SYMBOL: {}", trade.s);
    println!("PRICE: {}", trade.p);
    println!("QTY: {}", trade.q);
    println!("TIMESTAMP: {}", date_time.format("%Y-%m-%d %H:%M:%S"));
    println!("-----------------------");
}

#[tokio::main]
async fn main() {

    //TODO

    // env_logger::init();
    // let socket = "wss://fstream.binance.com/ws/bnbusdt@aggTrade";
    // match connect_async(socket).await {
    //     Ok((ws_stream, _)) => {
    //         info!("WebSocket connected");

    //         let (_, read) = ws_stream.split();
    //         read.for_each(|message| async {
    //             match message {
    //                 Ok(msg) => {
    //                     if let Ok(text) = msg.to_text() {
    //                         handle_trades(text).await;
    //                     }
    //                 }
    //                 Err(e) => {
    //                     error!("Error: {}", e);
    //                 }
    //             }
    //         })
    //         .await;
    //     }
    //     Err(e) => {
    //         error!("Failed to connect: {:?}", e);
    //     }
}
// }
