use super::*;

#[async_trait]
impl Quotes for LunoClient {

    async fn get_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/quotes/{}", id)));

        let url = Url::parse(&url_str)
            .unwrap();

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }
}