use std::env;
use std::process;

use minigrep;

fn main() {
    let args: env::Args = env::args();
    let config = minigrep::Config::new(args).unwrap_or_else(|err| {
        eprintln!("parse args error : {}", err);
        process::exit(1);
    });
    print!("searching for {} ", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("application run error {}", e);
        process::exit(1);
    };
}
