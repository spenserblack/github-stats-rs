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

use std::collections::HashMap;

pub use repository::Repo;

pub mod repository;

/// This crate's standard error type.
pub type Error = Box<dyn std::error::Error>;

type JsonMap<T> = HashMap<String, T>;

// This crate's standard result type.
type Response = serde_json::Value;

/// This crate's standard `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
