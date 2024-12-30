use async_trait::async_trait;
use chrono::{NaiveDate, Utc};

use crate::{
    args::NoteSearchArgs,
    model::{GetNotesResponse, Note},
};

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

pub const MOCK_URL: &str = "mocked_instance";
pub const MOCK_TOKEN: &str = "mocked_token";

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
        self.response_counter = 0;

        println!(
            "Mocking polling for token with device code: {}",
            device_code
        );

        if self.response_counter == 1 {
            return Ok(crate::model::TokenPollResponse::Success(
                MOCK_TOKEN.to_string(),
            ));
        }

        self.response_counter += 1;

        Ok(crate::model::TokenPollResponse::Pending)
    }

    async fn create_note(
        &mut self,
        content: String,
        _tags: Vec<String>,
        date: NaiveDate,
    ) -> anyhow::Result<crate::model::Note> {
        let note = crate::model::Note {
            id: Some(1),
            content,
            tags: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            target_date: date,
        };

        Ok(note)
    }

    async fn ping(&self) -> anyhow::Result<()> {
        Ok(())
    }

    #[allow(clippy::unwrap_used)]
    async fn get_notes(&mut self) -> anyhow::Result<GetNotesResponse> {
        let notes = vec![
            Note {
                id: Some(1),
                tags: vec!["tag1".to_string(), "tag3".to_string()],
                created_at: chrono::DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
                    .unwrap()
                    .into(),
                updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
                    .unwrap()
                    .into(),
                content: "Short note".to_string(),
                target_date: Utc::now().naive_utc().date(),
            },
            Note {
                id: Some(2),
                tags: vec!["tag2".to_string(), "tag3".to_string()],
                created_at: chrono::DateTime::parse_from_rfc3339("2024-01-02T10:00:00Z")
                    .unwrap()
                    .into(),
                updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-02T11:00:00Z")
                    .unwrap()
                    .into(),
                content: "Multi-line note\nWith several\nDistinct lines\nTo test preview"
                    .to_string(),
                target_date: Utc::now().naive_utc().date(),
            },
            Note {
                id: Some(3),
                tags: vec!["tag3".to_string(), "tag4".to_string()],
                created_at: chrono::DateTime::parse_from_rfc3339("2024-01-03T10:00:00Z")
                    .unwrap()
                    .into(),
                updated_at: chrono::DateTime::parse_from_rfc3339("2024-01-03T10:00:00Z")
                    .unwrap()
                    .into(),
                content:
                    "Note with special formatting:\n* bullet point\n> quote\n```\ncode block\n```"
                        .to_string(),
                target_date: Utc::now().naive_utc().date(),
            },
        ];

        Ok(GetNotesResponse { notes })
    }

    async fn search(
        &mut self,
        _args: &NoteSearchArgs,
    ) -> anyhow::Result<crate::model::GetNotesResponse> {
        Ok(self.get_notes().await?)
    }

    fn get_server_url(&self) -> String {
        MOCK_URL.to_string()
    }

    async fn delete(&self, _ids: &[i64]) -> anyhow::Result<()> {
        Ok(())
    }
}
