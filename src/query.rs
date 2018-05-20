#[derive(Debug, PartialEq)]
pub enum Query {
    String(String),
}

impl Query {
    pub fn new(s: &str) -> Result<Query, &'static str> {
        Ok(Query::String(s.to_string()))
    }

    pub fn matches(&self, s: &str) -> bool {
        match self {
            Query::String(q) => s.contains(q),
        }
    }

    pub fn to_lowercase(&self) -> Self {
        match self {
            Query::String(q) => Query::String(q.to_lowercase()),
        }
    }
}
