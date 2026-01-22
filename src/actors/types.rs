use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum TradingSignal {
    Buy {
        symbol: String,
        price: f64,
        quantity: String,
    },

    Sell {
        symbol: String,
        price: f64,
        quantity: String,
    },

    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResult {
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub status: String,
}
