use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{args::NoteAddArgs, utils::date::date_value::DateValue};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditorTemplate {
    #[serde(default)]
    pub tags: HashSet<String>,
    #[serde(default)]
    pub date: Option<DateValue>,
    #[serde(skip)]
    pub content: String,
}

impl Default for EditorTemplate {
    fn default() -> Self {
        Self {
            tags: HashSet::new(),
            date: Some(DateValue::Today),
            content: "".to_string(),
        }
    }
}

pub struct Editor {
    template: String,
}

impl Editor {
    pub fn new(template: &str) -> Self {
        Editor {
            template: template.to_string(),
        }
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

    pub fn open(&self, args: &NoteAddArgs) -> anyhow::Result<EditorTemplate> {
        print!("\x1B[?1049h");
        io::stdout().flush()?;
        let content = self
            .with_initial_content(&self.template, &args.content.join(" "))?
            .parse_template()?;

        // Restore state and ensure buffer is cleared properly
        print!("\x1B[?1049l\x1B[H\x1B[2J");
        io::stdout().flush()?; // Important to flush here too

        Ok(content)
    }

    pub fn with_initial_content(&self, template: &str, _content: &str) -> anyhow::Result<String> {
        let mut tempfile =
            tempfile::NamedTempFile::new().context("Failed to create temporary file")?;

        // Write initial content
        std::io::Write::write_all(&mut tempfile, template.as_bytes())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_template() {
        let template = r#"tags = ["work", "important"]
#tags = []
date = "today"
#date = "YYYY-MM-DD"
+++
Some content"#
            .to_string();

        let parsed = template.parse_template().unwrap();

        assert_eq!(parsed.tags.len(), 2);
        assert_eq!(parsed.content, "Some content");
    }

    #[test]
    fn test_parse_template_no_content() {
        let template = r#"tags = ["work", "important"]
#tags = []
date = "today"
#date = "YYYY-MM-DD"
+++"#
            .to_string();

        let parsed = template.parse_template().unwrap();

        assert_eq!(parsed.tags.len(), 2);
        assert_eq!(parsed.content, "");
    }

    #[test]
    fn test_parse_template_no_tags() {
        let template = r#"date = "today"
#date = "YYYY-MM-DD"
+++
Some content"#
            .to_string();

        let parsed = template.parse_template().unwrap();

        assert_eq!(parsed.tags.len(), 0);
        assert_eq!(parsed.content, "Some content");
    }

    #[test]
    fn test_parse_template_no_date() {
        let template = r#"tags = ["work", "important"]
#tags = []
+++
Some content"#
            .to_string();

        let parsed = template.parse_template().unwrap();

        assert_eq!(parsed.tags.len(), 2);
        assert_eq!(parsed.content, "Some content");
    }
}
