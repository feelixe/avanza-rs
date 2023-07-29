use crate::client::Client;
use serde::{Deserialize, Serialize};

use crate::{account::Account, char_encode::char_encode, error::Error};

#[derive(Debug, Serialize)]
struct TotalValuesRequest(Vec<String>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalValuesResponse {
    #[serde(rename(serialize = "total_value"))]
    pub aggegated: Aggegated,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggegated {
    pub total_value: TotalValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalValue {
    pub value: f64,
}

impl Client {
    fn encode_account_ids(accounts: &Vec<Account>) -> Result<Vec<String>, Error> {
        let encoded_account_ids: Result<Vec<_>, _> = accounts
            .iter()
            .map(|acc| {
                return acc
                    .id
                    .parse::<u32>()
                    .map_err(|err| Error::AccountIdParseError(String::from("asd"), err))
                    .map(char_encode);
            })
            .collect();
        return encoded_account_ids;
    }

    pub async fn get_total_values(
        &self,
        accounts: &Vec<Account>,
    ) -> Result<TotalValuesResponse, Error> {
        let encoded_account_ids = Self::encode_account_ids(accounts)?;

        let request_body = TotalValuesRequest(encoded_account_ids);

        let res = self
            .http_client
            .post(&self.config.urls.total_values)
            .body_json(&request_body)?
            .recv_json::<TotalValuesResponse>()
            .await?;

        return Ok(res);
    }
}
