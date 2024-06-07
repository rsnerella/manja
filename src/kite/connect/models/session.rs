use secrecy::{ExposeSecret, Secret};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    demat_consent: String,
}

#[derive(Clone, Debug)]
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
    pub api_key: Secret<String>,
    pub access_token: Secret<String>,
    pub public_token: Secret<String>,
    pub refresh_token: Secret<String>,
    pub enctoken: Secret<String>,
    pub login_time: String,
    pub meta: Option<Meta>,
}

// Implement Serialize for UserSession
impl Serialize for UserSession {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UserSession", 16)?;
        state.serialize_field("user_type", &self.user_type)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("user_name", &self.user_name)?;
        state.serialize_field("user_shortname", &self.user_shortname)?;
        state.serialize_field("broker", &self.broker)?;
        state.serialize_field("exchanges", &self.exchanges)?;
        state.serialize_field("products", &self.products)?;
        state.serialize_field("order_types", &self.order_types)?;
        state.serialize_field("avatar_url", &self.avatar_url)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("api_key", self.api_key.expose_secret())?;
        state.serialize_field("access_token", self.access_token.expose_secret())?;
        state.serialize_field("public_token", self.public_token.expose_secret())?;
        state.serialize_field("refresh_token", self.refresh_token.expose_secret())?;
        state.serialize_field("enctoken", self.enctoken.expose_secret())?;
        state.serialize_field("login_time", &self.login_time)?;
        state.serialize_field("meta", &self.meta)?;
        state.end()
    }
}

// Implement Deserialize for UserSession
impl<'de> Deserialize<'de> for UserSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct UserSessionFields {
            user_type: String,
            email: String,
            user_name: String,
            user_shortname: String,
            broker: String,
            exchanges: Vec<String>,
            products: Vec<String>,
            order_types: Vec<String>,
            avatar_url: Option<String>,
            user_id: String,
            api_key: String,
            access_token: String,
            public_token: String,
            refresh_token: String,
            enctoken: String,
            login_time: String,
            meta: Option<Meta>,
        }

        let fields = UserSessionFields::deserialize(deserializer)?;

        Ok(UserSession {
            user_type: fields.user_type,
            email: fields.email,
            user_name: fields.user_name,
            user_shortname: fields.user_shortname,
            broker: fields.broker,
            exchanges: fields.exchanges,
            products: fields.products,
            order_types: fields.order_types,
            avatar_url: fields.avatar_url,
            user_id: fields.user_id,
            api_key: Secret::new(fields.api_key),
            access_token: Secret::new(fields.access_token),
            public_token: Secret::new(fields.public_token),
            refresh_token: Secret::new(fields.refresh_token),
            enctoken: Secret::new(fields.enctoken),
            login_time: fields.login_time,
            meta: fields.meta,
        })
    }
}
