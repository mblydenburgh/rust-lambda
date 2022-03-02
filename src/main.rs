use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use lambda_http::{handler, Request};
use lambda_runtime::{Context, Error as LambdaError};
use serde_json::{json, Value};

mod models;
mod handlers;
use self::{
    models::{
        user::AddUserEvent
    },
    handlers::{
        create_user_handler::create_user,
        get_user_handler::get_user
    }
};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

async fn handler_func(event: Request, _c: Context) -> Result<Value, LambdaError> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let body_string: &str = match event.body() {
        lambda_http::Body::Text(text) => text.as_str(),
        _ => "",
    };

    let result = match event.method() {
        &lambda_http::http::method::Method::GET => {
            println!("Handling GET request");
            get_user(client, body_string).await?
        }
        &lambda_http::http::method::Method::POST => {
            println!("Handling POST request");
            create_user(client, body_string).await?
        }
        &lambda_http::http::method::Method::DELETE => {
            println!("Handling DELETE request");
            json!("delete")
        }
        &lambda_http::http::method::Method::PUT => {
            println!("Handling PUT request");
            json!("put")
        }
        _ => {
            println!("Handling other request");
            json!("other")
        }
    };
    

    Ok(json!(result))
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
