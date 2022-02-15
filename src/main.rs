use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{handler, Request};
use lambda_runtime::{Context, Error as LambdaError};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda_runtime::run(handler(handler_fun)).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

async fn handler_fun(event: Request, _c: Context) -> Result<Value, LambdaError> {
    println!("event received: {:?}", event);
    let uuid = Uuid::new_v4().to_string();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let body_string: &str = match event.body() {
        lambda_http::Body::Text(text) => text.as_str(),
        _ => "",
    };
    println!("Body string: {}", body_string);

    let custom_event: CustomEvent = serde_json::from_str(body_string)?;
    println!("serialized event: {:?}", custom_event);

    let request = client
        .put_item()
        .table_name("users")
        .item("uuid", AttributeValue::S(uuid))
        .item(
            "first_name",
            AttributeValue::S(custom_event.first_name.into()),
        )
        .item(
            "last_name",
            AttributeValue::S(custom_event.last_name.into()),
        );

    println!("executing request {:?}", request);
    request.send().await?;

    Ok(json!({"message": "Record written"}))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_payload_to_event() {
        let body_string = r#"
        {
            "first_name":"first",
            "last_name":"last"
            }
        "#;

        let event: CustomEvent = serde_json::from_str(body_string).unwrap();

        assert!(event.first_name == "first");
        assert!(event.last_name == "last");
    }
}
