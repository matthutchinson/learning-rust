// Using a hash map and vectors, create a text interface to allow a user to add employee names to a
// department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.

// hash of departments with growable vector of names
// two commands, add, and list
//
// add  => Add {name} to {department}
// list => List {department}
//      => List

use std::process;
use std::io;
use std::io::Write;
use std::collections::HashMap;

enum Command {
    Add { name: String, department: String },
    ListDepartment(String),
    List,
    Help,
    Exit,
    None,
}

impl Command {
    fn call(&self, directory: &mut HashMap<String, Vec<String>>) {
        match self {
            Command::Add{name, department} => {
                let mut names = directory.entry(department.to_string()).or_insert(Vec::new());
                names.push(name.to_string());
                println!("OK. I added {} to {}", name, department);
            },
            Command::List => {
                if directory.is_empty() {
                    println!("The directory is empty, why not add someone?");
                } else {
                    for (department, names) in directory.iter() {
                        print_department(department, names);
                    }
                }
            },
            Command::ListDepartment(department) => {
                match directory.get(department) {
                    Some(names) => print_department(department, names),
                    _ => println!("The '{}' department was not found in the directory", department)
                }
            },
            Command::Help => {
                println!("List - lists all employees");
                println!("List {{department}} - lists all employees in a department");
                println!("Add {{name}} to {{department}} - adds an employee by name to a department");
                println!("Help - shows available commands");
                println!("Exit - quits the program");
            },
            Command::Exit => process::exit(0x0100), // multi-platform exit code
            _ => {} // do nothing for all other commands, including the None command
        }
    }
}

fn print_department(department: &str, names: &Vec<String>) {
    println!("--- {} ---", department);
    for name in names.iter() {
        println!("{}", name);
    }
}

fn parse_command(input: &str) -> Command {
    // get command from first word of input e.g. List, Add
    let words: Vec<_> = input.split_whitespace().collect();

    match words.get(0) {
        Some(_word) => { },
        None => { return Command::None }
    }

    // macth with lower case
    match words[0].to_ascii_lowercase().as_ref() {
        "add" => {
            // get index of the 'to' word, abort if not present
            let mut to_index = words.iter().position(|&r| r == "to");
            match to_index {
                Some(idx) => {
                    Command::Add {
                        name: words[1..idx].join(" "),
                        department: words[idx+1..].join(" ")
                    }
                },
                None => { Command::None },
            }
        },
        "list" => {
            // check if we have any word after the List command
            match words.get(1) {
                // if yes use rest of command as input to ListDepartment command
                Some(_word) => { Command::ListDepartment(words[1..].join(" ")) },
                None => { Command::List },
            }
        },
        "help" => { Command::Help },
        "exit" | "quit" => { Command::Exit },
        _ => { Command::None },
    }
}

fn prompt_for_input() -> String {
    // cursor, flushed to remain on same line
    print!("> ");
    io::stdout().flush().unwrap();

    // read string from stdio and return it, panic if there is a problem
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed read input from stdin");
    input
}


fn main() {
    // welcome prompt
    println!("Directory Bot, type help for available commands");

    // init hash map directory, this is passed to each command
    let mut directory = HashMap::new();

    loop {
        let input = prompt_for_input();
        let cmd = parse_command(&input.trim());
        cmd.call(&mut directory);
    }
}
