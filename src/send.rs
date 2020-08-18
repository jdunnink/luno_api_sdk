use super::*;

#[async_trait]
impl Send for LunoClient {

    async fn send(
        &self,
    amount:         &str,
    currency:       &str,
    address:        &str,
    description:    Option<&str>,
    message:        Option<&str>,
    external_id:    Option<&str>,
    dest_tag_flag:  bool,
    dest_tag:       Option<&str>,
    )  -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let url_str = self.get_base_url(Some("/api/1/send"));
        let url = Url::parse(&url_str).unwrap();
        let mut reqw = (&self.client).post(url);
        let mut params = HashMap::new();
        
        
        params.insert("amount", amount);
        params.insert("currency", currency);
        params.insert("address", address);

        match description {
            Some(d) => {
                params.insert("description", d);
            },
            None => (),
        }

        match message {
            Some(m) => {
                params.insert("message", m);
            },
            None => (),
        }

        match external_id {
            Some(e) => {
                params.insert("external_id", e);
            },
            None => (),
        }

        match dest_tag {
            Some(d) => {
                if dest_tag_flag == true {
                    params.insert("destination_tag", d);
                }
            },
            None => (),
        }

        reqw = reqw.form(&params);
        reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret));

        let resp = reqw.send().await?;

        Ok(resp)
    }
}