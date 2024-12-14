use serde::{Deserialize, Serialize};

#[expect(dead_code)]
pub struct LoginResponse {
    pub token: String,
}

pub enum TokenPollResponse {
    Pending,
    Success(String),
    #[expect(dead_code)]
    Failure(String),
}

#[derive(Serialize)]
pub struct DeviceCodeRequest {
    pub device_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
}
