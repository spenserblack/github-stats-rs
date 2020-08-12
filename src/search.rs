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
/// ```no_run
/// # async fn run() {
/// use github_stats::{Query, Search};
///
/// let query = Query::new()
///     .repo("rust-lang", "rust")
///     .is("pr")
///     .is("merged");
///
/// let results = Search::issues(&query)
///     .per_page(10)
///     .page(1)
///     .search()
///     .await;
///
/// match results {
///     Ok(results) => { /* do stuff */ }
///     Err(e) => eprintln!(":("),
/// }
/// # }
/// ```
///
/// [Github]: https://github.com/
pub struct Search {
    search_area: SearchArea,
    query: String,
    per_page: usize,
    page: usize,
}

enum SearchArea {
    Issues,
    Users,
}

impl fmt::Display for SearchArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SearchArea::*;
        write!(f, "{}", match self {
            Issues => "issues",
            Users => "users",
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    total_count: u64,
    items: Vec<Value>,
}

impl Search {
    fn new(search_area: SearchArea, query: &Query) -> Self {
        Search {
            search_area,
            query: query.to_string(),
            per_page: 10,
            page: 1,
        }
    }

    pub fn issues(query: &Query) -> Self {
        Search::new(SearchArea::Issues, query)
    }

    pub fn users(query: &Query) -> Self {
        Search::new(SearchArea::Users, query)
    }

    /// Gets the query that will be used for the search.
    pub fn get_query(&self) -> &str {
        &self.query
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
    pub async fn search(&self) -> Result<SearchResults> {
        let results: SearchResults = reqwest::get(&self.to_string()).await?.json().await?;
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

impl fmt::Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let search_area = match self.search_area {
            SearchArea::Issues => "issues",
            SearchArea::Users => "users",
        };
        write!(
            f,
            "https://api.github.com/search/{0}?per_page={1}&page={2}&q={3}",
            search_area, self.per_page, self.page, self.query,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_search() {
        const EXPECTED: &str = "https://api.github.com/search/issues?per_page=1&page=1&q=repo:rust-lang/rust+is:pr+is:merged";
        let search = Search::issues(
            &Query::new().repo("rust-lang", "rust").is("pr").is("merged"),
        )
        .per_page(1);

        assert_eq!(EXPECTED, search.to_string());
    }
}
