
mod config;
mod client;
mod websocket;

use config::Config; 
use client::Client;

#[tokio::main] // This sets up the async runtime
async fn main() {

    let config = Config::from_env();
    let client: Client = Client::new(&config); 

    match client.get_server_time().await {
        Ok(time) => println!("Server time: {}", time), 
        Err(e) => eprintln!("Error: {}", e),
    }

    if let Err(e) = websocket::stream_trades("btcusdt").await {
        eprintln!("WebSocket failed: {}", e); 
    }

}
