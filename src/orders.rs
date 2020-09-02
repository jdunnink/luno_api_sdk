use super::*;

#[async_trait]
impl Orders for LunoClient {

    async fn get_fee_info(&self, pair: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let url_str = self.get_base_url(Some(&format!("/api/1/fee_info?pair={}", pair)));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn list_orders(&self, params: Option<HashMap<&str, String>>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some("/api/1/listorders"));

        match params {
            Some(p) => {
                let len = p.len();
                if len > 0 {
                    url_str.push('?');
                    url_str = self.add_url_params(url_str, p);
                }
            }
            None => (),
        }

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn list_trades_user(&self, mut params: HashMap<&str, String>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some("/api/1/listtrades"));

        if params.contains_key("pair") {
            url_str.push_str(&format!("?pair={}", params["pair"]));
            params.remove("pair");
        } else {
            return Err("no currency pair defined in params".into());
        }

        url_str = self.add_url_params(url_str, params);

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn get_order(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/orders/{}", id)));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn post_market_order(
                            &self,
        pair:               &str,
        action:             &str,
        counter_volume:     Option<&str>,           // for buy orders
        base_volume:        Option<&str>,           // for sell orders
        base_account_id:    Option<&str>,
        counter_account_id: Option<&str>,
    )  -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/marketorder"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();
        
        
        params.insert("pair", pair);
        params.insert("type", action);

        match counter_volume {
            Some(c) => {
                params.insert("counter_volume", c);
            },
            None => (),
        }

        match base_volume {
            Some(b) => {
                params.insert("base_volume", b);
            },
            None => (),
        }

        match base_account_id {
            Some(b) => {
                params.insert("base_account_id", b);
            },
            None => (),
        }

        match counter_account_id {
            Some(c) => {
                    params.insert("counter_account_id", c);
            },
            None => (),
        }

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }

    async fn post_limit_order(
                            &self,
        pair:               &str,
        action:             &str,
        post_only:          Option<&str>,
        volume:             &str,
        price:              &str,
        stop_price:         Option<&str>,
        stop_direction:     Option<&str>,
        base_account_id:    Option<&str>,
        counter_account_id: Option<&str>
    )  -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/postorder"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();

        params.insert("pair", pair);
        params.insert("type", action);
        params.insert("volume", volume);
        params.insert("price", price);

        match post_only {
            Some(p) => {
                params.insert("post_only", p);
            },
            None => (),
        }

        match stop_price {
            Some(s) => {
                params.insert("stop_price", s);
            },
                None => (),
        }

        match stop_direction {
            Some(s) => {
                params.insert("stop_direction", s);
            },
                None => (),
        }

        match base_account_id {
            Some(b) => {
                params.insert("base_account_id", b);
            },
                None => (),
        }

        match counter_account_id {
            Some(c) => {
                params.insert("counter_account_id", c);
            },
                None => (),
        }

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }

    async fn stop_order(&self, order_id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/stoporder"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();

        params.insert("order_id", order_id);

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }
}