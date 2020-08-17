use super::*;

#[async_trait]
impl Receive for LunoClient {

    async fn get_receive_address(&self, asset: &str, address: Option<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some(&format!("/api/1/funding_address?asset={}", asset)));

        match address {
            Some(a) => url_str.push_str(&format!("&address={}", a)),
            None => (),
        }

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn create_receive_address(&self, asset: &str, name: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/funding_address"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();
        
        params.insert("asset", asset);
        params.insert("name", name);

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }
}