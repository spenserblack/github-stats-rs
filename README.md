# github-stats
[![Crates.io](https://img.shields.io/crates/v/github-stats)](https://crates.io/crates/github-stats)
[![docs.rs](https://docs.rs/github-stats/badge.svg)](https://docs.rs/github-stats/)
![Crates.io](https://img.shields.io/crates/d/github-stats)

[![Travis CI](https://api.travis-ci.com/spenserblack/github-stats-rs.svg?branch=master)](https://travis-ci.com/spenserblack/github-stats-rs)
[![GitHub Release Date](https://img.shields.io/github/release-date/spenserblack/github-stats-rs)](https://github.com/spenserblack/github-stats-rs/releases/latest)

A tool to get stats from Github

## Get Stats of Repository

```rust
use github_stats::Repo;

let repo = Repo::new("rust-lang", "rust");

match repo {
    Ok(repo) => {/* Do some stuff */},
    Err(e) => eprintln!(":("),
}
```

## Search Latest Merged PR and Get Total Merged PR Count

```rust
use github_stats::{Query, Search};

// Gets latest merged PR
let search = Search::new(
    "issues",
    &Query::new().repo("rust-lang", "rust").is("pr").is("merged"),
)
.per_page(1)
.search();

match search {
    Ok(results) => println!("# of merged PRs: {}", results.total_count()),
    Err(e) => eprintln!(":("),
}
```
