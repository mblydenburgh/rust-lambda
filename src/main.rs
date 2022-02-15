use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{handler_fn, Context, Error as LambdaError};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = handler_fn(handler_fun);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

async fn handler_fun(event: Value, _c: Context) -> Result<Value, LambdaError> {
    let uuid = Uuid::new_v4().to_string();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let user_event: CustomEvent = serde_json::from_value(event).unwrap();

    let request = client
        .put_item()
        .table_name("users")
        .item("uuid", AttributeValue::S(String::from(uuid)))
        .item(
            "first_name",
            AttributeValue::S(String::from(user_event.first_name)),
        )
        .item(
            "last_name",
            AttributeValue::S(String::from(user_event.last_name)),
        );

    request.send().await?;

    Ok(json!({"message": "Record written"}))
}
