use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

pub struct Config {
    query: String,
    filename: String,
    search_mode: SearchMode,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("please provide a query string!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("please provide a file name!"),
        };

        Ok(Config {
            query,
            filename,
            search_mode: if env::var("CASE_INSENSITIVE").is_err() {
                SearchMode::CaseSensitive
            } else {
                SearchMode::CaseInsensitive
            },
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = search(&config.query, &contents, config.search_mode);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    mode: SearchMode,
) -> Vec<&'a str> {
    let query = match mode {
        SearchMode::CaseSensitive => query.to_string(),
        SearchMode::CaseInsensitive => query.to_lowercase(),
    };

    contents
        .lines()
        .filter(|line| match mode {
            SearchMode::CaseSensitive => line.contains(&query),
            SearchMode::CaseInsensitive => line.to_lowercase().contains(&query),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, SearchMode::CaseSensitive),
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, SearchMode::CaseInsensitive),
        );
    }
}
