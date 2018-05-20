use std::error::Error;
use std::io::{self, Read};

pub mod query;

pub use query::Query;

#[derive(Debug, PartialEq)]
pub enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

impl SearchMode {
    pub fn smart(s: &str) -> Self {
        if s.chars().any(char::is_uppercase) {
            SearchMode::CaseSensitive
        } else {
            SearchMode::CaseInsensitive
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    query: Query,
    search_mode: SearchMode,
}

impl Config {
    pub fn new(query: Query, search_mode: SearchMode) -> Config {
        Config {
            query,
            search_mode,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;

    let results = search(config, &buffer);

    print!("{}", results);

    Ok(())
}

pub fn search(
    Config {
        query,
        search_mode,
    }: Config,
    contents: &str,
) -> String {
    let normalized_contents = match search_mode {
        SearchMode::CaseInsensitive => contents.to_lowercase(),
        SearchMode::CaseSensitive => contents.to_string(),
    };

    let query = match search_mode {
        SearchMode::CaseInsensitive => query.to_lowercase(),
        SearchMode::CaseSensitive => query,
    };

    let matches = contents
        .lines()
        .zip(normalized_contents.lines())
        .filter(|(_, normalized_line)| query.matches(normalized_line))
        .map(|(line, _)| line); // TODO return a SearchResult for rendering

    let mut result = String::new();
    for m in matches {
        result.push_str(m);
        result.push('\n');
    }

    result
}
