use serde::{Serialize, Deserialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct FlattenedRepoData {
    org: String,
    repo: String,
    name: String,
    description: Option<String>,
    url: String,
    createdAt: String,
    updatedAt: String,
    pushedAt: String,
    diskUsage: i32,
    watchersCount: i32,
    stargazersCount: i32,
    forksCount: i32,
    openIssuesCount: i32,
    openPRsCount: i32
}