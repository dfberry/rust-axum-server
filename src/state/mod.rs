use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use crate::mongo_database::{
    models::FlattenedRepoData,

};
#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Arc<RwLock<Config>>,
}

#[derive(Clone, Debug)]
pub struct MongoDb {
    pub collection_client: mongodb::Collection<FlattenedRepoData>
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
    //pub github_client_id: String,
    //pub github_password: String,
    pub github_redirect_uri: String,
    //pub github_scope: String,
    pub port: String,
    //pub environment: String,
    //pub database_url: String,
    pub admin_key: String,
}


#[derive(Debug)]
pub struct Config {
    pub env_file: EnvFile,
    pub package: Package,
    pub mongo: MongoDb,
}
#[derive(Deserialize, Debug)]
pub struct CargoToml {
    pub package: Package,
}
impl Config {


    pub async fn get() -> Result<Self, Box<dyn std::error::Error>> {

        // Environment variables
        let pat = env::var("PAT")?;
        eprintln!("Length of PAT: {}", pat.len());
    
        // let github_client_id = env::var("GITHUB_CLIENT_ID")?;
        // println!("Length of GITHUB_CLIENT_ID: {}", github_client_id.len());
    
        // let github_password = env::var("GITHUB_PASSWORD")?;
        // println!("Length of GITHUB_PASSWORD: {}", github_password.len());
    
        let github_redirect_uri = env::var("GITHUB_REDIR_URL")?;
        eprintln!("Length of GITHUB_REDIR_URL: {}", github_redirect_uri.len());
    
        let port = env::var("PORT").unwrap_or_else(|_| "4000".to_string());
        eprintln!("Length of PORT: {}", port.len());
    
        // let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
        // println!("Length of ENVIRONMENT: {}", environment.len());
    
        // let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost:5432".to_string());
        // println!("Length of DATABASE_URL: {}", database_url.len());
    
        let admin_key = env::var("ADMIN_KEY").unwrap_or_else(|_| "".to_string());
        eprintln!("Length of ADMIN_KEY: {}", admin_key.len());
    
        // MongoDB
        let db_url = env::var("MONGO_DATABASE_URL")?;
        eprintln!("Length of MONGO_DATABASE_URL: {}", db_url.len());
    
        let db_name = env::var("MONGO_DATABASE_NAME")?;
        eprintln!("Length of MONGO_DATABASE_NAME: {}", db_name.len());
    
        let db_collection = env::var("MONGO_COLLECTION_NAME")?;
        eprintln!("Length of MONGO_COLLECTION_NAME: {}", db_collection.len());
  
        let client = Client::with_uri_str(db_url).await.unwrap();
        let collection: Collection<FlattenedRepoData> = client.database(&db_name).collection(&db_collection);
        let mongo = MongoDb {
            collection_client: collection,
        };
        
        let env_file = EnvFile {
            pat,
            //github_client_id,
            //github_password,
            github_redirect_uri,
            //github_scope: "user".to_string(),
            port,
            //environment: environment.clone(),
            //database_url,
            admin_key,
        };

        // Print out the env_file
        eprintln!("{:?}", env_file);

        // Toml file
        // Determine the file path based on the environment
        
        let cargo_toml = read_and_parse_cargo_toml("./Cargo.toml").await?;

        let package = Package {
            name: cargo_toml.package.name,
            version: cargo_toml.package.version,
        };

        // Print out the package
        eprintln!("{:?}", package);

        Ok(Config {
            env_file,
            package,
            mongo
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