extern crate reqwest;
extern crate tokio;
extern crate chrono;

use std::collections::HashMap;
use reqwest::Url;
use reqwest::Client;
use reqwest::Method;

pub struct LunoClient {
    base_url: String,
    api_key: String,
    api_secret: String,
    client: reqwest::Client,
}

impl LunoClient {

    pub fn new(api_key: &str, api_secret: &str) -> LunoClient {
    
        LunoClient {
            base_url: String::from("https://api.mybitx.com"),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
            client: Client::new(),
        }
    }

    pub fn get_base_url(&self, append: Option<&str>) -> String {

        let mut base_url = String::from(&self.base_url);
        
        match append {
            Some(a) => base_url.push_str(a),
            None => (),
        }
        
        base_url
    }

    pub fn add_url_params(&self, mut url_str: String, params: HashMap<&str, String>) -> String {

        let len = params.len();
        let mut i = 0;
        for (key, value) in params {
            url_str.push_str(&format!("{}={}", key, value));
            if i < len - 1 {
                url_str.push_str("&");
            }
            i += 1;
        }

        url_str
    }

    pub async fn go_get(&self, url: reqwest::Url, auth: bool) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut reqw = (&self.client).get(url);

        if auth == true {
            reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret)); 
        }

        let resp = reqw.send().await?;

        Ok(resp)
    }

    pub async fn dispatch(&self, method: Method, url_str: String, auth: bool) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let url = Url::parse(&url_str).unwrap();

        match method {
            Method::GET => {
                let resp = self.go_get(url, auth).await?;
                Ok(resp)
            }
            _ => Err("error occurred: method not found in dispatch".into())
        }
    }

    pub async fn get_balance(&mut self, assets: Vec<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
 
        let mut url_str = self.get_base_url(Some("/api/1/balance?"));
        
        let vec_len = assets.len();
        let mut i = 0;
        while i < vec_len {
            url_str.push_str(&format!("assets={}", assets[i]));
            if i < vec_len - 1 {
                url_str.push_str("&");
            }
            i += 1;
        }

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;       

        Ok(resp)
    }

    pub async fn list_transactions(&mut self, id: &str, min_row: i32, max_row: i32) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let mut append = format!("/api/1/accounts/{}/transactions", id);
        append = format!("{}?min_row={}&max_row={}", append, min_row.to_string(), max_row.to_string());
        let url_str = self.get_base_url(Some(&append));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_fee_info(&mut self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let url_str = self.get_base_url(Some(&format!("/api/1/fee_info?pair={}", pair)));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn list_pending_transactions(&mut self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/accounts/{}/pending", id)));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn list_beneficiaries(&mut self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let url_str = self.get_base_url(Some("/api/1/beneficiaries"));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn list_orders(&mut self, params: Option<HashMap<&str, String>>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some("/api/1/listorders"));

        match params {
            Some(p) => {
                let len = p.len();
                if len > 0 {
                    url_str.push_str("?");
                    url_str = self.add_url_params(url_str, p);
                }
            }
            None => (),
        }

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }


    pub async fn list_trades_user(&mut self, mut params: HashMap<&str, String>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some("/api/1/listtrades"));

        if params.contains_key("pair") {
            url_str.push_str(&format!("?pair={}", params["pair"]));
            params.remove("pair");
        } else {
            return Err("no currency pair defined in params".into());
        }

        url_str = self.add_url_params(url_str, params);

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_order(&mut self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/orders/{}", id)));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_quote(&mut self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/quotes/{}", id)));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_receive_address(&mut self, asset: &str, address: Option<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some(&format!("api/1/funding_address?asset={}", asset)));

        match address {
            Some(a) => url_str.push_str(&format!("&address={}", a)),
            None => (),
        }

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn list_withdrawal_requests(&mut self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/withdrawals"));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }

    pub async fn get_withdrawal(&mut self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/withdrawals/{}", id)));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = (&self.client)
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        Ok(resp)
    }
}