use mongodb::{error::Result, Client, Collection, Database};
use serde_json::Value;
use mongodb::bson::{doc, Document};
use futures::stream::TryStreamExt;

use crate::database;

use super::models::FlattenedRepoData;

pub async fn get_connection(
    connection_string: &str
) -> Result<Client> {
    Ok(Client::with_uri_str(connection_string).await.unwrap())
}

pub async fn read_log(
    collection_client : &Collection<FlattenedRepoData>, 
    org: &String, 
    repo: &String, 
    begin_date: &String, 
    end_date: &String
) -> Result<Vec<FlattenedRepoData>> {
    // Construct the query filter
    let filter = doc! {
        "org": org,
        "repo": repo,
        "date": {
            "$gte": begin_date,
            "$lte": end_date,
        }
    };

    // Execute the query
    let mut cursor = collection_client.find(filter).await?;
    let mut results: Vec<FlattenedRepoData> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        results.push(doc);
    }

    Ok(results)
}