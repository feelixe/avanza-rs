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

    use crate::totp::generate_totp;

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
        };

        let client = super::client::Client::authenticate(&credentials, &totp_secret)
            .await
            .expect("Auth failed");

        let accounts = client.list_accounts().await.expect("List accounts failed");

        println!("{}", serde_json::to_string(&accounts).unwrap());
    }
}
