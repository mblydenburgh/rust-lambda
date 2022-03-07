use serde_json::Value;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{Error as LambdaError};
use log::{info};

use crate::{
    models::{
        user::User
    }
};

pub async fn get_user(client: &Client, id: &str) -> Result<Value, LambdaError> {
    info!("Getting user {}", id);

    let request = client
        .query()
        .table_name("rust-lambda-table")
        .key_condition_expression("#id = :uuid")
        .expression_attribute_names("#id", "userId")
        .expression_attribute_values(":uuid", AttributeValue::S(id.to_string()));

        let result = request.send().await?;
        let user = match result.items.unwrap().first() {
            Some(res) => User {
                uuid: res.get("userId").unwrap().as_s().unwrap().clone(),
                first_name: res.get("first_name").unwrap().as_s().unwrap().clone(),
                last_name: res.get("last_name").unwrap().as_s().unwrap().clone()
            },
            _ => User {
                uuid: String::from(""),
                first_name: String::from(""),
                last_name: String::from("")
            }
        };
        
        let user_json = serde_json::to_value(&user).unwrap();
        Ok(user_json)
}