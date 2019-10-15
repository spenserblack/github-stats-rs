//! For getting repository information.

use chrono::prelude::{DateTime, Utc};
use serde::Deserialize;

use crate::{Result, User};

/// Represents that stats of a [Github] repository.
///
/// [Github]: https://github.com/
#[derive(Debug, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: User,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pushed_at: DateTime<Utc>,
    pub homepage: String,
    /// In *kilo*bytes.
    pub size: u64,
    pub stargazers_count: u64,
    pub language: String,
    pub forks_count: u64,
    pub archived: bool,
    pub disabled: bool,
    /// Issues + PRs
    pub open_issues: u64,
    pub default_branch: String,
    /// Number of watchers.
    pub subscribers_count: u64,
}

impl Repo {
    /// Creates a new `Repo`.
    ///
    /// # Example
    ///
    /// ```
    /// use github_stats::Repo;
    ///
    /// let repo = Repo::new("rust-lang", "rust");
    /// ```
    pub fn new(user: &str, repo: &str) -> Result<Self> {
        let repo: Repo = reqwest::get(&repo_api_url(user, repo))?.json()?;

        Ok(repo)
    }
}

// Takes [Github] user and repo IDs to make a link to the API for that repo.
//
// [Github]: https://github.com/
fn repo_api_url(user: &str, repo: &str) -> String {
    const URL: &str = "https://api.github.com/repos";
    format!("{}/{}/{}", URL, user, repo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:#?}", repo_stats("rust-lang", "rust").unwrap());
        assert!(true);
    }
}
