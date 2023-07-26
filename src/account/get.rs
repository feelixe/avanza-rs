use crate::client::Client;
use crate::error::Error;

use super::Account;

impl Client {
    pub async fn get_acccount(&self, account_id: &str) -> Result<Account, Error> {
        let res = self.list_accounts().await?;
        let account = res
            .0
            .into_iter()
            .find(|account| return account.id == account_id);
        match account {
            Some(account) => {
                return Ok(account);
            }
            None => {
                return Err(Error::AccountNotFound(String::from(
                    "Account with id not found",
                )))
            }
        }
    }
}
