use clap::{App, Arg};
use github_stats::{Query, Search};

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

    println!("Getting (almost) total merged PRs of {}/{}", owner, repo);

    let query = Query::new()
        .repo(owner, repo)
        .is("pr")
        .is("merged");

    let search = Search::issues(&query)
        .per_page(10)
        .page(1);

    let search = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => {
            println!("Using authorization token for search.");
            search.authorization(&token)
        },
        Err(_) => {
            println!("No authorization token found.");
            search
        }
    };

    println!("Running search: {}", search);
    let results = search
        .search("github-stats-rs example")
        .await
        .unwrap();


    println!("Total merged PRs: {}", results.total_count());
}
