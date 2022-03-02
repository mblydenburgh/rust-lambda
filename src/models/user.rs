use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddUserEvent {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
}