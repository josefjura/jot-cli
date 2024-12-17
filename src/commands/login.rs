use crate::{app_config::AppConfig, auth::AuthFlow, web_client};

pub async fn login_cmd(config: AppConfig) -> Result<(), anyhow::Error> {
    if config.profile_exists {
        println!("Using profile: {:?}", config.profile_path);
    }
    let mut client = web_client::get_client(&config);
    let token = AuthFlow::new().login(client.as_mut()).await;

    match token {
        Ok(token) => {
            println!("Api Key Path: {}", config.api_key_path);
            std::fs::write(config.api_key_path, token)?;
            println!("User successfully logged in.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
