use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut o: Vec<String> = vec![];
        let mut p: Vec<String> = vec![];

        for item in args {
            if item.contains("--") || item.contains("-") {
                p.push(item.clone());
            } else {
                o.push(item.clone())
            }
        }

        if o.len() < 2 {
            return Err("not enough arguments");
        }
        let query = o[0].clone();
        let filename = o[1].clone();

        let case_sensitive = is_case_sensitive(&p);
        println!("{:?}", case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

fn is_case_sensitive(args: &[String]) -> bool {
    if env::var("CASE_INSENSITIVE").is_err() {
        return false;
    }
    for item in args {
        if item == "-i" {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
    #[test]
    fn t_case_sensitive() {
        let args: Vec<String> = vec!["-i".to_string(), "foo".to_string(), "faa".to_string()];
        assert!(!is_case_sensitive(&args))
    }
}
