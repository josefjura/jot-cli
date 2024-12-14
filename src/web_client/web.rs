use async_trait::async_trait;

use crate::model::{DeviceCodeRequest, Token, TokenPollResponse};

use super::Client;

pub struct WebClient {
    server_url: String,
    client: reqwest::Client,
}

impl WebClient {
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Client for WebClient {
    async fn send_device_code(&self, device_code: &str) -> anyhow::Result<()> {
        let data = DeviceCodeRequest {
            device_code: device_code.to_string(),
        };

        let response = self
            .client
            .post(format!("{}/auth/device", self.server_url))
            .json(&data)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to register device");
        }

        Ok(())
    }

    async fn poll_for_token(&mut self, device_code: &str) -> anyhow::Result<TokenPollResponse> {
        let response = self
            .client
            .get(format!("{}/auth/status/{}", self.server_url, device_code))
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let token: Token = response.json().await?;
                Ok(TokenPollResponse::Success(token.access_token))
            }
            reqwest::StatusCode::ACCEPTED => Ok(TokenPollResponse::Pending),
            _ => anyhow::bail!("Authentication polling failed: {}", response.status()),
        }
    }

    fn get_server_url(&self) -> String {
        self.server_url.clone()
    }
}
