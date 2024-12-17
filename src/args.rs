use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use toml::value::Date;

use crate::utils::date_target::DateTarget;

#[derive(Parser, Debug)]
#[command(
    name = "jot",
    version,
    about,
    long_about = "Simple CLI for jotting down notes"
)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub config: ConfigArgs,
}

#[derive(Debug, Args, Serialize)]
pub struct ConfigArgs {
    /// Mock server requests
    #[cfg(debug_assertions)]
    #[arg(long, short, default_value_t = false)]
    pub mock: bool,

    /// Parameter for mock specification
    #[cfg(debug_assertions)]
    #[arg(long)]
    pub mock_param: Option<String>,

    /// Mock server requests
    #[arg(long, short, env = "JOT_PROFILE")]
    pub profile_path: Option<String>,

    /// Mock server requests
    #[arg(long, short)]
    pub server_url: Option<String>,
}

#[derive(Debug, Subcommand, Serialize, PartialEq)]
pub enum Command {
    /// Authenticates user against server
    Login,
    /// Prints out curent configuration
    Config,
    /// Initializes a new profile
    Init,
    /// Notes subcommands
    #[clap(subcommand)]
    Note(NoteCommand),
    /// Creates a new note. Alias for 'note add'.
    Down(NoteAddArgs),
}

#[derive(Debug, Subcommand, Serialize, PartialEq)]
pub enum NoteCommand {
    /// Creates a new note.
    Add(NoteAddArgs),
    /// Lists notes.
    Search(NoteSearchArgs),
}

#[derive(Debug, Args, Serialize, PartialEq)]
pub struct NoteAddArgs {
    /// Assign to current day
    #[arg(long, short, default_value_t = false)]
    pub today: bool,
    /// Note content
    #[arg(trailing_var_arg = true)]
    pub content: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Serialize, Deserialize)]
pub enum OutputFormat {
    Pretty,
    Plain,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Pretty
    }
}

#[derive(Debug, clap::Args, PartialEq, Serialize, Deserialize)]
#[command(about = "Search and list notes")]
pub struct NoteSearchArgs {
    /// Search term to filter notes
    #[arg(default_value = None)]
    pub term: Option<String>,

    /// Filter by tags (can be specified multiple times or comma-separated)
    #[arg(long, value_name = "TAGS", value_delimiter = ',')]
    pub tag: Option<Vec<String>>,

    /// Filter by date (e.g., "today", "last week", "2024-03-16")
    #[arg(long, value_name = "DATE", value_parser = parse_date_target)]
    pub date: Option<DateTarget>,

    /// Number of lines to display for each note (default: full content)
    #[arg(long, value_name = "N")]
    pub lines: Option<usize>,

    /// Output format (pretty, plain, or json)
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
    pub output: OutputFormat,
}

impl Default for NoteSearchArgs {
    fn default() -> Self {
        Self {
            term: None,
            tag: None,
            date: None,
            lines: None,
            output: OutputFormat::Pretty,
        }
    }
}

pub fn parse_date_target(s: &str) -> anyhow::Result<DateTarget> {
    return s.parse();
}
