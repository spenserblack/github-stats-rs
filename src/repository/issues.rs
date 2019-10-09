use crate::{Response, Result};

pub fn issue_stats(user: &str, repo: &str) -> Result<(u64, u64, u64, u64)> {
    const URL: &str = "https://api.github.com/search/issues?per_page=1&q=";
    let open_issues_url = format!("{0}repo:{1}/{2}+type:issue+state:open", URL, user, repo);
    let closed_issues_url = format!("{0}repo:{1}/{2}+type:issue+state:closed", URL, user, repo);
    let open_pull_requests_url = format!("{0}repo:{1}/{2}+type:pr+state:open", URL, user, repo);
    let closed_pull_requests_url = format!("{0}repo:{1}/{2}+type:pr+state:closed", URL, user, repo);
    let open_issues_res: Response = reqwest::get(&open_issues_url)?.json()?;
    let closed_issues_res: Response = reqwest::get(&closed_issues_url)?.json()?;
    let open_pull_requests_res: Response = reqwest::get(&open_pull_requests_url)?.json()?;
    let closed_pull_requests_res: Response = reqwest::get(&closed_pull_requests_url)?.json()?;

    let open_issues = open_issues_res["total_count"]
        .as_u64()
        .ok_or("Cannot get open issue count")?;
    let closed_issues = closed_issues_res["total_count"]
        .as_u64()
        .ok_or("Cannot get closed issue count")?;
    let open_pull_requests = open_pull_requests_res["total_count"]
        .as_u64()
        .ok_or("Cannot get open pull request count")?;
    let closed_pull_requests = closed_pull_requests_res["total_count"]
        .as_u64()
        .ok_or("Cannot get closed pull request count")?;

    Ok((
        open_issues,
        closed_issues,
        open_pull_requests,
        closed_pull_requests,
    ))
}
