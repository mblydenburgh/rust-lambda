use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_json::{json, Value};
use lambda_runtime::{Error as LambdaError};
use uuid::Uuid;

use crate::{
    models::{
        user::{AddUserEvent, User}
    }
};

pub async fn create_user(client: &Client, payload: &str) -> Result<Value, LambdaError> {
    let uuid = Uuid::new_v4().to_string();
    println!("Creating user {}", &uuid);
    let add_user_event: AddUserEvent = serde_json::from_str(payload)?;
    
    let request = client
        .put_item()
        .table_name("rust-lambda-table")
        .item("userId", AttributeValue::S(String::from(&uuid)))
        .item(
            "modelTypeAndId",
            AttributeValue::S(format!("User#{}", String::from(&uuid))),
        )
        .item("first_name", AttributeValue::S(add_user_event.first_name.clone()))
        .item("last_name", AttributeValue::S(add_user_event.last_name.clone()));

    
    request.send().await?;
    let created_user = User {
        uuid: uuid,
        first_name: add_user_event.first_name,
        last_name: add_user_event.last_name
    };
    let user_json = serde_json::to_value(&created_user).unwrap();
    Ok(json!(user_json))
}
