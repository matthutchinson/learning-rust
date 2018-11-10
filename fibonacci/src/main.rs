use std::io;

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn iterative_fib(n: u32) -> u32 {
    let mut first;
    let mut second = 1;
    let mut third = 0;
    let mut index = 1;

    while index <= n {
        first = second;
        second = third;
        third = first + second;

        index = index + 1;
    }

    third
}


fn main() {
    println!("Length of fibonacci sequence please?");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // trim away new line char, and parse it as u32
    let n: u32 = input.trim().parse().expect("That is not a number!");

    println!("\nFibonacci sequence for {} is (iteratively)", n);

    // ..=n dentoes an inclusive range
    for index in 0..=n {
        println!("{}", iterative_fib(index));
    }

    println!("\nFibonacci sequence for {} is (recursive)", n);

    // ..=n dentoes an inclusive range
    for index in 0..=n {
        println!("{}", fib(index));
    }
}
