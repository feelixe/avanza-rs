use serde::{Deserialize, Serialize};

pub(crate) mod get;
pub(crate) mod list;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub url_parameter_id: String,
}
