
mod config;
mod client;
mod websocket;

use config::Config; 
use client::Client;

#[tokio::main] // This sets up the async runtime
async fn main() {

    let config = Config::from_env();
    let client: Client = Client::new(&config); 

    match client.get_account().await {
      Ok(account) => println!("Account works!"),
      Err(e) => eprintln!("Account error: {}", e),
    }


    match client.get_account().await {
        Ok(account) => {
            println!("Account balances:"); 
            if let Some(balances) = account["balances"].as_array() {
                for balance in balances {
                    let asset = balance["asset"].as_str().unwrap_or(""); 
                    let free = balance["free"].as_str().unwrap_or("0"); 
                    if free != "0.00000000" {
                        println!(" {}: {}", asset, free); 
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e)
    }

        // Get current BTC price
    match client.get_price("BTCUSDT").await {
        Ok(price) => {
            println!("Current BTC price: ${:.2}", price);
            
            // Place buy order 5% below market price
            let buy_price = price * 0.95;
            println!("Placing buy order at: ${:.2}", buy_price);
            
            match client.place_order(
                "BTCUSDT",
                "BUY",
                "0.001",
                &format!("{:.2}", buy_price)
            ).await {
                Ok(order) => println!("Order placed: {:#?}", order),
                Err(e) => eprintln!("Order error: {}", e),
            }
        }
        Err(e) => eprintln!("Price error: {}", e),
    }

    match client.place_order("BTCUSDT", "BUY", "0.001", "89889").await {
        Ok(order) => println!("Order placed: {:#?}", order),
        Err(e) => eprintln!("Error: {}", e)
    }


}
