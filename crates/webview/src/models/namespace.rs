pub mod register;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}
