use super::*;

#[async_trait]
impl Quotes for LunoClient {

    async fn exercise_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/quotes/{}", id)));
        let url = Url::parse(&url_str).unwrap();

        let mut reqw = (&self.client).put(url);

        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));
        let resp = reqw.send().await?;
        Ok(resp)
    }

    async fn delete_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/quotes/{}", id)));
        let url = Url::parse(&url_str).unwrap();

        let mut reqw = (&self.client).delete(url);

        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));
        let resp = reqw.send().await?;
        Ok(resp)
    }


    async fn create_quote(&self, action: &str, base_amount: f64, pair: &str, base_account_id: Option<&str>, counter_account_id: Option<&str>) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/quotes"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();
        let ba_string;

        ba_string = base_amount.to_string();

        params.insert("type", action);
        params.insert("base_amount", &ba_string);
        params.insert("pair", pair);
        match base_account_id{
            Some(s) => {
                params.insert("base_account_id", s);
            }
            None => (),
        }

        match counter_account_id{
            Some(s) => {
                params.insert("counter_account_id", s);
            }
            None => (),
        }

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));
        let resp = reqw.send().await?;
        Ok(resp)
    }

    async fn get_quote(&self, id: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some(&format!("/api/1/quotes/{}", id)));

        let resp = self.dispatch(Method::GET, url_str, true).await?;

        Ok(resp)
    }
}