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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let mut o: Vec<String> = vec![];
        let mut p: Vec<String> = vec![];

        // let foo: Vec<String> = vec!["a".to_string(), "b".to_string()];
        // let foox: Vec<String> = args.collect();

        // let active_classes: Vec<&String> = foo.iter().filter(|x| x == &&"a".to_string()).collect();
        // let active_classes: Vec<&String> = foo.iter().filter(|x| x != &&"a".to_string()).collect();
        // let non_active_classes: Vec<_> = foo.iter().filter(|x| x.status != Status::Active).collect();

        // // fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // //     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        // // }

        // let args: Vec<String> = env::args().collect();
        // let p: Vec<&String> = foox
        //     .iter()
        //     .filter(|&item| item.contains("--") || item.contains("-"))
        //     .collect();
        // let o: Vec<&String> = foox
        //     .iter()
        //     .filter(|&item| !item.contains("--") || !item.contains("-"))
        //     .collect();
        // let o: Vec<String> = args.filter(|item| item.contains("--") || item.contains("-")).collect();

        for item in args {
            if item.contains("--") || item.contains("-") {
                p.push(item);
            } else {
                o.push(item)
            }
        }

        let p_iter = p.into_iter();
        let mut o_iter = o.into_iter();

        let mut case_sensitive = true;
        if !env::var("CASE_INSENSITIVE").is_err() {
            case_sensitive = false;
        }
        for w in p_iter {
            match &w as &str {
                "-i" => case_sensitive = false,
                _ => (),
            }
        }

        let query = match o_iter.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match o_iter.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        println!("{:?}", case_sensitive);

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
    // #[test]
    // fn t_case_sensitive() {
    //     let args: Vec<String> = vec!["-i".to_string(), "foo".to_string(), "faa".to_string()];
    //     assert!(!is_case_sensitive(&args))
    // }
}
