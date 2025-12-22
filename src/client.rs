use crate::config::Config; 
use hmac::{Hmac, Mac}; 
use sha2::Sha256;
use tokio::signal;


type HmacSha256 = Hmac<Sha256>;


pub struct Client {
    base_url: String,
    api_key: String, 
    api_secret: String, 
    http: reqwest::Client
}

impl Client {

    pub fn new(config: &Config) -> Self {
        Self {
            base_url: config.base_url.clone(), 
            api_key: config.api_key.clone(),
            api_secret: config.api_secret.clone(),
            http: reqwest::Client::new()
        }
    }

    pub async  fn get_server_time(&self) -> Result<u64, reqwest::Error> {
        let url = format!("{}/api/v3/time", self.base_url);
        let response: serde_json::Value = self.http.get(&url).send().await?.json().await?;
        Ok(response["serverTime"].as_u64().unwrap())
    }

    fn sign(&self, query: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(query.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    fn timestamp(&self) -> u64 {
        std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64
    }

    pub async fn get_account(&self) -> Result<serde_json::Value, reqwest::Error> {
        let timestamp = self.timestamp(); 
        let query = format!("timestamp={}", timestamp); 
        let signature = self.sign(&query); 

        let url = format!(
        "{}/api/v3/account?{}&signature={}",
            self.base_url, query, signature
        );

        let response = self.http
        .get(&url)
        .header("X-MBX-APIKEY", &self.api_key)
        .send()
        .await?
        .json()
        .await?;

        Ok(response)
    }

    pub async fn place_order(
        &self,
        symbol: &str,
        side: &str,
        quantity: &str,
        price: &str,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let timestamp = self.timestamp();
        let query = format!(
            "symbol={}&side={}&type=LIMIT&timeInForce=GTC&quantity={}&price={}&timestamp={}",
            symbol, side, quantity, price, timestamp
        );
        let signature = self.sign(&query);
        let body = format!("{}&signature={}", query, signature);

        let url = format!("{}/api/v3/order", self.base_url);

        let response = self.http
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_price(&self, symbol: &str) -> Result<f64, reqwest::Error> {
        let url = format!("{}/api/v3/ticker/price?symbol={}", self.base_url, symbol);
        let response: serde_json::Value = self.http.get(&url).send().await?.json().await?;
        
        let price_str = response["price"].as_str().unwrap_or("0");
        let price: f64 = price_str.parse().unwrap_or(0.0);
    
        Ok(price)
}


}