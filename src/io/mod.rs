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
    // Create the directory path if it doesn't exist
    if let Some(parent) = Path::new(&file_path_with_timestamp).parent() {
        if let Err(e) = fs::create_dir_all(parent).await {
            eprintln!("Error creating directory: {:?}", e);
            return Ok(()); // Return Ok(()) to ensure no error is propagated
        }
    }
    let result = async {
        let mut file = match fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .open(&file_path_with_timestamp)
            .await {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {:?}", e);
                    return Ok(()); // Return Ok(()) to ensure no error is propagated
                }
            };
    
        let json_string = match serde_json::to_string_pretty(json_blob) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error serializing JSON: {:?}", e);
                return Ok(()); // Return Ok(()) to ensure no error is propagated
            }
        };

        if let Err(e) = file.write_all(json_string.as_bytes()).await {
            eprintln!("Error writing to file: {:?}", e);
            return Ok(()); // Return Ok(()) to ensure no error is propagated
        }
    
        if let Err(e) = file.flush().await {
            eprintln!("Error flushing file: {:?}", e);
            return Ok(()); // Return Ok(()) to ensure no error is propagated
        }
    
        Ok::<(), ()>(())
    }.await;

    match result {
        Ok(_) => {
            println!("Successfully wrote JSON to file: {}", file_path_with_timestamp);
        },
        Err(_) => {
            eprintln!("Error writing JSON to file");
        },
    }

    Ok(())
}