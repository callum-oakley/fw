extern crate clap;
extern crate qw;

use clap::App;
use std::env;
use std::process;

use qw::{Config, Query, SearchMode};

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();

    let matches = App::new(prog_name.clone())
        .version("0.1.0")
        .about("Query for words")
        .args_from_usage(
            "<QUERY> 'Query to search for'
            -i, --insensitive 'Case insensitive'
            -s, --sensitive 'Case sensitive'",
        )
        .get_matches();

    let raw_query = matches.value_of("QUERY").unwrap();

    let query = Query::new(raw_query).unwrap_or_else(|e| {
        eprintln!("{}: error parsing query: {}", prog_name, e);
        process::exit(1);
    });

    let search_mode = if matches.is_present("sensitive")
        && matches.is_present("insensitive")
    {
        eprintln!(
            "{}: can't search both case sensitive and case insensitive!",
            prog_name
        );
        process::exit(1);
    } else if matches.is_present("sensitive") {
        SearchMode::CaseSensitive
    } else if matches.is_present("insensitive") {
        SearchMode::CaseInsensitive
    } else {
        SearchMode::smart(raw_query)
    };

    if let Err(e) = qw::run(Config::new(query, search_mode)) {
        eprintln!("{}: application error: {}", &prog_name, e);
        process::exit(1);
    }
}
