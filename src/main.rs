extern crate fw;

use std::env;
use std::process;

use fw::Config;

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();
    if let Err(e) = fw::run(Config::new(args)) {
        eprintln!("{}: application error: {}", prog_name, e);
        process::exit(1);
    }
}
