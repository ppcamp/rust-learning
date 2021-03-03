#![allow(unused_variables)]
#![allow(dead_code)]

// to get **argv (in c++)
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).expect("Some error ocurred");
    // If not found a CASE_INSENSITIVE in env, will put 0
    let case_sensitive = env::var("CASE_INSENSITIVES").unwrap_or_else(|op| "0".to_string());
    println!("{:?}", case_sensitive);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

fn read_file(filename: &String) -> String {
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename);
    match contents {
        Ok(content) => content,
        Err(reason) => panic!(reason),
    }
}
