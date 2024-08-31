use chrono::DateTime;
use chrono::Utc;
use anyhow::{Result, Context, anyhow};

pub fn option_datetime_to_string(date: Option<DateTime<Utc>>) -> String {
    date.map(|d| d.to_string())
        .unwrap_or_else(|| "No date available".to_string())
}
pub fn parse_repo_string(repo: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = repo.split('/').collect();
    match parts.as_slice() {
        [owner, name] => 
        Ok((owner.to_string(), name.to_string())),
        _ => Err(anyhow!("Invalid repository format. Expected 'owner/repo'.")),
    }
}