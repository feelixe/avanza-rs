use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

use super::Account;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountListResponse(pub Vec<Account>);

impl Client {
    pub async fn list_accounts(&self) -> Result<AccountListResponse, Error> {
        let res = self
            .http_client
            .get(&self.config.urls.accounts)
            .recv_json::<AccountListResponse>()
            .await?;
        return Ok(res);
    }
}
