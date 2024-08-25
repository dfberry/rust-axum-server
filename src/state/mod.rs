use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}

use std::env;
use toml;
use serde::Deserialize;
use dotenv::dotenv;
use tokio::fs as async_fs;
//use urlencoding::encode;

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct EnvFile {
    pub pat: String,
    pub github_client_id: String,
    pub github_password: String,
    pub github_redirect_uri: String,
    pub github_scope: String,
    pub port: String,
    pub environment: String,
    pub database_url: String,
}


#[derive(Debug)]
pub struct Config {
    pub env_file: EnvFile,
    pub package: Package,
}
#[derive(Deserialize)]
pub struct CargoToml {
    pub package: Package,
}
impl Config {


    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {

        dotenv().ok();

        // Environment variables
        let pat = env::var("PAT")?;
        let github_client_id = env::var("GITHUB_CLIENT_ID")?;
        let github_password = env::var("GITHUB_PASSWORD")?;
        let github_redirect_uri = env::var("GITHUB_REDIR_URL")?;
        let port = env::var("PORT").unwrap_or_else(|_| "4000".to_string());
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost:5432".to_string());

        let env_file = EnvFile {
            pat,
            github_client_id,
            github_password,
            github_redirect_uri,
            github_scope: "user".to_string(),
            port,
            environment: environment.clone(),
            database_url,
        };

        // Print out the env_file
        println!("{:?}", env_file);

        // Toml file
        // Determine the file path based on the environment
        let file_path = "./Cargo.toml";
        let cargo_toml_content = async_fs::read_to_string(file_path)
            .await
            .map_err(|e| format!("Failed to read {}: {}", file_path, e))?;
        
        let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)?;

        let package = Package {
            name: cargo_toml.package.name,
            version: cargo_toml.package.version,
        };

        // Print out the package
        println!("{:?}", package);

        Ok(Config {
            env_file,
            package,
        })
    }
}