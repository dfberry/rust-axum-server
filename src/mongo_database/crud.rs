use mongodb::{Client, Collection, error::Result};
use mongodb::bson::Document;
use serde::Serialize;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use std::fmt::Debug;

pub async fn get_connection(connection_string: String) -> Client {
    return Client::with_uri_str(connection_string).await.unwrap()
}

pub async fn read<'a, T: DeserializeOwned + Send + Sync + Debug + 'a>(collection: &Collection<T>, filter: Document, sort: Document, limit: i64) -> Result<Vec<T>> {

    let mut cursor = collection
    .find(filter)
    .sort(sort)
    .limit(limit)
    .await?;
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        results.push(result?);
    }
    Ok(results)
}