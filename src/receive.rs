use super::*;

#[async_trait]
impl Receive for LunoClient {

    pub async fn get_receive_address(&self, asset: &str, address: Option<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut url_str = self.get_base_url(Some(&format!("api/1/funding_address?asset={}", asset)));

        match address {
            Some(a) => url_str.push_str(&format!("&address={}", a)),
            None => (),
        }

        let resp = self.dispatch(Method::POST, url_str, true).await?;

        Ok(resp)
    }
}