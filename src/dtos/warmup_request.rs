use serde::Deserialize;
use crate::error::ApplicationError;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct WarmUpCachBuilder {
    pub endpoint: String,

    pub key: String,

    pub cdn_ttl: i64,

    pub client_tll: i64,
}

impl WarmUpCachBuilder {
    pub fn new() -> WarmUpCachBuilder {
       let options: WarmUpCachBuilder = Default::default();
       options
    }

     pub fn set_endpoint(mut self, input: String) -> WarmUpCachBuilder {
        self.endpoint = input;
        self
    }

    pub fn set_key(mut self, input: String) -> WarmUpCachBuilder {
        self.key = input;
        self
    }

    pub fn set_cdn_cache_seconds(mut self, input: i64) -> WarmUpCachBuilder {
        self.cdn_ttl = input;
        self
    }

    pub fn set_client_cache_seconds(mut self, input: i64) -> WarmUpCachBuilder {
        self.client_tll = input;
        self
    }

    pub async fn warm_up(self) -> Result<(), ApplicationError> {
        let url = format!("https://{}/cache?key={}&client_cache={}&cdn_cache={}",
            self.endpoint,
            self.key,
            self.client_tll,
            self.cdn_ttl,
        );
        let _res = reqwest::get(url).await?;
        Ok(())
    }
}