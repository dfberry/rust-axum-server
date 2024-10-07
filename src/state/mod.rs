use std::sync::{Arc, RwLock};
use serde::Deserialize;
use anyhow::{Result, Context};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}

use std::env;
use toml;
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


#[derive(Deserialize, Debug)]
pub struct Config {
    pub env_file: EnvFile,
    pub package: Package,
}
#[derive(Deserialize, Debug)]
pub struct CargoToml {
    pub package: Package,
}
impl Config {


    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {

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
        
        let cargo_toml = read_and_parse_cargo_toml("./Cargo.toml").await?;

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
async fn read_and_parse_cargo_toml(file_path: &str) -> Result<CargoToml> {
    let mut file = File::open(file_path)
        .await
        .with_context(|| format!("Failed to open {}", file_path))?;
    
    let mut cargo_toml_content = String::new();
    file.read_to_string(&mut cargo_toml_content)
        .await
        .with_context(|| format!("Failed to read {}", file_path))?;
    
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)
        .with_context(|| format!("Failed to parse {}", file_path))?;
    
    Ok(cargo_toml)
}
pub async fn get_cargo_version() -> Result<String> {
    let cargo_toml = read_and_parse_cargo_toml("./Cargo.toml").await?;
    Ok(cargo_toml.package.version)
}