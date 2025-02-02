use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{
        DateTime,
        serde_helpers::{
            serialize_bson_datetime_as_rfc3339_string, 
            serialize_hex_string_as_object_id,
            deserialize_hex_string_from_object_id,
            deserialize_bson_datetime_from_rfc3339_string
    }},
};
#[derive(Deserialize, Serialize, Debug)]
pub struct FlattenedRepoData {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub pushedAt: String,
    pub diskUsage: i32,
    pub watchersCount: i32,
    pub stargazersCount: i32,
    pub forksCount: i32,
    pub openIssuesCount: i32,
    pub openPRsCount: i32,
    pub org: String,
    pub repo: String,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", deserialize_with = "deserialize_bson_datetime_from_rfc3339_string")]
    pub log_time: bson::DateTime
}