use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockListFilter {
    pub country_codes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockListSortBy {
    pub field: String,
    pub order: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockListRequest {
    pub filter: StockListFilter,
    pub limit: u32,
    pub offset: u32,
    pub sort_by: StockListSortBy,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub orderbook_id: String,
    pub r#type: String,
    pub name: String,
    pub short_name: String,
    pub currency: String,
    pub country_code: String,
    pub direct_yield: f64,
    pub price_earnings_ratio: f64,
    pub number_of_owners: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockListResponsePagination {
    pub offset: u32,
    pub limit: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockListResponse {
    pub stocks: Vec<Instrument>,
    pub total_number_of_orderbooks: u32,
    pub pagination: StockListResponsePagination,
}

impl Client {
    pub async fn get_stock_list(
        &self,
        request: &StockListRequest,
    ) -> Result<StockListResponse, Error> {
        let res = self
            .http_client
            .post(&self.config.urls.stock_list)
            .body_json(&request)?
            .recv_json::<StockListResponse>()
            .await?;
        return Ok(res);
    }
}
