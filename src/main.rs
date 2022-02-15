use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{Context, Error as LambdaError};
use lambda_http::{handler, Request, RequestExt};
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda_runtime::run(handler(handler_fun)).await?;
    Ok(())
}

async fn handler_fun(event: Request, _c: Context) -> Result<Value, LambdaError> {
    println!("event received: {:?}", event);
    let uuid = Uuid::new_v4().to_string();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let params = event.query_string_parameters();
    let first_name = params.get("first_name").unwrap();
    let last_name = params.get("last_name").unwrap();

    let request = client
        .put_item()
        .table_name("users")
        .item("uuid", AttributeValue::S(String::from(uuid)))
        .item(
            "first_name",
            AttributeValue::S(String::from(first_name)),
        )
        .item(
            "last_name",
            AttributeValue::S(String::from(last_name)),
        );

    request.send().await?;

    Ok(json!({"message": "Record written"}))
}
