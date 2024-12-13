use async_trait::async_trait;

use crate::{app_config::AppConfig, model::TokenPollResponse};

#[cfg(debug_assertions)]
mod mock;

mod web;

// In your client code
#[cfg(debug_assertions)]
pub fn get_client(config: &AppConfig) -> Box<dyn Client> {
    use mock::MockClient;
    use web::WebClient;

    if config.mock_server {
        println!("Mocking server requests");
        Box::new(MockClient::new())
    } else {
        Box::new(WebClient::new(config.server_url.clone()))
    }
}

#[cfg(not(debug_assertions))]
pub fn get_client(config: &AppConfig) -> Box<dyn Client> {
    use web::WebClient;

    Box::new(WebClient::new(config.server_url.clone()))
}

#[async_trait]
pub trait Client {
    async fn send_device_code(&self, device_code: &str) -> anyhow::Result<()>;
    async fn poll_for_token(&mut self, device_code: &str) -> anyhow::Result<TokenPollResponse>;
}
