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

// Takes [Github] user and repo IDs to make a link to the API for that repo.
//
// [Github]: https://github.com/
fn repo_api_url(user: &str, repo: &str) -> String {
    format!("{}/{}/{}", GITHUB_API_REPO_URL, user, repo)
}

/// Requests repo data from [Github]'s API.
///
/// [Github]: https://github.com/
pub fn repo_stats(user: &str, repo: &str) -> Result<Response> {
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
