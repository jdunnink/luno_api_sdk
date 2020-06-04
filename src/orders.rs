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
                    url_str.push_str("?");
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
}