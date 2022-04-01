use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct OriginResponse {
    pub key: String,
    pub data: String,
    pub cdn_cache: i64,
    pub client_cache: i64,
}