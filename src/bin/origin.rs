use lambda_http::{
    http::{self, StatusCode},
    service_fn, Error, IntoResponse, Request, RequestExt,
};
use serverless_cache::{dtos::origin_response::OriginResponse, utils::api_helper::ApiHelper, error::ApplicationError};
use std::{collections::HashMap, thread, time};
use rand::Rng;

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
    
    let response = OriginResponse {
        key: event
            .query_string_parameters()
            .first("key")
            .unwrap()
            .to_string(),
        data: fake_call_to_db(),
        cdn_cache: 3600,
        client_cache: 10,
    };

    // not necessary in the GET of the origin, but I could use a param to force the call to cache service
    //post_to_cache_service(&response).await?;

    let mut headers = HashMap::new();
    headers.insert(http::header::CONTENT_TYPE, "application/json".to_string());
    Ok(ApiHelper::response(
        StatusCode::OK,
        serde_json::to_string(&response)?,
        headers,
    ))
}

// async fn post_to_cache_service(response: &OriginResponse) -> Result<(), ApplicationError> {
//     let url = std::env::var("CACHE_ENDPOINT").expect("CACHE_ENDPOINT must be set");
//     let client = reqwest::Client::new();
//     let _res = client.post(url)
//         .json(response)
//         .send()
//         .await?;
//     Ok(())
// }

fn fake_call_to_db() -> String {
    //simulate a call to a database
    let mut rng = rand::thread_rng();
    let milliseconds = rng.gen_range(20..1000);
    thread::sleep(time::Duration::from_millis(milliseconds));

    // response from a database
    let response = r#"{
        "_id": "6245cc2fdd635f58f53cb3e8",
        "index": 3,
        "guid": "337dda62-8a6a-4804-bb83-560e5539ebd6",
        "isActive": false,
        "balance": "$3,336.43",
        "picture": "http://placehold.it/32x32",
        "age": 31,
        "eyeColor": "green",
        "name": "Huffman Baldwin",
        "gender": "male",
        "company": "ZERBINA",
        "email": "huffmanbaldwin@zerbina.com",
        "phone": "+1 (973) 569-3138",
        "address": "358 Ainslie Street, Wintersburg, Maine, 1161",
        "about": "Ex fugiat do magna duis magna cillum laborum officia. Dolor tempor consectetur mollit amet consectetur incididunt nisi incididunt nostrud aliquip anim. Officia enim id velit aute et dolor esse exercitation veniam aute labore sit occaecat. Est sint irure duis velit Lorem Lorem aliqua labore. Nulla eiusmod nulla incididunt cillum enim laboris. Non et exercitation esse enim laboris ut pariatur nostrud. Id excepteur id anim consequat irure labore proident consequat pariatur aliqua deserunt mollit.\r\n",
        "registered": "2019-01-12T02:42:52 -01:00",
        "latitude": 25.581892,
        "longitude": 174.839012,
        "tags": [
          "id",
          "ad",
          "consequat",
          "nostrud",
          "incididunt",
          "nostrud",
          "fugiat"
        ],
        "friends": [
          {
            "id": 0,
            "name": "Floyd Richard"
          },
          {
            "id": 1,
            "name": "Tate Gill"
          },
          {
            "id": 2,
            "name": "Mays Cruz"
          }
        ],
        "greeting": "Hello, Huffman Baldwin! You have 2 unread messages.",
        "favoriteFruit": "banana"
      }"#;
    response.to_string()
}
