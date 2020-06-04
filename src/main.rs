use luno_sdk::LunoClient;
//use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use dotenv::dotenv;
use std::env;

//use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key_id = env::var("key_id").expect("api_key_id must be set");
    let api_key_secret = env::var("key_secret").expect("api_key_secret must be set");

    let mut luno_client = LunoClient::new(&api_key_id, &api_key_secret);

    let ticker_resp = luno_client.get_ticker("XBTEUR");
    println!("get_ticker method --> received reponse: {}", ticker_resp.await?.text().await?);

//    let tickers_resp = luno_client.get_tickers().await?;
//    println!("get_tickers method --> received reponse: {}", tickers_resp);

//    let d = NaiveDate::from_ymd(2020, 5, 30);
//    let t = NaiveTime::from_hms_milli(12, 34, 56, 789);

//    let dt = NaiveDateTime::new(d, t);

//    let trades_resp = luno_client.get_trades("XBTEUR", dt).await?;
//    println!("get trades method method --> received reponse: {}", trades_resp);

//    let orderbook_top_resp = luno_client.get_orderbook_top("XBTEUR").await?;
//    println!("get_orderbook method --> received reponse: {}", orderbook_top_resp);

//    let orderbook_resp = luno_client.get_orderbook("XBTEUR").await?;
//    println!("get_orderbook method --> received reponse: {}", orderbook_resp);

//    let balance_resp = luno_client.get_balance(vec!["XBT", "EUR", "XRP"]).await?;

//    println!("balance method --> received response: {}", balance_resp.text().await?);

//    let test_account_id = "2713851666598861617";

//    let transaction_resp = luno_client.list_transactions(test_account_id, -100, 0).await?;
//    println!("list transactions method--> received response: {:?}", transaction_resp.text().await?);

//    let pending_transaction_resp = luno_client.list_pending_transactions(test_account_id).await?;
//    println!("list transactions method--> received response: {:?}", pending_transaction_resp);

//    let benef_resp = luno_client.list_beneficiaries().await?;
//    println!("list beneficiaries response: {:?}", benef_resp);

//    let fee_info_resp = luno_client.get_fee_info("XBTEUR");
//    println!("fee info response: {:?}", fee_info_resp.await?);

//    let get_recv_addr_resp = luno_client.get_receive_address("XBT", None);

//    println!("response: {}", get_recv_addr_resp.await?.text().await?);

 //   let withdrawal_resp = luno_client.list_withdrawal_requests();

//    println!("response: {}", withdrawal_resp.await?.text().await?);

    Ok(())
}
