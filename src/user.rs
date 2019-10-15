//! For getting user information.

use serde::Deserialize;

/// Represents that stats of a [Github] user.
///
/// [Github]: https://github.com/
#[derive(Debug, Deserialize)]
pub struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    html_url: String,
    r#type: String,
}
