mod actors;
mod client;
mod config;
mod websocket;

use actors::OrderManagerHandle; // Add this
use config::Config;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    println!("Binance Trading Bot - Actor Version\n");

    let config = Config::from_env();
    let order_manager = OrderManagerHandle::new(&config);

    println!("=== Test 1: Account Info ===");
    match order_manager.get_acconunt_info().await {
        Ok(account) => println!("Account: {}\n", account),
        Err(e) => println!("Error {} \n", e),
    }

    sleep(Duration::from_secs(1)).await;

    println!("=== Test 2: Get BTC Price ===");
    match order_manager.get_price("BTCUSDT".to_string()).await {
        Ok(price) => println!("BTC Price: ${}\n", price),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    sleep(Duration::from_secs(1)).await;

    println!("=== Test 3: Place Order (commented out for safety) ===");
}
