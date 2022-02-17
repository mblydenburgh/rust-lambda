use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_http::{handler, Request};
use lambda_runtime::{Context, Error as LambdaError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddUserEvent {
    first_name: String,
    last_name: String,
}

async fn handler_func(event: Request, _c: Context) -> Result<Value, LambdaError> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let body_string: &str = match event.body() {
        lambda_http::Body::Text(text) => text.as_str(),
        _ => "",
    };
    let add_user_event: AddUserEvent = serde_json::from_str(body_string)?;
    let user_json = serde_json::to_value(&add_user_event).unwrap();
    let uuid = Uuid::new_v4().to_string();

    let request = client
        .put_item()
        .table_name("rust-lambda-table")
        .item("uuid", AttributeValue::S(uuid))
        .item(
            "first_name",
            AttributeValue::S(add_user_event.first_name),
        )
        .item(
            "last_name",
            AttributeValue::S(add_user_event.last_name),
        );

    
    request.send().await?;
    Ok(json!(user_json))
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

        let event: AddUserEvent = serde_json::from_str(body_string).unwrap();

        assert!(event.first_name == "first");
        assert!(event.last_name == "last");
    }
}
