use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // returns a Result<T, E>, OK gets unwrapped and Err is handled
    // i.e. Result<OK(_), Err(e)>
    // takes a slice of Strings returns a Result with Config
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("usage: minigrep [-c] query filename");
        }

        // clone was needed here, when we passed `&args` (a slice with String elements in the
        // parameter args) but this new function didn't own args
        // remember: slice is a list of consectutive references in memory
        // let args_length = args.len();
        // let query = args[args_length-2].clone();
        // let filename = args[args_length-1].clone();

        // jump past 1st arg (command name)
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        // if set, we'll switch to case insenstive
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // if we have more than 3 args, handle flags in initial args
        // cmd line arg has precedence over env var if specified
        // if -c passed we'll ALWAYS make search case sensitive, regardless of env var
        // if args.len() > 3 {
        //     if args[1..args_length-2].contains(&String::from("-c")) {
        //         case_sensitive = true;
        //     }
        // }

        // Config { query: query, filename: filename } would also work here
        Ok(Config { query, filename, case_sensitive })
    }
}


// Box<Error> return a type that implements Error trait, but we don't have to say which type the
// return value will be; remember ? will return early, any error from the current function
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // using iterators and collecting
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()

    // or with a for loop over lines() iterator, building a results Vec
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // shadowed variable with the same name
    let query = query.to_lowercase(); // to_lowercase returns a String (not string slice)
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
