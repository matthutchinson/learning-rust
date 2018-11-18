use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;


fn read_username_from_file(f: &mut File) -> Result<String, io::Error> {
    let mut s = String::new();
    // the ?'s ensure errors are returned early
    // to use this the method must return a Result type
    f.read_to_string(&mut s)?;
    // return the Ok result, no chance of an error now
    Ok(s)
}

// use std::fs;
// fn read_username_from_file_std_lib_method() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

fn main() {
    // returns an std::result::Result<std::fs::File, std::io::Error>
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // creates file if it doesn't exist
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("Tried to create file but there was a problem: {:?}", e)
                    },
                }
            },
            other_error => {
                panic!("There was a problem opening the file: {:?}", other_error)
            },
        }
    };

    // or the above is possible with closure methods map_err(|error| and unwrap_or_else(|error|
    // let f = File::open("hello.txt").map_err(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // });

    let username = read_username_from_file(&mut f).unwrap();
    println!("name read was {:?}", username);
}

