//! Gets the stats of a [Github] repository.
//!
//! # Example
//!
//! ```
//! use github_stats::Repo;
//!
//! let repo = Repo::new("rust-lang", "rust");
//!
//! match repo {
//!     Ok(repo) => {/* Do some stuff */},
//!     Err(e) => eprintln!(":("),
//! }
//! ```
//!
//! [Github]: https://github.com/

use chrono::NaiveDateTime;

pub use repository::Repo;

pub mod repository;

/// This crate's standard error type.
pub type Error = Box<dyn std::error::Error>;

// This crate's standard result type.
type Response = serde_json::Value;

/// This crate's standard `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

fn gh_datestr_to_chrono(date: &str) -> Result<NaiveDateTime> {
    const GITHUB_DATETIME_FORMAT: &str = "%+";

    let date = NaiveDateTime::parse_from_str(date, GITHUB_DATETIME_FORMAT)?;

    Ok(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_to_chrono_test() {
        const DATE_STRING: &str = "2010-06-16T20:39:03Z";

        assert!(gh_datestr_to_chrono(DATE_STRING).is_ok(), "Failed to parse date");
    }
}
