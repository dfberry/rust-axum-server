use anyhow::Result;
use octocrab::models::Repository;
use octocrab::models::UserProfile as User;
use octocrab::models::issues::Issue;
use octocrab::Page;
use octocrab::Octocrab;
use http::*;
use serde::{Serialize, Deserialize};
use std::option::Option; 

#[derive(Serialize, Deserialize)]
pub struct IssueQueryDef {
    #[serde(flatten)]
    inner: octocrab::models::issues::Issue,
}

#[derive(Serialize, Deserialize)]
pub struct PageDef<T>{
    pub items: Vec<T>,
    pub incomplete_results: Option<bool>,
    pub total_count: Option<u64>,
    #[serde(with = "http_serde_ext::uri::option", default)]
    pub next: Option<Uri>,
    #[serde(with = "http_serde_ext::uri::option", default)]
    pub prev: Option<Uri>,
    #[serde(with = "http_serde_ext::uri::option", default)]
    pub first: Option<Uri>,
    #[serde(with = "http_serde_ext::uri::option", default)]
    pub last: Option<Uri>,
}

#[derive(Debug)]
pub struct GitHub {
    pub name: String,
}

// Conversion function
fn convert_page_to_page_def(page: Page<Issue>) -> PageDef<IssueQueryDef> {
    let items = page.items.into_iter().map(|issue| IssueQueryDef { inner: issue }).collect();
    PageDef {
        items,
        incomplete_results: page.incomplete_results,
        total_count: page.total_count,
        next: page.next,
        prev: page.prev,
        first: page.first,
        last: page.last,
    }
}

impl GitHub {
    pub async fn repo(token: &str, org_or_user: &str, repo_name: &str) -> Result<Repository> {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;

        let repo = octocrab.repos(org_or_user, repo_name).get().await?;

        Ok(repo)
    }
    pub async fn user_profile(token: &str, user: &str) -> Result<User> {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;

        let repo = octocrab.users(user).profile().await?;

        Ok(repo)
    }
    pub async fn query(token: &str, query: &str) -> Result<PageDef<IssueQueryDef>> {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;

        let query_result = octocrab
            .search()
            .issues_and_pull_requests(query) //"tokei is:pr"
            .send()
            .await?;

        let page_def = convert_page_to_page_def(query_result);

        Ok(page_def)
    }
}