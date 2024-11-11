use octocrab::models::pulls::PullRequest;
use octocrab::models::Repository;
use octocrab::models::UserProfile as User;
use octocrab::models::issues::Issue;
use octocrab::models::RepositoryMetrics;
use octocrab::models::License;
use octocrab::Page;
use octocrab::Octocrab;
use octocrab::models::repos::RepoCommit;
use http::Uri;
use serde::{Serialize, Deserialize};
use std::option::Option; 
use std::collections::HashMap;
use futures::future::join_all;
use anyhow::{Result, Context, anyhow};
use chrono::DateTime;
use chrono::Utc;
use crate::utils::{option_datetime_to_string, parse_repo_string};
use reqwest::Client;
use std::error::Error;

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


#[derive(Serialize, Deserialize)]
pub struct PullRequestQueryDef {
    #[serde(flatten)]
    inner: octocrab::models::pulls::PullRequest
}

fn convert_pr_page_to_page_def(page: Page<PullRequest>) -> PageDef<PullRequestQueryDef> {
    let items = page.items.into_iter().map(|pr| PullRequestQueryDef { inner: pr }).collect();
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
    pub async fn repo_issues(token: &str, org_or_user: &str, repo_name: &str) -> Result<PageDef<IssueQueryDef>> {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;

        let issues = octocrab
            .issues(org_or_user, repo_name)
            .list()
            .send()
            .await?;

        let page_def = convert_page_to_page_def(issues);

        Ok(page_def)
    }
    pub async fn repo_prs(token: &str, org_or_user: &str, repo_name: &str) -> Result<PageDef<PullRequestQueryDef>> {
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;

        let prs = octocrab
            .pulls(org_or_user, repo_name)
            .list()
            .send()
            .await?;

        let page_def = convert_pr_page_to_page_def(prs);

        Ok(page_def)
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
    pub watchers: u64,
    pub last_commit: String,
    pub archived: Option<bool>,
    pub size: Option<u32>,
    pub visibility: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub license: Option<License>,
    pub allow_auto_merge: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct RepoStatsResult {
    pub repo_name: String,
    pub stats: RepoStats,
    pub metrics: Option<RepositoryMetrics>,
    pub errors: Vec<String>,
}

async fn fetch_repo_stats(octocrab: &Octocrab, repo: &str) -> RepoStatsResult {

    let mut repo_stats_result = RepoStatsResult {
        repo_name: repo.to_string(),
        stats: RepoStats {
            stars: 0,
            forks: 0,
            open_issues: 0,
            watchers: 0,
            last_commit: "".to_string(),
            archived: None,
            size: None,
            visibility: None,
            created_at: None,
            license: None,
            allow_auto_merge: None,
        },
        metrics: None,
        errors: Vec::new(),
    };

    let (owner, name) = parse_repo_string(repo).unwrap();
    let metrics = octocrab.repos(&owner, &name).get_community_profile_metrics().await;
    let last_commit = fetch_last_commit(octocrab, repo).await;

    match octocrab.repos(&owner, &name).get().await{
        Ok(info) => {
            repo_stats_result.stats.stars = info.stargazers_count.unwrap_or(0) as u32;
            repo_stats_result.stats.forks = info.forks_count.unwrap_or(0) as u32;
            repo_stats_result.stats.open_issues = info.open_issues_count.unwrap_or(0) as u32;
            repo_stats_result.stats.watchers = info.watchers_count.unwrap_or(0) as u64;
            repo_stats_result.metrics = metrics.ok();
            repo_stats_result.stats.last_commit = last_commit.unwrap_or_else(|e| e.to_string());
            repo_stats_result.stats.archived = info.archived;
            repo_stats_result.stats.size = info.size;
            repo_stats_result.stats.visibility = info.visibility;
            repo_stats_result.stats.created_at = info.created_at;
            repo_stats_result.stats.license = info.license;
            repo_stats_result.stats.allow_auto_merge = info.allow_auto_merge;
        },
        Err(e) => {
            repo_stats_result.errors.push(format!("Failed to fetch repository info for {}/{}: {}", owner, name, e));
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

pub async fn fetch_community_metrics(octocrab: &Octocrab, repo: &str) -> Result<RepositoryMetrics> {
    let metrics = octocrab.repos("dfberry", repo).get_community_profile_metrics().await?;
    Ok(metrics)
}

fn get_commit_date(commit: &RepoCommit) -> String {
    let author_date = commit
    .commit
    .author.as_ref().map(|a| a.date.clone()).unwrap();

    option_datetime_to_string(author_date)
}

pub async fn fetch_last_commit(octocrab: &Octocrab, repo: &str) -> Result<String> {

    let (owner, name) = parse_repo_string(repo)?;

    let commits = octocrab
        .repos(&owner, &name)
        .list_commits()
        .per_page(1)
        .send()
        .await?;

    let first_commit = commits.items.into_iter().next();

    // Handle the case where there are no commits
    let commit = match first_commit {
        Some(commit) => commit,
        None => return Ok("No commits found".to_string()),
    };

    // Get date of last commit
    let commit_date = get_commit_date(&commit);
    Ok(commit_date)
}


#[derive(Debug)]
pub struct GitHubApi {
    pub name: String,
}

impl GitHubApi{
    pub async fn get_user_by_token(token: &str) -> Result<User, Box<dyn Error>> {
        let client = Client::new();
        let request_url = "https://api.github.com/user";

        let response = client
            .get(request_url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "rust-reqwest")
            .send()
            .await?
            .json::<User>()
            .await?;
        println!("{response:#?}");
        Ok(response)
    }
}