use async_trait::async_trait;
use chrono::NaiveDateTime;
use super::LunoClient;
use reqwest::Method;

#[async_trait]
pub trait Market {
    async fn get_ticker(&mut self, ticker: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_tickers(&mut self) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn list_trades_market(&mut self, pair: &str, since: Option<NaiveDateTime>) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_orderbook_top(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
    async fn get_orderbook(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>;
}

#[async_trait]
impl Market for LunoClient {

    async fn get_ticker(&mut self, ticker: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/ticker?pair={}", ticker)));
        
        let resp = self.dispatch(Method::GET, url_str, false).await?;
        
        Ok(resp)
    }

    async fn get_tickers(&mut self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/tickers"));

        let resp = self.dispatch(Method::GET, url_str, false).await?;

        Ok(resp)
    }

    async fn list_trades_market(&mut self, pair: &str, since: Option<NaiveDateTime>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut append = format!("/api/1/trades?pair={}", pair);

        match since {
            Some(t) => append.push_str(&format!("&since={}", t.timestamp_millis().to_string())),
            None => (),
        }

        let url_str = self.get_base_url(Some(&append));

        let resp = self.dispatch(Method::GET, url_str, false).await?;

        Ok(resp)
    }

    async fn get_orderbook_top(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
 
        let url_str = self.get_base_url(Some(&format!("/api/1/orderbook_top?pair={}", pair)));

        let resp = self.dispatch(Method::GET, url_str, false).await?;

        Ok(resp)
    }

    async fn get_orderbook(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
 
        let url_str = self.get_base_url(Some(&format!("/api/1/orderbook?pair={}", pair)));

        let resp = self.dispatch(Method::GET, url_str, false).await?;

        Ok(resp)
    }
}