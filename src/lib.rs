use big_bytes::BigByte;

// The URL for [Github] repository data.
//
// Append `/`*user*`/`*repo* to the end for the full URL.
//
// # Example
//
// ```
// let url = format!("{}/{}/{}", GITHUB_API_REPO_URL, "rust-lang", "rust");
// ```
//
// [Github]: https://github.com/
const GITHUB_API_REPO_URL: &str = "https://api.github.com/repos";

/// This crate's standard error type.
pub type Error = Box<dyn std::error::Error>;

// This crate's standard result type.
type Response = serde_json::Value;

/// This crate's standard `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents that stats of a [Github] repository.
///
/// [Github]: https://github.com/
#[derive(Debug)]
pub struct Repo {
    name: String,
    // is_fork: bool,
    // created_at: String,
    // updated_at: String,
    // homepage: Option<String>,
    size: f64,
    stars: u64,
    // language: String,
    forks: u64,
    // open_issues: u32,
    // closed_issues: u32,
    // open_pull_requests: u32,
    // closed_pull_requests: u32,
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
        let repo_data = repo_stats(user, repo)?;
        let name = repo_data["name"].as_str().ok_or(r#""name" is not a string"#)?.to_string();
        let size = repo_data["size"].as_f64().ok_or(r#""size" cannot be read as f64"#)?;
        let stars = repo_data["stargazers_count"].as_u64().ok_or(r#""stars" cannot be read as u64"#)?;
        let forks = repo_data["forks"].as_u64().ok_or(r#""forks_count" cannot be read as u64"#)?;
        let repo = Repo {
            name,
            size,
            stars,
            forks,
        };
        Ok(repo)
    }

    /// Gets the repository's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the repository's size in kilobytes.
    pub fn size(&self) -> f64 {
        self.size
    }

    /// Gets the repository's size in a human-readable format.
    ///
    /// `precision` is the number of decimal places to display.
    pub fn human_size(&self, precision: usize) -> String {
        self.size.big_byte(precision)
    }

    /// Gets the repository's star count.
    pub fn stars(&self) -> u64 {
        self.stars
    }

    /// Gets the repository's fork count.
    pub fn forks(&self) -> u64 {
        self.forks
    }
}

// Takes [Github] user and repo IDs to make a link to the API for that repo.
//
// [Github]: https://github.com/
fn repo_api_url(user: &str, repo: &str) -> String {
    format!("{}/{}/{}", GITHUB_API_REPO_URL, user, repo)
}

// Requests repo data from [Github]'s API.
//
// [Github]: https://github.com/
fn repo_stats(user: &str, repo: &str) -> Result<Response> {
    let response: Response = reqwest::get(&repo_api_url(user, repo))?
        .json()?;
    Ok(response)
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
