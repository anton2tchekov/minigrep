use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn config_new_should_fail() {
        let args: Vec<String> = vec![String::from("program"), String::from("search")];
        Config::new(&args);
    }

    #[test]
    fn config_new_should_succeed() {
        let args: Vec<String> = vec![
            String::from("program"),
            String::from("search"),
            String::from("file.txt"),
        ];
        let conf = Config::new(&args)?;
        assert_eq!(conf.query, "search");
        assert_eq!(conf.file_path, "file.txt");
    }
}
