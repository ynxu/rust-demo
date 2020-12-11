use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didnot get a query String")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didnot get a filename String")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("case_sensitive {}", case_sensitive);
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         result.push(line)
    //     }
    // }
    // result
    content.lines().filter(|line|line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines().filter(|line|line.to_lowercase().contains(&query.to_lowercase())).collect()
    // contents.lines().map(|line:&'a str|->&'a str{&(line.to_lowercase())[..]}).filter(|line|line.contains(&query.to_lowercase())).collect()
}
