use serde_json::json;
use serde_json::Value;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio;

pub async fn write_json_to_file(
    file_name_and_path: &str, 
    json_blob: &Value
) -> Result<(), Box<dyn std::error::Error>> {

    println!("Writing JSON to file: {}", file_name_and_path);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_name_and_path)
        .await?;

    file.write_all(serde_json::to_string_pretty(json_blob)?.as_bytes()).await?;
    file.flush().await?;

    println!("Successfully wrote JSON to file: {}", file_name_and_path);

    Ok(())
}