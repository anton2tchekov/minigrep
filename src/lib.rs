//! # Minigrep
//!
//! `minigrep` is a funny exercise to learn Rust syntax
//! by building a simple program to search for a certain
//! string in a given text file.

use std::error::Error;
use std::fs;

pub use self::wrapper::Config;

pub mod wrapper {
    use std::env;
    pub struct Config {
        pub query: String,
        pub file_path: String,
        pub ignore_case: bool,
    }

    impl Config {
        pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path"),
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    match config.ignore_case {
        true => {
            let result = search_case_insensitive(&config.query, &contents);
            println!("{result:?}");
        }
        false => {
            let result = search(&config.query, &contents);
            println!("{result:?}");
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search a query (string slice) in a given content (also a string slice).
/// It is case insensitive.
///
/// # Examples
///
/// ```
/// let query = "RuS";
/// let contents = "\
/// My Russian
/// Rust lover
/// Rustan.";
/// let find = minigrep::search_case_insensitive(query, contents);
///
/// assert_eq!(vec!["My Russian", "Rust lover", "Rustan."], find);
/// ```
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, duct, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, duct, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive_search() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
