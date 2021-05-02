use std::error::Error;
use std::{env, fs};

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
            None => return Err("Didn't get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };

        // Get from env variable if is case sensitive.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search_internal<'a, F>(contents: &'a str,  filter_fun: F) -> Vec<(usize, &'a str)> 
where 
    F: Fn(&(usize, &str)) -> bool
{
    contents
        .lines()
        .enumerate()
        .map(|(i, line)| (i + 1, line))
        .filter(filter_fun)
        .collect()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    search_internal(contents, |(_, line)| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    search_internal(contents, |(_, line)| line.to_lowercase().contains(&query))
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("Line {}:\t{}", line.0, line.1);
    }

    Ok(())
}

#[cfg(test)]
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
    fn no_results() {
        let query = "aaaaaaaaaa";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<(usize, &str)>::new(), search(query, contents));
    }

    #[test]
    fn two_results() {
        let query = "u";
        let contents = "\
Rust:
safe, fast, productive.
Pick three elements.";

        assert_eq!(2, search(query, contents).len());
        assert_eq!(
            vec![(1, "Rust:"), (2, "safe, fast, productive.")],
            search(query, contents)
        )
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(query, contents));
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
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }
}
