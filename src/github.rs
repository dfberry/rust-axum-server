use octocrab::Octocrab;
use octocrab::models::Repository;
use octocrab::models::UserProfile as User;
use anyhow::Result;

#[derive(Debug)]
pub struct GitHub {
    pub name: String,
}

impl GitHub {
    pub async fn repo(token: &str, org_or_user: &str, repo_name: &str) -> Result<Repository> {
        let octocrab = Octocrab::builder().personal_token(token.to_string()).build()?;

        let repo = octocrab
            .repos(org_or_user, repo_name)
            .get()
            .await?;

        Ok(repo)
    }
    pub async fn user_profile(token: &str, user: &str) -> Result<User> {
        let octocrab = Octocrab::builder().personal_token(token.to_string()).build()?;

        let repo = octocrab.users(user).profile().await?;

        Ok(repo)
    }
}