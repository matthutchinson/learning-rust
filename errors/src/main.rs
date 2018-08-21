use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// or do the same as above with ? to propogate the error e.g.

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // the ?'s ensure errors are returned early
    // to use this the method must return a Result type
    File::open("hello.txt")?.read_to_string(&mut s)?;
    // return the Ok result, no chance of an error now
    Ok(s)
}

fn main() {
    // returns an std::result::Result<std::fs::File, std::io::Error>
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // creates file if it doesn't exist
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let username = read_username_from_file().unwrap();
    println!("name read was {:?}", username);
}

