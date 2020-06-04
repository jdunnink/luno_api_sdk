use super::*;

#[async_trait]
impl Beneficiaries for LunoClient {

    async fn list_beneficiaries(&self) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        let url_str = self.get_base_url(Some("/api/1/beneficiaries"));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }
}