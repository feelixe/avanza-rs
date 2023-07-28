# avanza-rs
Inspired by [fhqvst/avanza](https://github.com/fhqvst/avanza)

## Usage
```rust
use avanza_rs::client:{Client, Credentials};

let username = "username";
let password = "password";
let totp_secret = "totp_secret";

let credentials = super::client::Credentials {
    username: String::from(username),
    password: String::from(password),
    totp_secret: String::from(totp_secret)
};

let client = super::client::Client::authenticate(&credentials).await?;

let accounts = client.list_accounts().await?;

let account = client.get_acccount("1234567").await?;

let stock_order = StockOrder {
    orderbook_id: String::from("5247"),
    account_id: account.id.clone(),
    price: 215.0,
    volume: 10,
    side: Side::BUY,
};

let result = client.new_stock_order(&stock_order).await?;

```

## How to create totp_secret

1. Go to Mina Sidor > Profil > Sajtinställningar > Tvåfaktorsinloggning and click "Återaktivera". (Only do this step if you have already set up two-factor auth.)
2. Click "Aktivera" on the next screen.
3. Select "Annan app för tvåfaktorsinloggning".
4. Click "Kan du inte scanna QR-koden?" to reveal your TOTP Secret.
5. Install crate totp with `cargo install totp`.
6. Run `totp <TOTP_SECERT>` and enter the 6 digit code on Avanza.
7. Done, save you're `<TOTP_SECERT>` to `.env`
