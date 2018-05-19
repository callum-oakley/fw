use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
pub enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

impl SearchMode {
    fn smart(s: &str) -> Self {
        if s.chars().any(char::is_uppercase) {
            SearchMode::CaseSensitive
        } else {
            SearchMode::CaseInsensitive
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    queries: Vec<String>,
    search_mode: SearchMode,
}

impl Config {
    pub fn new<I>(args: I) -> Config
    where
        I: Iterator<Item = String>,
    {
        let mut queries = Vec::new();
        let mut no_more_flags = false;
        let mut search_mode: Option<SearchMode> = None;

        for arg in args {
            if no_more_flags {
                queries.push(arg);
            } else {
                match arg.as_ref() {
                    "--" => no_more_flags = true,
                    "-s" => search_mode = Some(SearchMode::CaseSensitive),
                    "-i" => search_mode = Some(SearchMode::CaseInsensitive),
                    _ => queries.push(arg),
                }
            }
        }

        let search_mode = match search_mode {
            Some(mode) => mode,
            None => SearchMode::smart(&concat(&queries)),
        };

        Config {
            queries,
            search_mode,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;

    let results = search(&config.queries, &buffer, config.search_mode);

    print!("{}", results);

    Ok(())
}

pub fn search(
    queries: &Vec<String>,
    contents: &str,
    mode: SearchMode,
) -> String {
    let queries = match mode {
        SearchMode::CaseSensitive => queries.clone(),
        SearchMode::CaseInsensitive => queries
            .iter()
            .map(|query| query.to_lowercase())
            .collect(),
    };

    contents
        .lines()
        .filter(|line| match mode {
            SearchMode::CaseSensitive => {
                queries.iter().all(|query| line.contains(query))
            }
            SearchMode::CaseInsensitive => {
                let line = line.to_lowercase();
                queries.iter().all(|query| line.contains(query))
            }
        })
        .rev()
        .fold("".to_string(), |acc, line| {
            format!("{}\n{}", line, acc)
        })
}

fn concat(s: &Vec<String>) -> String {
    let mut res = String::new();
    for t in s {
        res.push_str(t);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let queries = vec!["duct".to_string()];
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            "safe, fast, productive.\n",
            search(&queries, contents, SearchMode::CaseSensitive),
        );
    }

    #[test]
    fn search_case_insensitive() {
        let queries = vec!["rUsT".to_string()];
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            "Rust:\nTrust me.\n",
            search(&queries, contents, SearchMode::CaseInsensitive),
        );
    }

    #[test]
    fn search_multiple_queries() {
        let queries = vec!["us".to_string(), "me".to_string()];
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            "Trust me.\n",
            search(&queries, contents, SearchMode::CaseInsensitive),
        )
    }

    #[test]
    fn smart_case_lower() {
        assert_eq!(
            SearchMode::CaseInsensitive,
            SearchMode::smart("hello!"),
        )
    }

    #[test]
    fn smart_case_mixed() {
        assert_eq!(
            SearchMode::CaseSensitive,
            SearchMode::smart("Hello!"),
        )
    }

    #[test]
    fn config_all_queries() {
        let args: Vec<String> = vec!["aaa", "bbb", "ccc"]
            .into_iter()
            .map(String::from)
            .collect();
        let queries = args.clone();
        assert_eq!(
            Config::new(args.into_iter()),
            Config {
                queries,
                search_mode: SearchMode::CaseInsensitive,
            },
        )
    }

    #[test]
    fn config_sensitive_flag() {
        let args: Vec<String> = vec!["aaa", "bbb", "-s"]
            .into_iter()
            .map(String::from)
            .collect();
        let queries: Vec<String> = vec!["aaa", "bbb"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(
            Config::new(args.into_iter()),
            Config {
                queries,
                search_mode: SearchMode::CaseSensitive,
            },
        )
    }

    #[test]
    fn config_sensitive_flag_as_query() {
        let args: Vec<String> = vec!["aaa", "--", "-s"]
            .into_iter()
            .map(String::from)
            .collect();
        let queries: Vec<String> = vec!["aaa", "-s"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(
            Config::new(args.into_iter()),
            Config {
                queries,
                search_mode: SearchMode::CaseInsensitive,
            },
        )
    }

    #[test]
    fn config_smart_case() {
        let args: Vec<String> = vec!["aaa", "bBb", "ccc"]
            .into_iter()
            .map(String::from)
            .collect();
        let queries = args.clone();
        assert_eq!(
            Config::new(args.into_iter()),
            Config {
                queries,
                search_mode: SearchMode::CaseSensitive,
            },
        )
    }

    #[test]
    fn config_insensitive_flag() {
        let args: Vec<String> = vec!["aaa", "bBb", "-i"]
            .into_iter()
            .map(String::from)
            .collect();
        let queries: Vec<String> = vec!["aaa", "bBb"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(
            Config::new(args.into_iter()),
            Config {
                queries,
                search_mode: SearchMode::CaseInsensitive,
            },
        )
    }
}
