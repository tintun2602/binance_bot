use std::env;

pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
}

impl Config {
    pub fn new(base_url: String, api_key: String, api_secret: String) -> Self {
        Self {
            base_url,
            api_key,
            api_secret,
        }
    }

    pub fn from_env() -> Self {
        dotenvy::dotenv().expect("Failed to load .env file");

        let base_url = env::var("BINANCE_BASE_URL").expect("Missing BINANCE_BASE_URL");
        let api_key = env::var("BINANCE_API_KEY").expect("Missing BINANCE_API_KEY");
        let api_secret = env::var("BINANCE_API_SECRET").expect("Missing BINANCE_API_SECRET");
        Self {
            base_url,
            api_key,
            api_secret,
        }
    }
}
