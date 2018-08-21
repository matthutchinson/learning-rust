fn main() {
    loop {
        println!("again!");
        // continue; // use to restart the loop
        break;
    }

    let mut number = 3;
    while number > 0 {
        println!("{}!", number);

        number = number - 1; // decrement
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value at index {} is: {}", index, a[index]);

        index = index + 1;
    }

    // iterator
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // reverse iterator
    for number in (1..4).rev() {
        println!("{}!", number);
    }
/bin/bash: q: command not found
    println!("LIFTOFF!!!");
}
