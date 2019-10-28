use std::fmt;

use crate::Repo;

#[derive(Default)]
pub struct Query {
    author: Vec<String>,
    repo: Vec<String>,
    is: Vec<String>,
    r#in: Vec<In>,
    assignee: Vec<String>,
    user: Vec<String>,
    org: Vec<String>,
    fullname: Vec<FullName>,
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

    /// *Adds* an author to the query.
    ///
    /// Result is `author:username`.
    pub fn author(mut self, username: &str) -> Self {
        self.author.push(username.to_owned());
        self
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

    /// *Adds* an `assignee` statement to the query.
    ///
    /// Results in `assignee:username`.
    pub fn assignee(mut self, username: &str) -> Self {
        self.assignee.push(username.to_owned());
        self
    }

    /// *Adds* an `user` statement to the query.
    ///
    /// Results in `user:statement`.
    pub fn user(mut self, statement: &str) -> Self {
        self.user.push(String::from(statement));
        self
    }

    /// *Adds* an `org` statement to the query.
    ///
    /// Results in `org:statement`.
    pub fn org(mut self, statement: &str) -> Self {
        self.org.push(String::from(statement));
        self
    }

    /// *Adds* a `fullname` statement to the query.
    ///
    /// Results in `fullname:first_name last_name`.
    pub fn fullname(mut self, first_name: &str, last_name: &str) -> Self {
        self.fullname.push(FullName::new(first_name, last_name));
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
            let mut author: Vec<_> = self.author.iter().map(|s| format!("author:{}", s)).collect();
            let mut repo: Vec<String> = self.repo.iter().map(|s| format!("repo:{}", s)).collect();
            let mut is: Vec<String> = self.is.iter().map(|s| format!("is:{}", s)).collect();
            let mut r#in: Vec<String> = self.r#in.iter().map(|s| s.to_string()).collect();
            let mut user: Vec<String> = self.user.iter().map(|s| format!("user:{}", s)).collect();
            let mut assignee: Vec<String> = self.assignee.iter().map(|s| format!("assignee:{}", s)).collect();
            let mut org: Vec<String> = self.org.iter().map(|s| format!("org:{}", s)).collect();
            let mut fullname: Vec<String> = self.fullname.iter().map(|s| s.to_string()).collect();
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
                Vec::with_capacity(
                author.len()
                    + repo.len()
                    + is.len()
                    + r#in.len()
                    + assignee.len()
                    + user.len()
                    + org.len()
                    + fullname.len()
                    + label.len()
                    + r#type.len()
                    + state.len()
                    + no.len()
                    + language.len()
                );

            queries.append(&mut repo);
            queries.append(&mut is);
            queries.append(&mut r#in);
            queries.append(&mut author);
            queries.append(&mut assignee);
            queries.append(&mut user);
            queries.append(&mut org);
            queries.append(&mut fullname);
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

struct FullName(String, String);

impl FullName {
    fn new(first_name: &str, last_name: &str) -> Self {
        FullName(String::from(first_name), String::from(last_name))
    }
}

impl fmt::Display for FullName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fullname:{} {}", self.0, self.1)
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
            .assignee("spenserblack")
            .user("spenserblack")
            .author("spenserblack")
            .language("rust")
            .to_string();

        assert_eq!(
            "repo:rust-lang/rust+is:merged+[BUG] in:name+author:spenserblack+assignee:spenserblack+user:spenserblack+label:hacktoberfest+type:pr+no:assignee+language:rust",
            query
        );
    }

    #[test]
    fn org_query() {
        let query = Query::new()
            .org("rust-lang")
            .to_string();

        assert_eq!(
            "org:rust-lang",
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

    #[test]
    fn fullname_string() {
        let fullname = FullName::new(
            "Octo",
            "Cat",
        );

        assert_eq!("fullname:Octo Cat", fullname.to_string());
    }
}
