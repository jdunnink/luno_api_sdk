
use luno_sdk::LunoClient;
use luno_sdk::Quotes;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key_id = env::var("key_id").expect("api_key_id must be set");
    let api_key_secret = env::var("key_secret").expect("api_key_secret must be set");

    let luno_client = LunoClient::new(&api_key_id, &api_key_secret);

    let wdl_resp = luno_client.delete_quote("fake-id");
    println!("received reponse: {}", wdl_resp.await?.text().await?);

    Ok(())
}
