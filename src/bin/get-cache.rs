use std::collections::HashMap;
use lambda_http::{
    http::{StatusCode, self}, service_fn, Error, IntoResponse, Request, RequestExt,
};
use serverless_cache::{
    queries::get_cache::{GetCache, GetCacheQuery},
    utils::api_helper::ApiHelper, dtos::origin_response::OriginResponse, error::ApplicationError,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    lambda_http::run(service_fn(|event: Request| {
        execute(&dynamodb_client, event)
    }))
    .await?;
    Ok(())
}

pub async fn execute(client: &aws_sdk_dynamodb::Client, event: Request) -> Result<impl IntoResponse, Error> {
    println!("EVENT {:?}", event);

    let value = try_get_value(client, event).await?;

    let mut headers = HashMap::new();
    headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
    headers.insert(http::header::CACHE_CONTROL, format!("public, max-age={}, s-maxage={};", value.client_cache, value.cdn_cache));

    let mut status_code = StatusCode::NOT_FOUND;
    if value.data != "" {
        status_code = StatusCode::OK;
    }

    Ok(ApiHelper::response(
        status_code,
        value.data,
        headers,
    ))
}

async fn try_get_value(client: &aws_sdk_dynamodb::Client, event: Request) -> Result<OriginResponse, ApplicationError> {
    let mut response: OriginResponse = Default::default();
    if let Some(cache_key) = event.query_string_parameters().first("key") {
        response.key = cache_key.to_string();
        let cache = GetCache::new().await.execute(client, &cache_key).await?;
        if let Some(cache) = cache {
            if cache.ttl > chrono::Utc::now().timestamp() {
                response.data = cache.data;
                response.client_cache = cache.client_cache;
                response.cdn_cache = cache.cdn_cache;
            }
        } else {
            let origin_response = call_origin(event).await?;
            if let Some(origin_response) = origin_response {
                response.data = origin_response.data;
                response.client_cache = origin_response.client_cache;
                response.cdn_cache = origin_response.cdn_cache;
            }
        }
    }
    
    Ok(response)
}

async fn call_origin(event: Request) -> Result<Option<OriginResponse>, ApplicationError> {
    if let Some(origin_url) = event.query_string_parameters().first("origin_url") {
        let origin_response = reqwest::get(origin_url)
                                .await?
                                .json::<OriginResponse>()
                                .await?;
        return Ok(Some(origin_response));
    }

    Ok(None)
}