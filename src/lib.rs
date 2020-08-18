use async_trait::async_trait;
use chrono::NaiveDateTime;
use std::collections::HashMap;

use reqwest::Url;
use reqwest::Client;
use reqwest::Method;

mod accounts;
mod market;
mod client;
mod beneficiaries;
mod orders;
mod withdrawals;
mod receive;
mod quotes;
mod send;

// todo list //

//      add post methods
//      add put methods
//      add unit tests
//      add integration tests

pub struct LunoClient {
    base_url: String,
    api_key: String,
    api_secret: String,
    client: reqwest::Client,
}

#[async_trait]
pub trait Accounts {
    async fn create_account(&self, ticker: &str, name: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn update_account_name(&self, id: &str, name: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_pending_transactions(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_transactions(&self, id: &str, min_row: i32, max_row: i32) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_balances(&self, assets: Vec<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Market {
    async fn get_ticker(&self, ticker: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_tickers(&self) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_trades_market(&self, pair: &str, since: Option<NaiveDateTime>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_orderbook_top(&self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_orderbook(&self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Beneficiaries {
    async fn list_beneficiaries(&self) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Orders {
    async fn get_fee_info(&self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_orders(&self, params: Option<HashMap<&str, String>>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_trades_user(&self, mut params: HashMap<&str, String>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    // post market order
    async fn get_order(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    // post limit order
    // stop order
}

#[async_trait]
pub trait Quotes {
    async fn create_quote(&self, action: &str, base_amount: f64, pair: &str, base_account_id: Option<&str>, counter_account_id: Option<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn exercise_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn delete_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Receive {
    async fn get_receive_address(&self, asset: &str, address: Option<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn create_receive_address(&self, asset: &str, name: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Send {
    async fn send(
                        &self,
        amount:         &str,
        currency:       &str,
        address:        &str,
        description:    Option<&str>,
        message:        Option<&str>,
        external_id:    Option<&str>,
        dest_tag_flag:  bool,
        dest_tag:       Option<&str>,
    )  -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Withdrawals {
    async fn list_withdrawal_requests(&self) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_withdrawal(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn request_withdrawal(
        &self,
        action: &str,
        amount: &str,
        benef_id: Option<&str>,
        reference: Option<&str>,
        extern_id: Option<&str>    
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn cancel_withdrawal_request(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}