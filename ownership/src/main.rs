fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // println!("{}", s);           // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still use x afterward
    println!("{}", x);

    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    println!("{}", s1);

    let s2 = String::from("hello give back"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2);        // s2 is moved into takes_and_gives_back, which also
                                              // moves its return value into _s3
    println!("{}", s3);

    // with function returning a tuple
    let ss1 = String::from("hello");

    let (ss2, len) = calculate_length(ss1);

    println!("The length of '{}' is {}.", ss2, len);

    println!("The length of '{}' is {} (by reference).", ss2, calculate_length_by_reference(&ss2));

    let mut ss3 = String::from("hello");
    change(&mut ss3);

    println!("{}! (changed by mutable reference)", ss3);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // Also here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens, it has the Copy trait

fn gives_ownership() -> String {             // gives_ownership will move its return value into
                                             // the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the
                                             // calling function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// passing a reference to the String, returning length
fn calculate_length_by_reference(s: &String) -> usize {
    // we dont have ownership of the String passed in here, only a reference to it
    // having references as function parameters is called borrowing
    s.len()
}

// mutable references allow the String to be changed by reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
