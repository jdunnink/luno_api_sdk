use super::*;

#[async_trait]
impl Accounts for LunoClient {

    async fn update_account_name(&self, id: &str, name: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>>  {

        let url_str = self.get_base_url(Some(&format!("/api/1/accounts/{}/name", id)));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).put(url);
        let mut params = HashMap::new();
        
        params.insert("name", name);
        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }

    async fn create_account(&self, currency: &str, name: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/accounts"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();

        params.insert("currency", currency);
        params.insert("name", name);
        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }

    async fn list_pending_transactions(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/accounts/{}/pending", id)));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn list_transactions(&self, id: &str, min_row: i32, max_row: i32) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let mut append = format!("/api/1/accounts/{}/transactions", id);
        append = format!("{}?min_row={}&max_row={}", append, min_row.to_string(), max_row.to_string());
        let url_str = self.get_base_url(Some(&append));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn list_balances(&self, assets: Vec<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
 
        let mut url_str = self.get_base_url(Some("/api/1/balance?"));
        
        let vec_len = assets.len();
        let mut i = 0;
        while i < vec_len {
            url_str.push_str(&format!("assets={}", assets[i]));
            if i < vec_len - 1 {
                url_str.push('&');
            }
            i += 1;
        }

        let resp = self.dispatch(Method::GET, url_str, true).await?;   

        Ok(resp)
    }
}