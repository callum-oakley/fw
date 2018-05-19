extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!(
            "{}: problem parsing arguments: {}",
            prog_name, err
        );
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("{}: application error: {}", prog_name, e);
        process::exit(1);
    }
}
