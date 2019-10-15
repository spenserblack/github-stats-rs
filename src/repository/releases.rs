//! For getting release information.

use chrono::NaiveDateTime;

use crate::{gh_datestr_to_chrono, Response, Result};

/// Represents the stats of a [Github] release.
///
/// All releases are tags, but *not all tags are releases.*
///
/// [Github]: https://github.com/
#[derive(Debug)]
pub struct Release {
    tag: String,
    name: String,
    published: NaiveDateTime,
}

impl Release {
    /// The latest release of a repository.
    ///
    /// `None` if there is no valid latest release.
    pub fn latest(user: &str, repo: &str) -> Result<Option<Release>> {
        let url = &format!("https://api.github.com/repos/{0}/{1}/releases/latest", user, repo);

        let response: Response = reqwest::get(url)?.json()?;

        if let None = response.get("id") {
            return Ok(None);
        }

        let tag = response["tag_name"].as_str().ok_or(r#""tag_name" is not a string"#)?.to_string();
        let name = response["name"].as_str().ok_or(r#""name" is not a string"#)?.to_string();
        let published = gh_datestr_to_chrono(
            response["published_at"]
                .as_str()
                .ok_or(r#""published_at" is not a string"#)?
        )?;

        let release = Release {
            tag,
            name,
            published,
        };

        Ok(Some(release))
    }

    /// The tag associated with the release.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// The name of the release.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// When the release was published.
    pub fn published(&self) -> &NaiveDateTime {
        &self.published
    }
}
