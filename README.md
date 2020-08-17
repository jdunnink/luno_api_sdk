# luno_api_sdk
SDK in Rust for accessing the Luno exchange API.

This is an unofficial Rust API Wrapper for accessing the Luno Exchange API. Please read the License

<CAUTION>

- This module is a work in progress and is by no means ready to use in production.
- This module is made for educational purposes and is not meant to be used in a real cryptocurrency trading application.
- Any financial losses resulting from the use of this module, are solely the responsibility of the persons involved.
- Any liability exposure resulting from the use of this module, are solely the responsibility of the persons involved.



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
  
  todo:
  
  - send()
  - request_withdrawal()
  - cancel_withdrawal()
  - exercise_quote()
  - discard_quote()
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





















MIT License

Copyright (c) 2020 Jesse Dunnink

This software is licensed under the MIT License:

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
