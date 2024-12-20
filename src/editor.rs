use std::{collections::HashSet, io::Read};

use anyhow::Context;
use chrono::NaiveDate;
use config::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditorTemplate {
    #[serde(default)]
    pub tags: HashSet<String>,
    pub date: Option<NaiveDate>,
    #[serde(default)]
    pub today: bool,
    #[serde(skip)]
    pub content: String,
}

impl EditorTemplate {
    pub fn new() -> Self {
        EditorTemplate {
            tags: HashSet::new(),
            date: None,
            today: false,
            content: String::new(),
        }
    }
}

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    fn read_from_file(&self, tempfile: tempfile::NamedTempFile) -> anyhow::Result<String> {
        // Read VISUAL or EDITOR environment variable
        let editor = std::env::var("VISUAL")
            .unwrap_or_else(|_| std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string()));

        let mut child = std::process::Command::new(editor)
            .arg(tempfile.path())
            .spawn()
            .context("Failed to open editor")?;

        let status = child.wait().context("Failed to wait for editor")?;

        if !status.success() {
            return Err(anyhow::anyhow!("Editor returned non-zero exit code"));
        }

        // Read content of the tempfile
        let mut content = String::new();
        let mut file = std::fs::File::open(tempfile.path())
            .context("Failed to open temporary file".to_string())?;
        file.read_to_string(&mut content)
            .context("Failed to read temporary file".to_string())?;

        Ok(content)
    }

    pub fn with_initial_content(&self, content: &str) -> anyhow::Result<String> {
        let mut tempfile =
            tempfile::NamedTempFile::new().context("Failed to create temporary file")?;

        // Write initial content
        std::io::Write::write_all(&mut tempfile, content.as_bytes())
            .context("Failed to write initial content")?;

        self.read_from_file(tempfile)
    }
}

pub trait ParseTemplate {
    fn parse_template(&self) -> anyhow::Result<EditorTemplate>;
}

impl ParseTemplate for String {
    fn parse_template(&self) -> anyhow::Result<EditorTemplate> {
        let parts: Vec<_> = self.split("+++").collect();

        let toml_string = parts[0];
        let mut parsed_toml = toml::from_str::<EditorTemplate>(toml_string)?;

        if parts.len() > 1 {
            parsed_toml.content = parts[1].trim().to_string();
        }

        Ok(parsed_toml)
    }
}
