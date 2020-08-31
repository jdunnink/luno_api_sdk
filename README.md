# luno_api_sdk
SDK in Rust for accessing the Luno exchange API.

This is an unofficial Rust API Wrapper for accessing the Luno Exchange API. Please read the License.

<CAUTION>

- This module is a work in progress and is by no means ready to use in production.
- This module is made for educational purposes and is not meant to be used in a real cryptocurrency trading application.
- Any financial losses resulting from the use of this module, are solely the responsibility of the persons involved.


supported methods:  (last updated: 17-08-2020)

  - create_account()
  - update_account()
  - list_pending_transactions()
  - list_balances()
  - get_ticker()
  - get_tickers()
  - list_trades_market()
  - get_orderbook_top()
  - get_orderbook()
  - list_beneficiaries
  - get_fee_info()
  - list_orders()
  - list_trades_user()
  - get_order()
  - create_quote()
  - get_quote()
  - get_receive_address()
  - create_receive_address()
  - list_withdrawawl_requests()
  - get_withdrawal()  
  - send()
  - request_withdrawal()
  - cancel_withdrawal()
  - exercise_quote()
  - discard_quote()
  - post_market_order()
  - post_limit_order()
  - stop_order()
  
  todo:
  
  - add unit tests
  - add integration tests
  - refactor method functionality
  - refactor as crate
  
  example of use:
  
```rust
use luno_sdk::LunoClient;
use luno_sdk::Market;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let luno_client = LunoClient::new("api_key_id", "api_key_secret");

    let resp = luno_client.get_ticker("XBTEUR");
    println!("received reponse: {}", resp.await?.text().await?);

    Ok(())
}
```
