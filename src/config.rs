#[derive(Debug, Clone)]
pub struct Urls {
    pub authenticate: String,
    pub get_session: String,
    pub totp: String,
    pub total_values: String,
    pub accounts: String,
    pub fund_buy: String,
    pub stock_order: String,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub urls: Urls,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            urls: Urls {
                fund_buy: String::from("https://www.avanza.se/_api/fund-guide/fund-order-page/buy"),
                stock_order: String::from("https://www.avanza.se/_api/trading-critical/rest/order/new"),
                authenticate: String::from("https://www.avanza.se/_api/authentication/sessions/usercredentials"),
                totp: String::from("https://www.avanza.se/_api/authentication/sessions/totp"),
                get_session: String::from("https://www.avanza.se/_cqbe/authentication/session"),
                accounts: String::from("https://www.avanza.se/_api/account-overview/accounts/list"),
                total_values: String::from("https://www.avanza.se/_api/account-performance/overview/total-values"),
            },
        }
    }
}
