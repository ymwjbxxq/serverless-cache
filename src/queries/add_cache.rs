use crate::{dtos::{ set_cache_request::SetCacheRequest}, error::ApplicationError};
use aws_sdk_dynamodb::{Client, model::{AttributeValue, ReturnValue}};
use async_trait::async_trait;

#[async_trait]
pub trait AddCacheQuery {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, product: &SetCacheRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct AddCache {
  table_name: String,
}

#[async_trait]
impl AddCacheQuery for AddCache {
  async fn new() -> Self {
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    Self { table_name }
  }

  async fn execute(&self, client: &Client, request: &SetCacheRequest) -> Result<(), ApplicationError> {
      let ttl = (chrono::Utc::now() + chrono::Duration::seconds(request.cdn_ttl)).timestamp();
      client
            .update_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(request.key.clone()))
            .update_expression("SET cache_data = :cache_data, ttl_expire_at = :ttl_expire_at, client_cache = :client_cache, cdn_cache = :cdn_cache")
            .expression_attribute_values(":cache_data",AttributeValue::S(request.data.clone()))
            .expression_attribute_values(":ttl_expire_at",AttributeValue::N(format!("{:?}", ttl)))
            .expression_attribute_values(":client_cache",AttributeValue::N(format!("{:?}", request.client_tll)))
            .expression_attribute_values(":cdn_cache",AttributeValue::N(format!("{:?}", request.cdn_ttl)))
            .return_values(ReturnValue::UpdatedNew)
            .send()
            .await?;

      Ok(())
  }
}
