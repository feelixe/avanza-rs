mod account;
mod char_encode;
mod client;
mod config;
mod error;
mod fund;
mod middleware;
mod position;
mod stock;
mod totp;

pub use account::list::AccountListResponse;
pub use account::Account;
pub use client::Client;
pub use config::{Configuration, Urls};
pub use error::Error;
pub use fund::buy::{FundBuyRequest, FundBuyResponse};
pub use position::total_values::{Aggegated, TotalValue, TotalValuesResponse};
pub use stock::list::{
    Instrument, StockListFilter, StockListRequest, StockListResponse, StockListResponsePagination,
    StockListSortBy,
};
pub use stock::order::{
    OrderRequestStatus, Side, StockOrder, StockOrderResponse, StockOrderUnknownResponse,
};

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        stock::list::{StockListFilter, StockListRequest, StockListSortBy},
        totp::generate_totp,
    };

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
            totp_secret: String::from(totp_secret),
        };

        let client = super::client::Client::authenticate(&credentials)
            .await
            .expect("Auth failed");

        let request = StockListRequest {
            filter: StockListFilter {
                country_codes: vec![String::from("SE")],
            },
            limit: 20,
            offset: 0,
            sort_by: StockListSortBy {
                field: String::from("name"),
                order: String::from("desc"),
            },
        };

        let stocks = client
            .get_stock_list(&request)
            .await
            .expect("could not get stocklist");

        println!("{:#?}", stocks);
    }
}
