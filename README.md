# Binance Trading Bot (Rust Learning Project)

A Rust-based Binance trading bot built for learning purposes. Currently supports public REST API calls and WebSocket streaming.

## Features

- **REST API**: Fetch server time from Binance
- **WebSocket**: Stream real-time trade data
- **Async**: Built on Tokio runtime
- **Config**: Environment-based configuration via `.env`

## Project Structure

```
src/
├── main.rs       # Entry point, orchestrates modules
├── config.rs     # Environment configuration loading
├── client.rs     # HTTP client for REST API calls
└── websocket.rs  # WebSocket streaming for market data
```

## Prerequisites

- Rust (latest stable)
- Binance account (optional, for future authenticated endpoints)

## Setup

1. Clone the repository:
   ```bash
   git clone <your-repo-url>
   cd binance_bot
   ```

2. Create a `.env` file in the project root:
   ```
   BINANCE_BASE_URL=https://testnet.binance.vision
   BINANCE_API_KEY=your_api_key_here
   BINANCE_API_SECRET=your_api_secret_here
   ```

3. Build and run:
   ```bash
   cargo run
   ```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `dotenvy` | Load environment variables from `.env` |
| `reqwest` | HTTP client for REST API |
| `tokio` | Async runtime |
| `tokio-tungstenite` | WebSocket client |
| `futures-util` | Stream utilities for WebSocket |
| `serde_json` | JSON parsing |

## Usage

Running the bot will:
1. Print the Binance server time
2. Connect to the BTC/USDT trade stream and print live trades

Press `Ctrl+C` to stop.

## Roadmap

- [x] Project structure & modules
- [x] Config & env handling
- [x] Public REST calls
- [x] WebSocket market data
- [ ] Strategy as pure logic
- [ ] Paper trading
- [ ] Signed requests (authenticated endpoints)
- [ ] Real testnet orders

## Notes

- This is an educational project, not intended for production trading
- Currently uses Binance testnet for safety
- No real orders are placed

## License

MIT
