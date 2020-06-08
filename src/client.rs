use super::*;

impl LunoClient {

    pub fn new(api_key: &str, api_secret: &str) -> LunoClient {
    
        LunoClient {
            base_url: String::from("https://api.mybitx.com"),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
            client: Client::new(),
        }
    }

    pub fn get_base_url(&self, append: Option<&str>) -> String {

        let mut base_url = String::from(&self.base_url);
        
        match append {
            Some(a) => base_url.push_str(a),
            None => (),
        }
        
        base_url
    }

    pub fn add_url_params(&self, mut url_str: String, params: HashMap<&str, String>) -> String {

        let len = params.len();
        let mut i = 0;
        for (key, value) in params {
            url_str.push_str(&format!("{}={}", key, value));
            if i < len - 1 {
                url_str.push('&');
            }
            i += 1;
        }

        url_str
    }

    async fn go_get(&self, url: reqwest::Url, auth: bool) -> Result<reqwest::Response, Box<dyn std::error::Error>> {

        let mut reqw = (&self.client).get(url);

        if auth == true {
            reqw = reqw.basic_auth(&self.api_key, Some(&self.api_secret)); 
        }

        let resp = reqw.send().await?;

        Ok(resp)
    }

    pub async fn dispatch(&self, method: Method, url_str: String, auth: bool) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        
        if !url_str.contains(&self.base_url) {
            return Err("url_str does not contain base url.".into());
        }

        let url = Url::parse(&url_str).unwrap();

        match method {
            Method::GET => {
                let resp = self.go_get(url, auth).await?;
                Ok(resp)
            }
            _ => Err("method not found.".into())
        }
    }
}