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

    async fn request_withdrawal(
        &self,
        action: &str,
        amount: &str,
        benef_id: Option<&str>,
        reference: Option<&str>,
        extern_id: Option<&str>    
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/withdrawals"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();
        
        params.insert("type", action);
        params.insert("amount", amount);

        match benef_id {
            Some(id) => {
                params.insert("beneficiary_id", id);
            },
            None => (),
        }

        match reference {
            Some(r) => {
                params.insert("reference", r);
            },
            None => (),
        }

        match extern_id {
            Some(e) => {
                params.insert("external_id", e);
            },
            None => (),
        }

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }

    async fn cancel_withdrawal_request(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/withdrawals/{}", id)));

        let url = Url::parse(&url_str).unwrap();

        let mut reqw = (&self.client).delete(url);

        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));
        let resp = reqw.send().await?;
        Ok(resp)
    }
}