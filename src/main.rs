use crate::app_config::AppConfig;
use anyhow::Context;
use args::CliArgs;
use auth::AuthFlow;
use clap::Parser;
use init::read_profile;
use profile::{get_profile_path, Profile};

mod app_config;
mod args;
mod auth;
mod init;
mod model;
mod profile;
mod web_client;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let profile_path = get_profile_path(&args.config.profile);

    if let Some(command) = args.command {
        let profile = Profile::from_path(&profile_path)?;
        let config = AppConfig::from_args(args.config, &profile_path, profile.as_ref());
        match command {
            args::Command::Config => {
                let json = serde_json::to_string_pretty(&config)?;
                println!("{}", json);
                return Ok(());
            }
            args::Command::Login => {
                if config.profile_exists {
                    println!("Using profile: {:?}", profile_path);
                }
                let mut client = web_client::get_client(&config);
                let _ = AuthFlow::new().login(client.as_mut()).await?;
                println!("Logging in...");
            }
            args::Command::Init => {
                if config.profile_exists {
                    println!("Using profile: {:?}", profile_path);
                } else {
                    println!("Profile will be written into '{:?}'", profile_path);
                    let new_profile =
                        read_profile(&config).context("An error during profile initialization")?;

                    println!("New profile: {:?}", new_profile);
                }
            }
        }
    }

    Ok(())
}
