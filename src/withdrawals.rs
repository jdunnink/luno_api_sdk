use super::*;

#[async_trait]
impl Withdrawals for LunoClient {

    async fn list_withdrawal_requests(&self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/withdrawals"));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }

    async fn get_withdrawal(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/withdrawals/{}", id)));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }
}