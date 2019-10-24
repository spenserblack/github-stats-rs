use std::fmt;

use crate::Repo;

#[derive(Default)]
pub struct Query {
    repo: Vec<String>,
    is: Vec<String>,
    r#in: Vec<In>,
    user: Vec<String>,
    label: Vec<String>,
    r#type: Vec<String>,
    state: Vec<String>,
    no: Vec<String>,
    language: Vec<String>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            ..Default::default()
        }
    }

    pub fn from_repo(repo: Repo) -> Self {
        let repo = vec![String::from(repo.full_name())];
        Query {
            repo,
            ..Default::default()
        }
    }

    /// *Adds* a repo to the query.
    ///
    /// Results in `repo:user/repo`.
    pub fn repo(mut self, user: &str, repo: &str) -> Self {
        self.repo.push(format!("{}/{}", user, repo));
        self
    }

    /// *Adds* an `is` statement to the query.
    ///
    /// Results in `is:statement`.
    pub fn is(mut self, statement: &str) -> Self {
        self.is.push(String::from(statement));
        self
    }

    /// *Adds* an `in` statement to the query
    ///
    /// Results in `keyword in:field`.
    pub fn r#in(mut self, keyword: &str, field: &str) -> Self {
        self.r#in.push(In(String::from(keyword), String::from(field)));
        self
    }

    /// *Adds* an `user` statement to the query.
    ///
    /// Results in `user:statement`.
    pub fn user(mut self, statement: &str) -> Self {
        self.user.push(String::from(statement));
        self
    }

    /// *Adds* a `label` statement to the query.
    ///
    /// Results in `label:statement`.
    pub fn label(mut self, statement: &str) -> Self {
        self.label.push(String::from(statement));
        self
    }

    /// *Adds* a `type` statement to the query.
    ///
    /// Results in `type:statement`.
    ///
    /// *Use `r#type` to escape `type` keyword.
    pub fn r#type(mut self, statement: &str) -> Self {
        self.r#type.push(String::from(statement));
        self
    }

    /// *Adds* a `no` statement to the query.
    ///
    /// Results in `no:statement`.
    pub fn no(mut self, statement: &str) -> Self {
        self.no.push(String::from(statement));
        self
    }

    /// *Adds* a `language` statement to the query.
    ///
    /// Results in `language:statement`.
    pub fn language(mut self, statement: &str) -> Self {
        self.language.push(String::from(statement));
        self
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let queries = {
            let mut repo: Vec<String> = self.repo.iter().map(|s| format!("repo:{}", s)).collect();
            let mut is: Vec<String> = self.is.iter().map(|s| format!("is:{}", s)).collect();
            let mut r#in: Vec<String> = self.r#in.iter().map(|s| s.to_string()).collect();
            let mut user: Vec<String> = self.user.iter().map(|s| format!("user:{}", s)).collect();
            let mut label: Vec<String> = self.label.iter().map(|s| format!("label:{}", s)).collect();
            let mut r#type: Vec<String> =
                self.r#type.iter().map(|s| format!("type:{}", s)).collect();
            let mut state: Vec<String> =
                self.state.iter().map(|s| format!("state:{}", s)).collect();
            let mut no: Vec<String> =
                self.no.iter().map(|s| format!("no:{}", s)).collect();
            let mut language: Vec<String> =
                self.language.iter().map(|s| format!("language:{}", s)).collect();

            let mut queries: Vec<String> =
                Vec::with_capacity(repo.len() + is.len() + r#in.len() + user.len() + label.len() + r#type.len() + state.len()
                    + no.len() + language.len());

            queries.append(&mut repo);
            queries.append(&mut is);
            queries.append(&mut r#in);
            queries.append(&mut user);
            queries.append(&mut label);
            queries.append(&mut r#type);
            queries.append(&mut state);
            queries.append(&mut no);
            queries.append(&mut language);
            queries
        };

        let queries = queries.join("+");

        write!(f, "{}", queries)
    }
}

struct In(String, String);

impl fmt::Display for In {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} in:{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_query() {
        let query = Query::new()
            .repo("rust-lang", "rust")
            .r#type("pr")
            .is("merged")
            .label("hacktoberfest")
            .no("assignee")
            .r#in("[BUG]", "name")
            .user("spenserblack")
            .language("rust")
            .to_string();

        assert_eq!(
            "repo:rust-lang/rust+is:merged+[BUG] in:name+user:spenserblack+label:hacktoberfest+type:pr+no:assignee+language:rust",
            query
        );
    }

    #[test]
    fn in_string() {
        let r#in = In(
            String::from("Users"),
            String::from("title"),
        );

        assert_eq!("Users in:title", r#in.to_string());
    }
}
