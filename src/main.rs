use std::env;
use std::error::Error;
use std::fs;
use std::process;

use simple_grep::search;

struct Config {
    // we could have a reference of string and manage the lifetime.
    query: String,
    fp: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            // Interesting, this is a lower case, the start of the error is defined in the main()
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].to_string(),
            fp: args[2].to_string(),
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.fp)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
