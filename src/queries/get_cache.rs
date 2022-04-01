use crate::{
    error::ApplicationError, models::cache::Cache,
};
use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client};

#[async_trait]
pub trait GetCacheQuery {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, slug: &str) -> Result<Option<Cache>, ApplicationError>;
}

#[derive(Debug)]
pub struct GetCache {
    table_name: String,
}

#[async_trait]
impl GetCacheQuery for GetCache {
    async fn new() -> Self {
        let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
        Self { table_name }
    }

    async fn execute(&self, client: &Client, key: &str) -> Result<Option<Cache>, ApplicationError> {
        let result = client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(key.to_owned()))
            .send()
            .await?;

        Ok(match result.item {
          None => None,
          Some(item) => Some(Cache::from_dynamodb(item)?),
        })
    }
}
