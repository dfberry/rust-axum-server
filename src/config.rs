use std::env;
use std::fs;
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
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

        let env_file = EnvFile {
            pat,
            github_client_id,
            github_password,
            github_redirect_uri,
            github_scope: "user".to_string(),
            port,
        };

        // Toml file
        let file_path = "../Cargo.toml";
        let cargo_toml_content = async_fs::read_to_string(file_path)
            .await
            .map_err(|e| format!("Failed to read {}: {}", file_path, e))?;
        
        let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)?;

        let package = Package {
            name: cargo_toml.package.name,
            version: cargo_toml.package.version,
        };
        Ok(Config {
            env_file,
            package,
        })
    }
}