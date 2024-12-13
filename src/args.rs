use clap::{Args, Parser, Subcommand};
use serde::Serialize;

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
    pub profile: Option<String>,

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
}
