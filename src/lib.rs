//! Gets the stats of a [Github] repository.
//!
//! # Examples
//!
//! ## Get Stats of Repository
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
//! ## Search Latest Merged PR and Get Total Merged PR Count
//!
//! ```
//! use github_stats::{Query, Search};
//!
//! // Gets latest merged PR
//! let search = Search::issues(
//!     &Query::new().repo("rust-lang", "rust").is("pr").is("merged"),
//! )
//! .per_page(1)
//! .search();
//!
//! match search {
//!     Ok(results) => println!("# of merged PRs: {}", results.total_count()),
//!     Err(e) => eprintln!(":("),
//! }
//! ```
//!
//! [Github]: https://github.com/

pub use repository::Repo;
pub use search::{Query, Search};
pub use user::User;

mod repository;
pub mod search;
mod user;

/// This crate's standard error type.
pub type Error = Box<dyn std::error::Error>;

/// This crate's standard `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
