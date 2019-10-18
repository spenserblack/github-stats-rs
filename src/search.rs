use std::fmt;

use serde::Deserialize;
use serde_json::Value;

use crate::Result;

pub use query::Query;

mod query;

/// Uses [Github]'s search API.
///
/// # Example
/// ## Get merged PRs
///
/// ```
/// use github_stats::{Query, Search};
///
/// let query = Query::new()
///     .repo("rust-lang", "rust")
///     .is("pr")
///     .is("merged");
///
/// let results = Search::new("issues", &query)
///     .per_page(10)
///     .page(1)
///     .search();
///
/// match results {
///     Ok(results) => { /* do stuff */ }
///     Err(e) => eprintln!(":("),
/// }
/// ```
///
/// [Github]: https://github.com/
pub struct Search {
    search_area: Option<String>,
    query: Option<String>,
    per_page: usize,
    page: usize,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    total_count: u64,
    items: Vec<Value>,
}

impl Search {
    /// Creates a new search configuration.
    ///
    /// # Available Choices for `area`
    /// - `"issues"`
    /// *More choices will be made available as this project continues.*
    /// *Other choices, such as `"users"`, are technically possible, but*
    /// *are not yet properly supported.*
    pub fn new(area: &str, query: &Query) -> Self {
        Search {
            search_area: Some(String::from(area)),
            query: Some(query.to_string()),
            ..Default::default()
        }
    }

    /// Defaults to 10.
    pub fn per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page;
        self
    }

    /// Defaults to 1.
    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    /// Moves one page forward.
    pub fn next_page(&mut self) {
        if self.page < std::usize::MAX {
            self.page += 1; 
        }
    }

    /// Moves one page backward.
    pub fn prev_page(&mut self) {
        if self.page > std::usize::MIN {
            self.page -= 1;
        }
    }

    /// Runs the search.
    pub fn search(&self) -> Result<SearchResults> {
        // TODO Return error if search_area or query are None
        let results: SearchResults = reqwest::get(&self.to_string())?.json()?;
        Ok(results)
    }
}

impl SearchResults {
    /// Gets total count of values matching query.
    ///
    /// This ignores `per_page`. If you only want the total count, it is
    /// recommended that you set `per_page` to `1` to shrink results size.
    pub fn total_count(&self) -> u64 {
        self.total_count
    }

    /// Items matching the query.
    pub fn items(&self) -> &Vec<Value> {
        &self.items
    }
}

impl Default for Search {
    fn default() -> Self {
        Search {
            search_area: None,
            query: None,
            per_page: 10,
            page: 1,
        }
    }
}

impl fmt::Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let search_area: &str = if let Some(area) = &self.search_area {
            area
        } else {
            ""
        };
        let query: &str = if let Some(query) = &self.query {
            query
        } else {
            ""
        };
        write!(
            f,
            "https://api.github.com/search/{0}?per_page={1}&page={2}&q={3}",
            search_area, self.per_page, self.page, query,
        )
    }
}
