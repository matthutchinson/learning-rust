extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // passing &args (slice of String references)
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // skip first arg and collect as Vec
    // see http://tinyurl.com/y77cwx3e for tip
    let args = env::args().collect::<Vec<_>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // handle error with match like so
    // match run(config) {
    //     Err(e) => {
    //         println!("Application error: {}", e);
    //         process::exit(1);
    //     },
    //     _ => { }
    // }
    // or use the if let syntax
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
