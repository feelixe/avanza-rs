use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::client::Client;
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundBuyRequest {
    pub orderbook_id: String,
    pub account_id: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundBuyResponse {
    pub order_request_status: String,
    pub order_id: i64,
    pub message: String,
    pub account_id: String,
}

impl Client {
    pub async fn buy_fund(
        &self,
        account: &Account,
        orderbook_id: String,
        amount: f64,
    ) -> Result<FundBuyResponse, Error> {
        let order = FundBuyRequest {
            orderbook_id,
            account_id: account.id.clone(),
            amount: (amount * 100.0).round() / 100.0,
        };
        let res = self
            .http_client
            .post(&self.config.urls.fund_buy)
            .body_json(&order)?
            .recv_json::<FundBuyResponse>()
            .await?;
        return Ok(res);
    }
}
