pub mod account;
pub mod client;
pub mod char_encode;
pub mod config;
pub mod error;
pub mod fund;
pub mod position;
pub mod totp;
pub mod stock;
pub mod middleware;

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{totp::generate_totp, stock::list::{StockListRequest, StockListFilter, StockListSortBy}};

    #[test]
    fn totp() {
        dotenvy::dotenv().ok();
        let totp_secret =
            env::var("AVANZA_TOTP_SECRET").expect("AVANZA_TOTP_SECRET is required in .env");
        let code = generate_totp(&totp_secret);
        println!("code: {}", code);
    }

    #[tokio::test]
    async fn list_accounts() {
        dotenvy::dotenv().ok();
        let username = env::var("AVANZA_USERNAME").expect("AVANZA_USERNAME is required in .env");
        let password = env::var("AVANZA_PASSWORD").expect("AVANZA_PASSWORD is required in .env");
        let totp_secret =
            env::var("AVANZA_TOTP_SECRET").expect("AVANZA_TOTP_SECRET is required in .env");

        let credentials = super::client::Credentials {
            username: String::from(username),
            password: String::from(password),
            totp_secret: String::from(totp_secret)
        };

        let client = super::client::Client::authenticate(&credentials)
            .await
            .expect("Auth failed");

        let request = StockListRequest {
            filter: StockListFilter {
                country_codes: vec![String::from("SE")]
            },
            limit: 20,
            offset: 0,
            sort_by: StockListSortBy {
                field: String::from("name"),
                order: String::from("desc")
            }
        };

        let stocks = client.get_stock_list(&request).await.expect("could not get stocklist");

        println!("{:#?}", stocks);
    }
}
