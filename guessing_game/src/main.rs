extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: u32,
}

impl Guess {
    // initialiser with check on input param
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        // create and return new Guess instance
        Guess { value }
    }

    // public getter for struct value attribute
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Guess the number! (1-100)");

    // random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // trim away new line char, and parse it as u32
        // continue (restart loop)  if this cannot be parsed
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("That is not a number! ({})", err);
                continue;
            }
        };
        let guess = Guess::new(input);

        // cmp is available on u32
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
