use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use serverless_cache::{dtos::{batch_item_failures::{BatchItemFailures, ItemIdentifier}, warmup_request::WarmUpCachBuilder, set_cache_request::SetCacheRequest}, queries::add_cache::{AddCache, AddCacheQuery}};
use std::sync::{Arc, Mutex};
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    lambda_runtime::run(service_fn(|event: LambdaEvent<SqsEvent>| {
        execute(&dynamodb_client, event)
    }))
    .await?;
    Ok(())
}


pub async fn execute(client: &aws_sdk_dynamodb::Client, event: LambdaEvent<SqsEvent>) -> Result<Value, Error> {
    println!("EVENT {:?}", event);
    let failed_message: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut tasks = Vec::with_capacity(event.payload.records.len());
    let shared_client = Arc::from(client.clone());

    for record in event.payload.records.into_iter() {
        let shared_client = shared_client.clone();
        let message_id = record.message_id.unwrap();
        let failed_message = failed_message.clone();

        tasks.push(tokio::spawn(async move {
            if let Some(body) = &record.body {
                let request = serde_json::from_str::<SetCacheRequest>(&body);
                if let Ok(request) = request {
                    AddCache::new()
                        .await
                        .execute(&shared_client, &request)
                        .await
                        .map_or_else(|e| {
                          println!("ERROR {:?}", e);
                          failed_message.lock().unwrap().push(message_id.clone());
                        }, |_| ());

/* ##########################################################################
This works in this Lambda only if I add a custom domain to CLoudFront otherwise I would create a circular dependency because the comain name is not known at Lambda creation time.
Alternatively I could emit an event to a SNS topic and have a different Lambda to handle the SNS event and  do the cache warm up.
##########################################################################   */                    
                    WarmUpCachBuilder::new()
                        .set_endpoint(std::env::var("DISTRIBUTION_CUSTOM_DOMAIN_NAME").expect("DISTRIBUTION_CUSTOM_DOMAIN_NAME must be set"))
                        .set_key(request.key.clone())
                        .set_client_cache_seconds(request.client_tll.clone())
                        .set_cdn_cache_seconds(request.cdn_ttl.clone())
                        .warm_up()
                        .await
                        .map_or_else(|e| {
                          println!("WARM UP ERROR {:?}", e);
                        }, |_| ());
                } 
            }
        }));
    }

    join_all(tasks).await;

    let response = BatchItemFailures {
        batch_item_failures: failed_message.lock().unwrap().clone()
            .into_iter()
            .map(|message_id| {
                return ItemIdentifier {
                    item_identifier: message_id,
                };
            })
            .collect(),
    };
    Ok(serde_json::to_value(response).unwrap())
}
