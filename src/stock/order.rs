use std::fmt;

use crate::{account::Account, client::Client, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderRequestStatus {
    SUCCESS,
    ERROR,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockOrderUnknownResponse {
    pub order_request_status: OrderRequestStatus,
    pub message: String,
    pub order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockOrderResponse {
    pub message: String,
    pub order_id: String,
}

impl Into<StockOrderResponse> for StockOrderUnknownResponse {
    fn into(self) -> StockOrderResponse {
        StockOrderResponse {
            message: self.message,
            order_id: self.order_id.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockOrderRequest {
    pub orderbook_id: String,
    pub account_id: String,
    pub price: f64,
    pub volume: u32,
    pub side: String,
}

pub struct StockOrder {
    pub orderbook_id: String,
    pub account: Account,
    pub price: f64,
    pub volume: u32,
    pub side: Side,
}

pub enum Side {
    BUY,
    SELL,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::BUY => write!(f, "BUY"),
            Side::SELL => write!(f, "SELL"),
        }
    }
}

impl Client {
    pub async fn new_stock_order(
        &self,
        order: &StockOrder,
    ) -> Result<StockOrderResponse, Error> {
        let order_request = StockOrderRequest {
            orderbook_id: order.orderbook_id.clone(),
            account_id: order.account.id.clone(),
            volume: order.volume,
            price: order.price,
            side: order.side.to_string(),
        };
        let res = self
            .http_client
            .post(&self.config.urls.stock_order)
            .body_json(&order_request)?
            .recv_json::<StockOrderUnknownResponse>()
            .await?;

        if res.order_request_status == OrderRequestStatus::ERROR {
            return Err(Error::StockOrderError(format!(
                "Stock order failed with message: '{}'",
                res.message
            )));
        }

        let response: StockOrderResponse = res.into();

        return Ok(response);
    }
}
