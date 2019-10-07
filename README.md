# github-stats
A tool to get stats from Github

```rust
use github_stats::Repo;

let repo = Repo::new("rust-lang", "rust");

match repo {
    Ok(repo) => {/* Do some stuff */},
    Err(e) => eprintln!(":("),
}
```
