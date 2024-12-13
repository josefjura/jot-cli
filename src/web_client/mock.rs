use async_trait::async_trait;

use super::Client;

#[derive(Debug, Clone, Copy)]
pub struct MockClient {
    response_counter: u16,
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            response_counter: 0,
        }
    }
}

#[async_trait]
impl Client for MockClient {
    async fn send_device_code(&self, device_code: &str) -> anyhow::Result<()> {
        println!("Mocking sending device code: {}", device_code);
        Ok(())
    }

    async fn poll_for_token(
        &mut self,
        device_code: &str,
    ) -> anyhow::Result<crate::model::TokenPollResponse> {
        println!(
            "Mocking polling for token with device code: {}",
            device_code
        );

        if self.response_counter == 2 {
            return Ok(crate::model::TokenPollResponse::Success(
                "mock_token".to_string(),
            ));
        }

        self.response_counter += 1;

        Ok(crate::model::TokenPollResponse::Pending)
    }
}
