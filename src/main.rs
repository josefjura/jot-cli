#![deny(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
#![warn(clippy::expect_used)]

use crate::app_config::AppConfig;
use args::CliArgs;
use clap::Parser;
use commands::{config::config_cmd, init::init_cmd, login::login_cmd, note::note_cmd};
use profile::{get_profile_path, Profile};

mod app_config;
mod args;
mod auth;
mod commands;
mod editor;
mod formatters;
mod init;
mod model;
mod profile;
mod utils;
mod web_client;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let profile_path = get_profile_path(&args.config.profile_path);

    if let Some(command) = args.command {
        let profile = Profile::from_path(&profile_path)?;
        let config = AppConfig::from_args(args.config, &profile_path, profile.as_ref());
        match command {
            args::Command::Config => config_cmd(config)?,
            args::Command::Init => init_cmd(&config, &profile_path)?,
            args::Command::Login => login_cmd(config).await?,
            args::Command::Note(subcommand) => note_cmd(&config, subcommand).await?,
            args::Command::Down(args) => note_cmd(&config, args::NoteCommand::Add(args)).await?,
        }
    }

    Ok(())
}
