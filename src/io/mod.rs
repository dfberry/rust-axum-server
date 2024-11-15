use serde_json::json;
use serde_json::Value;
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio;
use chrono::Utc;
use std::path::Path;

pub async fn write_json_to_file(
    file_path: &str,
    file_name: &str, 
    json_blob: &Value
) -> Result<(), Box<dyn std::error::Error>> {

    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let file_path_with_timestamp = format!("{}/{}-{}", file_path, timestamp, file_name);

    println!("Writing JSON to file: {}", file_path_with_timestamp);

    // Create the directory path if it doesn't exist
    if let Some(parent) = Path::new(&file_path_with_timestamp).parent() {
        fs::create_dir_all(parent).await?;
    }

    let result = async {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .open(&file_path_with_timestamp)
            .await?;
    
        file.write_all(serde_json::to_string_pretty(json_blob)?.as_bytes()).await?;
        file.flush().await?;
    
        Ok::<(), Box<dyn std::error::Error>>(())
    }.await;

    match result {
        Ok(_) => {
            println!("Successfully wrote JSON to file: {}", file_path_with_timestamp);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error writing JSON to file: {:?}", e);
            Err(e)
        },
    }
}