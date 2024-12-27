#![deny(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
#![warn(clippy::expect_used)]

use crate::app_config::AppConfig;
use anyhow::Context;
use args::{CliArgs, Command};
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

        // Set profile_path variable to Some with the value of profile_path if it exists, otherwise set it to None
        let profile_path_cond = if profile_path.exists() {
            Some(profile_path.to_str().context("Unreadable file path.")?)
        } else {
            None
        };

        match command {
            Command::Config | Command::Init => match command {
                Command::Config => config_cmd(config)?,
                Command::Init => init_cmd(&config, &profile_path)?,
                _ => unreachable!(),
            },
            Command::Login => {
                let client = web_client::get_client(&config);
                login_cmd(client, profile_path_cond, &config.api_key_path).await?
            }
            Command::Note(_) | Command::Down(_) => {
                let client = web_client::get_client(&config);

                client.ping().await?;

                match command {
                    Command::Note(subcommand) => note_cmd(client, subcommand).await?,
                    Command::Down(args) => note_cmd(client, args::NoteCommand::Add(args)).await?,
                    _ => unreachable!(),
                }
            }
        }
    }

    Ok(())
}
