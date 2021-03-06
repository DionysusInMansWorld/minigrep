use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, &'static str> {
        let mut args = args.skip(1);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a file name"),
        };

        let case_sensitive = if let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" => false,
                "-s" => true,
                _ => return Err("illegal arugments"),
            }
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if result.is_empty() {
        println!("the file is empty");
    } else {
        let len = result.len();

        for (i, line) in result {
            println!("\"{}\" at line {}", line, i);
        }

        println!("has searched {} lines matched", len);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    search_iter(contents)
        .filter(|item| item.1.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = &query.to_lowercase();

    search_iter(contents)
        .filter(|item| item.1.to_lowercase().contains(query))
        .collect()
}

#[inline]
fn search_iter(contents: &str) -> impl Iterator<Item = (usize, &str)> {
    contents
        .lines()
        .enumerate()
        .map(|(i, line)| (i + 1, line))
}

mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec![(1, "Rust:")], search_case_insensitive(query, contents))
    }
}