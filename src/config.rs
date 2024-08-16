use std::env;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub pat: String,
    pub github_client_id: String,
    pub github_password: String,
    pub port: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let pat = env::var("PAT").expect("PAT env variable is required");
        let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID env variable is required");
        let github_password = env::var("GITHUB_PASSWORD").expect("GITHUB_PASSWORD env variable is required");
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

        Config {
            pat,
            github_client_id,
            github_password,
            port,
        }
    }
}