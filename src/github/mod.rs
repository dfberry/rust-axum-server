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

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct RepoStats {
    pub stars: u32,
    pub forks: u32,
    pub open_issues: u32,
    pub open_prs: u64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct RepoStatsResult {
    pub repo_name: String,
    pub stats: RepoStats,
    pub errors: Vec<String>,
}

async fn fetch_repo_stats(octocrab: &Octocrab, repo: &str) -> RepoStatsResult {
    // split the repo string into owner and name
    let parts: Vec<&str> = repo.split('/').collect();
    let (owner, name) = match parts.as_slice() {
        [owner, name] => (owner.to_string(), name.to_string()),
        _ => {
            // Handle the error case where the repo string does not have the expected format
            panic!("Invalid repository format. Expected 'owner/repo'.");
        }
    };

    let mut repo_stats_result = RepoStatsResult {
        repo_name: repo.to_string(),
        stats: RepoStats {
            stars: 0,
            forks: 0,
            open_issues: 0,
            open_prs: 0,
        },
        errors: Vec::new(),
    };

    match octocrab.repos(&owner, &name).get().await{
        Ok(info) => {
            repo_stats_result.stats.stars = info.stargazers_count.unwrap_or(0) as u32;
            repo_stats_result.stats.forks = info.forks_count.unwrap_or(0) as u32;
            repo_stats_result.stats.open_issues = info.open_issues_count.unwrap_or(0) as u32;
        },
        Err(e) => {
            repo_stats_result.errors.push(format!("Failed to fetch repository info for {}/{}: {}", owner, name, e));
        }
    };
    
    match octocrab
                        .pulls(&owner, &name)
                        .list()
                        .state(octocrab::params::State::Open)
                        .send()
                        .await{
        Ok(info) => {
                            repo_stats_result.stats.open_prs = 0;
        },
        Err(e) => {
            repo_stats_result.errors.push(format!("Failed to fetch pr count for {}/{}: {}", owner, name, e));
        }
    };
    repo_stats_result

}

pub async fn fetch_all_repos_stats(token: &str, repos: Vec<String>) -> Vec<RepoStatsResult> {
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

