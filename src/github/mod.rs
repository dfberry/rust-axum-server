use anyhow::Result;
use octocrab::models::Repository;
use octocrab::models::UserProfile as User;
use octocrab::models::issues::Issue;
use octocrab::Page;
use octocrab::Octocrab;
use octocrab::Error as OctocrabError;
use http::Uri;
use serde::{Serialize, Deserialize};
use std::option::Option; 
use std::fmt;
use futures::future::join_all;
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


#[derive(Debug)]
pub struct RepoStats {
    stars: u32,
    forks: u32,
    open_issues: u32,
    open_prs: u64,
}

async fn fetch_repo_stats(octocrab: &Octocrab, repo: &str) -> Result<RepoStats, OctocrabError> {
    // split the repo string into owner and name
    let parts: Vec<&str> = repo.split('/').collect();
    let (owner, name) = match parts.as_slice() {
        [owner, name] => (owner.to_string(), name.to_string()),
        _ => {
            // Handle the error case where the repo string does not have the expected format
            panic!("Invalid repository format. Expected 'owner/repo'.");
        }
    };

    let repo_info: Repository = match octocrab.repos(&owner, &name).get().await {
        Ok(info) => info,
        Err(e) => {
            println!("Failed to fetch repository info for {}/{}: {}", owner, name, e);
            return Err(e);
        }
    };

    let stars = repo_info.stargazers_count.unwrap_or(0) as u32;
    let forks = repo_info.forks_count.unwrap_or(0) as u32;
    let open_issues = repo_info.open_issues_count.unwrap_or(0) as u32;
    let open_prs = octocrab
        .pulls(&owner, &name)
        .list()
        .state(octocrab::params::State::Open)
        .send()
        .await?
        .total_count
        .unwrap_or(0);

    Ok(RepoStats {
        stars,
        forks,
        open_issues,
        open_prs,
    })
}

pub async fn fetch_all_repos_stats(token: &str, repos: Vec<String>) -> Vec<Result<RepoStats, OctocrabError>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .unwrap();

    let tasks: Vec<_> = repos
        .iter()
        .map(|repo| {
            let octocrab = octocrab.clone();
            let repo = repo.clone();
            async move { fetch_repo_stats(&octocrab, &repo).await }
        })
        .collect();

    let results = join_all(tasks).await;
    results
}

