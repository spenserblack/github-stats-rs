use std::collections::HashMap;

use crate::{JsonMap, Result, Response};


#[doc(hidden)]
pub fn from_api_url(url: &str) -> Result<JsonMap<u64>> {
    let response: Response = reqwest::get(url)?.json()?;
    let response = response
        .as_object()
        .ok_or(format!("Cannot read results of {:?} as JSON object", url))?;
    // let map: JsonMap<u64> = response.iter().collect();
    let mut map: JsonMap<u64> = HashMap::new();
    for (key, value) in response.iter() {
        let value = value
            .as_u64()
            .ok_or(format!("Couldn't parse value of {:?} to u64", key))?;
        map.insert(String::from(key), value);
    }
    Ok(map)
}

pub fn from_user_repo(user: &str, repo: &str) -> Result<JsonMap<u64>> {
    let url = format!("https://api.github.com/repos/{0}/{1}/languages", user, repo);

    from_api_url(&url)
}
