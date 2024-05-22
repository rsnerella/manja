use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    demat_consent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSession {
    pub user_type: String,
    pub email: String,
    pub user_name: String,
    pub user_shortname: String,
    pub broker: String,
    pub exchanges: Vec<String>,
    pub products: Vec<String>,
    pub order_types: Vec<String>,
    pub avatar_url: Option<String>,
    pub user_id: String,
    pub api_key: String,
    pub access_token: String,
    pub public_token: String,
    pub refresh_token: String,
    pub enctoken: String,
    pub login_time: String,
    pub meta: Option<Meta>,
}
