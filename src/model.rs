use serde::{Deserialize, Serialize};

pub struct LoginResponse {
    pub token: String,
}

pub enum TokenPollResponse {
    Pending,
    Success(String),
    Failure,
}

#[derive(Serialize)]
pub struct DeviceCodeRequest {
    pub device_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
}
