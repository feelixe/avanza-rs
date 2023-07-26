use serde::{Deserialize, Serialize};

pub mod get;
pub mod list;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub url_parameter_id: String,
}
