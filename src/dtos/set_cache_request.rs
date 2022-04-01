use serde::{Deserialize, Deserializer};

const ONE_HOUR: i64 = 3600;
const TEN_SECONDS: i64 = 10;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetCacheRequest {
    pub key: String,

    #[serde(deserialize_with = "any_to_string")]
    pub data: String,

    #[serde(rename = "cdnTTL")]
    #[serde(default = "one_hour")]
    pub cdn_ttl: i64,

    #[serde(rename = "clientTTL")]
    #[serde(default = "ten_seconds")]
    pub client_tll: i64,
}

fn one_hour() -> i64{
  ONE_HOUR
}
fn ten_seconds() -> i64{
  TEN_SECONDS
}

fn any_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{

    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Bool(x) => x.to_string(),
        serde_json::Value::Number(x) => x.to_string(),
        serde_json::Value::String(x) => x,
        serde_json::Value::Array(x) => serde_json::to_string(&x).unwrap(),
        serde_json::Value::Object(x) => serde_json::to_string(&x).unwrap(),
        _ => String::from(""),
    })
}