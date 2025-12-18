use crate::config::Config; 


pub struct Client {
    base_url: String, 
    http: reqwest::Client
}

impl Client {

    pub fn new(config: &Config) -> Self {
        Self {
            base_url: config.base_url.clone(), 
            http: reqwest::Client::new()
        }
    }

    pub async  fn get_server_time(&self) -> Result<u64, reqwest::Error> {
        let url = format!("{}/api/v3/time", self.base_url);
        let response: serde_json::Value = self.http.get(&url).send().await?.json().await?;
        Ok(response["serverTime"].as_u64().unwrap())
    }
}