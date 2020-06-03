extern crate reqwest;
extern crate tokio;
extern crate chrono;

use chrono::NaiveDateTime;
use reqwest::Url;
use reqwest::Client;

pub struct LunoClient {
    pub base_url: String,
    pub append_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub client: reqwest::Client,
}

impl LunoClient {
    pub fn new(api_key: &str, api_secret: &str) -> LunoClient {
        LunoClient {
            base_url: String::from("https://api.mybitx.com"),
            append_url: String::from(""),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
            client: Client::new(),
        }
    }

    pub async fn get_ticker(&mut self, ticker: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/ticker?pair=");
        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);
        comb_url.push_str(ticker);
        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .send()
            .await?;      

        Ok(resp)
    }

    pub async fn get_tickers(&mut self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/tickers");

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_trades(&mut self, pair: &str, since: NaiveDateTime) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/trades?pair=");

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);
        comb_url.push_str(pair);
        comb_url.push_str("&since=");
        comb_url.push_str(&since.timestamp_millis().to_string());

        println!("provided url: {}", comb_url);

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .send()
            .await?;     

        Ok(resp)
    }

    pub async fn get_orderbook_top(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/orderbook_top?pair=");

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);
        comb_url.push_str(pair);

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_orderbook(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/orderbook?pair=");

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);
        comb_url.push_str(pair);

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_balance(&mut self, assets: Vec<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/balance?");

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);
        
        let vec_len = assets.len();
        let mut i = 0;
        while i < vec_len {
            comb_url.push_str(&format!("assets={}", assets[i]));
            if i < vec_len - 1 {
                comb_url.push_str("&");
            }
            i += 1;
        }

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;       

        Ok(resp)
    }

    pub async fn list_transactions(&mut self, id: &str, min_row: i32, max_row: i32) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        self.append_url = String::from(&format!("/api/1/accounts/{}/transactions", id));

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);
        comb_url.push_str(&format!("?min_row={}&max_row={}", min_row.to_string(), max_row.to_string()));

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn list_pending_transactions(&mut self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from(&format!("/api/1/accounts/{}/pending", id));

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn list_beneficiaries(&mut self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        self.append_url = String::from("/api/1/beneficiaries");

        let mut comb_url = String::from(&self.base_url);
        comb_url.push_str(&self.append_url);

        let url = Url::parse(&comb_url)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

}