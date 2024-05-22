pub mod user;

use serde::{Deserialize, Serialize};

/// Default `T` is `HashMap`
#[derive(Serialize, Deserialize, Debug)]
pub struct KiteApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}
