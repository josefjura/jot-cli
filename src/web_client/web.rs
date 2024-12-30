use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use serde::Serialize;
use serde_json::json;

use crate::{
    args::NoteSearchArgs,
    model::{DeviceCodeRequest, GetNotesResponse, Note, Token, TokenPollResponse},
    utils::date::date_filter::DateFilter,
};

use super::Client;

pub struct WebClient {
    server_url: String,
    token: Option<String>,
    client: reqwest::Client,
}

impl WebClient {
    pub fn new(server_url: String, token: Option<String>) -> Self {
        Self {
            server_url,
            token,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Client for WebClient {
    async fn ping(&self) -> anyhow::Result<()> {
        let real_token = match self.token {
            Some(ref token) => token,
            None => anyhow::bail!("No token available"),
        };

        let response = self
            .client
            .get(format!("{}/health/auth", self.server_url))
            .header("Authorization", format!("Bearer {}", real_token))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Cannot verify login");
        }

        Ok(())
    }

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

    async fn create_note(
        &mut self,
        content: String,
        tags: Vec<String>,
        date: NaiveDate,
    ) -> anyhow::Result<Note> {
        let real_token = match self.token {
            Some(ref token) => token,
            None => anyhow::bail!("No token available"),
        };

        let response = self
            .client
            .post(format!("{}/note", self.server_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", real_token))
            .json(&json!({
                "content": content,
                "tags": tags,
                "target_date": date
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            println!("{:?}", response.text().await);
            anyhow::bail!("Failed to create note");
        }

        let json = response.json::<Note>().await?;

        Ok(Note {
            id: json.id,
            content: json.content,
            tags: json.tags,
            created_at: json.created_at,
            updated_at: json.updated_at,
            target_date: json.target_date,
        })
    }

    async fn get_notes(&mut self) -> anyhow::Result<GetNotesResponse> {
        let real_token = match self.token {
            Some(ref token) => token,
            None => anyhow::bail!("No token available"),
        };

        let response = self
            .client
            .get(format!("{}/note", self.server_url))
            .header("Authorization", format!("Bearer {}", real_token))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get notes");
        }

        // DEBUG RESPONSE
        // let bytes = response.bytes().await?;
        // println!("{:?}", std::str::from_utf8(&bytes)?);

        // let notes: GetNotesResponse = serde_json::from_slice(&bytes)?;
        let notes = response.json::<Vec<Note>>().await?;

        Ok(GetNotesResponse { notes })
    }

    async fn search(&mut self, args: &NoteSearchArgs) -> anyhow::Result<GetNotesResponse> {
        let real_token = match self.token {
            Some(ref token) => token,
            None => anyhow::bail!("No token available"),
        };

        let target_date = args
            .date
            .clone()
            .and_then(|d| d.search_for_day(Utc::now().date_naive()));

        let created_at = args
            .created
            .clone()
            .and_then(|d| d.search_for_day(Utc::now().date_naive()));

        let updated_at = args
            .updated
            .clone()
            .and_then(|d| d.search_for_day(Utc::now().date_naive()));

        let body = SearchRequest {
            tag: args.tag.clone(),
            term: args.term.clone(),
            limit: args.limit,
            target_date,
            created_at,
            updated_at,
        };

        let response = self
            .client
            .post(format!("{}/note/search", self.server_url))
            .json(&body)
            .bearer_auth(real_token)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to seaarch for notes, {}", response.text().await?);
        }

        let notes = response.json::<Vec<Note>>().await?;

        Ok(GetNotesResponse { notes })
    }

    fn get_server_url(&self) -> String {
        self.server_url.clone()
    }

    async fn delete(&self, ids: &[i64]) -> anyhow::Result<()> {
        let real_token = match self.token {
            Some(ref token) => token,
            None => anyhow::bail!("No token available"),
        };

        let _response = self
            .client
            .post(format!("{}/note/delete", self.server_url))
            .json(ids)
            .bearer_auth(real_token)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub term: Option<String>,
    pub limit: Option<i64>,
    pub tag: Vec<String>,
    pub target_date: Option<SearchRequestDate>,
    pub created_at: Option<SearchRequestDate>,
    pub updated_at: Option<SearchRequestDate>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SearchRequestDate {
    Single(NaiveDate),
    Range {
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<NaiveDate>,
    },
}

impl DateFilter {
    pub fn search_for_day(&self, today: NaiveDate) -> Option<SearchRequestDate> {
        match self {
            DateFilter::SpecificDate(date) => date.to_date(today).map(SearchRequestDate::Single),
            DateFilter::Range(from, until) => {
                let from = from.to_date(today);
                let until = until.to_date(today);

                Some(SearchRequestDate::Range { from, until })
            }
        }
    }
}
