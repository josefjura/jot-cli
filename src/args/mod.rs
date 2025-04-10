use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

use crate::utils::date::{date_filter::DateFilter, date_value::DateValue};

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
    /// Get latest note.
    Last(NoteLatestArgs),
}

#[derive(Debug, Args, Serialize, PartialEq)]
pub struct NoteAddArgs {
    /// Assign to current day
    #[arg(long, short, default_value_t = DateValue::Today)]
    pub date: DateValue,
    /// Note content
    #[arg(trailing_var_arg = true)]
    pub content: Vec<String>,
    /// Open in external editor
    #[arg(long, short, default_value_t = false)]
    pub edit: bool,
    /// Filter by tags (can be specified multiple times or comma-separated)
    #[arg(long, value_name = "TAGS", value_delimiter = ',')]
    pub tag: Vec<String>,
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

#[derive(Debug, Clone, clap::Args, PartialEq, Serialize, Deserialize)]
#[command(about = "Search and list notes")]
pub struct NoteSearchArgs {
    /// Search term to filter notes
    #[arg(default_value = None)]
    pub term: Option<String>,

    /// Filter by tags (can be specified multiple times or comma-separated)
    #[arg(long, value_name = "TAGS", value_delimiter = ',')]
    pub tag: Vec<String>,

    /// Filter by assigned date (e.g., "today", "last week", "2024-03-16")
    #[arg(long, value_name = "DATE")]
    pub date: Option<DateFilter>,

    /// Filter by date the note was created
    #[arg(long)]
    pub created: Option<DateFilter>,

    /// Filter by date the note was last updated
    #[arg(long)]
    pub updated: Option<DateFilter>,

    /// Number of lines to display for each note (default: full content)
    #[arg(long, value_name = "N")]
    pub lines: Option<usize>,

    /// Maximum number of results to return
    #[arg(long, short = 'l')]
    pub limit: Option<i64>,

    /// Output format (pretty, plain, or json)
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
    pub output: OutputFormat,

    // Ask for found notes to be deleted after displaying
    #[arg(long, default_value_t = false)]
    pub delete: bool,
}

#[derive(Debug, clap::Args, PartialEq, Serialize, Deserialize)]
#[command(about = "Retrieve the latest order")]
pub struct NoteLatestArgs {
    /// Search term to filter notes
    #[arg(default_value = None)]
    pub term: Option<String>,

    /// Filter by tags (can be specified multiple times or comma-separated)
    #[arg(long, value_name = "TAGS", value_delimiter = ',')]
    pub tag: Vec<String>,

    /// Output format (pretty, plain, or json)
    #[arg(long, value_enum, default_value_t = OutputFormat::Pretty)]
    pub output: OutputFormat,
}

impl Default for NoteSearchArgs {
    fn default() -> Self {
        Self {
            term: None,
            tag: vec![],
            date: None,
            lines: None,
            created: None,
            updated: None,
            limit: None,
            output: OutputFormat::Pretty,
            delete: false,
        }
    }
}
