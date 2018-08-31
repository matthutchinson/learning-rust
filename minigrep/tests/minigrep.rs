extern crate minigrep;

#[cfg(test)]
mod test {
    use minigrep::*;

    #[test]
    fn valid_config() {
        let args = vec!["cmd".to_string(), "needle".to_string(), "haystack".to_string()];
        let config = Config::new(&args).unwrap();

        assert_eq!("needle", config.query);
        assert_eq!("haystack", config.filename);
        assert_eq!(false, config.case_sensitive);
    }

    // unsure how to test Err returned from Config::new
    // #[test]
    // fn invalid_config() {
    //     let args = vec!["".to_string()];
    //     let config = Config::new(&args);
    //     assert_eq!(config, Err("sss"));
    // }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape"],
            search_case_insensitive(query, contents)
        );
    }
}
