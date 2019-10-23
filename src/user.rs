//! For getting user information.

use serde::Deserialize;

use crate::Result;

/// Represents that stats of a [Github] user.
///
/// [Github]: https://github.com/
#[derive(Debug, Deserialize)]
pub struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    html_url: String,
    url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

impl User {
    /// Creates a new `User`
    ///
    /// # Example
    ///
    /// ```
    /// use github_stats::User;
    ///
    /// let user = User::new("rust-lang");
    /// ```
    pub fn new(user: &str) -> Result<Self> {
        const URL: &str = "https://api.github.com/users";
        let url = format!("{}/{}", URL, user);
        let user: User = reqwest::get(&url)?.json()?;

        Ok(user)
    }
    pub fn login(&self) -> &str {
        &self.login
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
    pub fn avatar_url(&self) -> &str {
        &self.avatar_url
    }
    pub fn gravatar_id(&self) -> &str {
        &self.gravatar_id
    }
    /// Actual link to the user's page.
    pub fn html_url(&self) -> &str {
        &self.html_url
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn followers_url(&self) -> &str {
        &self.followers_url
    }
    pub fn following_url(&self) -> &str {
        &self.following_url
    }
    pub fn gists_url(&self) -> &str {
        &self.gists_url
    }
    pub fn starred_url(&self) -> &str {
        &self.starred_url
    }
    pub fn subscriptions_url(&self) -> &str {
        &self.subscriptions_url
    }
    pub fn organizations_url(&self) -> &str {
        &self.organizations_url
    }
    pub fn repos_url(&self) -> &str {
        &self.repos_url
    }
    pub fn events_url(&self) -> &str {
        &self.events_url
    }
    pub fn received_events_url(&self) -> &str {
        &self.received_events_url
    }
    /// *Use `r#type` to avoid conflict with `type` keyword.*
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
    pub fn site_admin(&self) -> bool {
        self.site_admin
    }
}
