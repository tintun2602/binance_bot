use futures_util::StreamExt; 
use tokio_tungstenite::connect_async; 

pub async fn stream_trades(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("wss://stream.binance.com:9443/ws/{}@trade", symbol.to_lowercase());
    let (ws_stream, _) = connect_async(&url).await?;

    println!("Connected to {} trade stream", symbol);

    let (_, mut read) = ws_stream.split(); 

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    println!("{}", msg.to_text()?);
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e); 
                break;
            }
        }
    }

    Ok(())
}