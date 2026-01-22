use std::fmt::format;

use crate::client::Client;
use crate::config::Config;
use tokio::{
    net::unix::pipe::Receiver,
    sync::{mpsc, oneshot},
};

#[derive(Debug)]
pub enum OrderMessage {
    PlaceOrder {
        symbol: String,
        side: String,
        quantity: String,
        price: String,
        respond_to: oneshot::Sender<Result<serde_json::Value, String>>,
    },
    GetAccountInfo {
        respond_to: oneshot::Sender<Result<serde_json::Value, String>>,
    },
    GetPrice {
        symbol: String,
        respond_to: oneshot::Sender<Result<f64, String>>,
    },
}

pub struct OrderManagerActor {
    receiver: mpsc::Receiver<OrderMessage>,
    client: Client,
}

impl OrderManagerActor {
    pub fn new(receiver: mpsc::Receiver<OrderMessage>, config: &Config) -> Self {
        OrderManagerActor {
            receiver,
            client: Client::new(config),
        }
    }

    pub async fn run(mut self) {
        println!("[OrderManager] Actor started");

        while let Some(msg) = self.receiver.recv().await {
            match msg {
                OrderMessage::PlaceOrder {
                    symbol,
                    side,
                    quantity,
                    price,
                    respond_to,
                } => {
                    self.handle_place_order(symbol, side, quantity, price, respond_to)
                        .await;
                }
                OrderMessage::GetAccountInfo { respond_to } => {
                    self.handle_get_account(respond_to).await;
                }
                OrderMessage::GetPrice { symbol, respond_to } => {
                    self.handle_get_price(symbol, respond_to).await;
                }
            }
        }

        println!("[OrderManager] actor stopped");
    }

    async fn handle_place_order(
        &self,
        symbol: String,
        side: String,
        quantity: String,
        price: String,
        respond_to: oneshot::Sender<Result<serde_json::Value, String>>,
    ) {
        println!(
            "[OrderManager] 📋 Placing {} order: {} {} @ ${}",
            side, quantity, symbol, price
        );

        let result = self
            .client
            .place_order(&symbol, &side, &quantity, &price)
            .await;

        let response = match result {
            Ok(order) => {
                println!("[OrderManager] Order placed successfully");
                println!("[OrderManager] Order details: {}", order);
                Ok(order)
            }
            Err(e) => {
                eprintln!("[OrderManager] Order failed: {}", e);
                Err(format!("Order failed: {}", e))
            }
        };

        // Send response back (ignore if receiver dropped)
        let _ = respond_to.send(response);
    }

    async fn handle_get_account(
        &self,
        respond_to: oneshot::Sender<Result<serde_json::Value, String>>,
    ) {
        println!("[OrderManager] Fetching account info");

        let result = self.client.get_account().await;

        let response = match result {
            Ok(account) => {
                println!("[OrderManager] Account info retrieved");
                Ok(account)
            }
            Err(e) => {
                eprintln!("[OrderManager] Failed to get account: {}", e);
                Err(format!("Failed to get account: {}", e))
            }
        };

        let _ = respond_to.send(response);
    }

    async fn handle_get_price(
        &self,
        symbol: String,
        respond_to: oneshot::Sender<Result<f64, String>>,
    ) {
        println!("[OrderManager] Fetching price for {}", symbol);

        let result = self.client.get_price(&symbol).await;

        let response = match result {
            Ok(price) => {
                println!("[OrderManager] Price for {}: ${}", symbol, price);
                Ok(price)
            }
            Err(e) => {
                eprintln!("[OrderManager] Failed to get price: {}", e);
                Err(format!("Failed to get price: {}", e))
            }
        };

        let _ = respond_to.send(response);
    }
}

#[derive(Clone)]
pub struct OrderManagerHandle {
    sender: mpsc::Sender<OrderMessage>,
}

impl OrderManagerHandle {
    pub fn new(config: &Config) -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let actor = OrderManagerActor::new(receiver, config);

        tokio::spawn(actor.run());

        Self { sender }
    }

    pub async fn place_order(
        &self,
        symbol: String,
        side: String,
        quantity: String,
        price: String,
    ) -> Result<serde_json::Value, String> {
        let (send, recv) = oneshot::channel();

        self.sender
            .send(OrderMessage::PlaceOrder {
                symbol,
                side,
                quantity,
                price,
                respond_to: (send),
            })
            .await
            .map_err(|_| "Actor is dead".to_string())?;

        recv.await
            .map_err(|_| "Actor dropped response".to_string())?
    }

    pub async fn get_acconunt_info(&self) -> Result<serde_json::Value, String> {
        let (send, recv) = oneshot::channel();

        self.sender
            .send(OrderMessage::GetAccountInfo { respond_to: send })
            .await
            .map_err(|_| "Actor is dead".to_string())?;
        recv.await
            .map_err(|_| "Actor dropped response".to_string())?
    }

    pub async fn get_price(&self, symbol: String) -> Result<f64, String> {
        let (send, recv) = oneshot::channel();

        self.sender
            .send(OrderMessage::GetPrice {
                symbol,
                respond_to: send,
            })
            .await
            .map_err(|_| "Actor is dead".to_string())?;

        recv.await
            .map_err(|_| "Actor dropped response".to_string())?
    }
}
