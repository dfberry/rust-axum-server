use std::sync::{Arc, RwLock};
use serde::Deserialize;
use anyhow::{Result, Context};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}

use std::env;
use toml;
use dotenvy::dotenv;
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

        dotenvy::dotenv()?;

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
    Ok(CargoToml {
        package: Package {
            name: "example".to_string(),
            version: "0.3.1".to_string(),
        },
    })
}

pub async fn get_cargo_version() -> Result<String> {
    let cargo_toml = read_and_parse_cargo_toml("./Cargo.toml").await?;
    Ok(cargo_toml.package.version)
}


