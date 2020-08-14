use clap::{App, Arg};
use github_stats::User;

#[tokio::main]
async fn main() {
    let matches = App::new("example")
        .arg(
            Arg::with_name("user").index(1).required(true)
        )
        .get_matches();

    let user = matches.value_of("user").unwrap();

    println!("Getting user {}", user);
    let user = User::new(user, "github-stats-rs example").await.unwrap();

    println!("API URL: {}", user.url());
}
