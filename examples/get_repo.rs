use clap::{App, Arg};
use github_stats::Repo;

#[tokio::main]
async fn main() {
    let matches = App::new("example")
        .arg(
            Arg::with_name("owner").index(1).required(true)
        )
        .arg(
            Arg::with_name("repo").index(2).required(true)
        )
        .get_matches();

    let owner = matches.value_of("owner").unwrap();
    let repo = matches.value_of("repo").unwrap();

    println!("Getting repo {}/{}", owner, repo);
    let repo = Repo::new(owner, repo, "github-stats-rs example").await.unwrap();

    println!("API URL: {}", repo.html_url());
}
