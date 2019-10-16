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

pub use search::{Query, Search};
pub use repository::Repo;
pub use user::User;

mod search;
mod repository;
mod user;

/// This crate's standard error type.
pub type Error = Box<dyn std::error::Error>;

/// This crate's standard `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
