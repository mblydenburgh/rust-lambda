use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_json::{json, Value};
use lambda_runtime::{Error as LambdaError};
use uuid::Uuid;

use crate::{
    models::{
        user::AddUserEvent
    }
};

pub async fn create_user(client: &Client, payload: &str) -> Result<Value, LambdaError> {
    let uuid = Uuid::new_v4().to_string();
    println!("Creating user {}", &uuid);
    let add_user_event: AddUserEvent = serde_json::from_str(payload)?;
    println!("User from payload: {:?}", &add_user_event);
    let user_json = serde_json::to_value(&add_user_event).unwrap();
    println!("User json: {}", &user_json);

    let request = client
        .put_item()
        .table_name("rust-lambda-table")
        .item("userId", AttributeValue::S(String::from(&uuid)))
        .item(
            "modelTypeAndId",
            AttributeValue::S(format!("User#{}", String::from(&uuid))),
        )
        .item("first_name", AttributeValue::S(add_user_event.first_name))
        .item("last_name", AttributeValue::S(add_user_event.last_name));

    request.send().await?;
    Ok(json!(user_json))
}
