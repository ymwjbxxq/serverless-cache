use std::collections::HashMap;
use aws_sdk_dynamodb::model::AttributeValue;
use crate::{error::ApplicationError, utils::dynamodb::AttributeValuesExt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cache {
    pub key: String,
    pub data: String,
    pub ttl: i64,
    pub client_cache: i64,
    pub cdn_cache: i64,
}

impl Cache {
  pub fn from_dynamodb(value: HashMap<String, AttributeValue>) -> Result<Cache, ApplicationError> {
    Ok(Cache {
      key:  value.get_string("pk").unwrap(),
      data: value.get_string("cache_data").unwrap(),
      ttl:  value.get_number("ttl_expire_at").unwrap(),
      client_cache:  value.get_number("client_cache").unwrap(),
      cdn_cache:  value.get_number("cdn_cache").unwrap(),
    })
  }
}