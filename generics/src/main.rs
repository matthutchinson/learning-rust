use std::fmt::Display;

// has concrete list param, any slice of i32 values, returns i32
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// has concrete list param, any slice of char values, returns char
// same behaviour as above
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// uses generic T parameters and generic return type
// because we want to compare values of type T in the body, we can only use types whose values can
// be ordered (having the iter() method)
// T is the name of the generic type here
// we need to be specific and use trait bounds on the generic type so compiler works with types we
// pass in
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// struct with generic types, T type must be the same for x and y here
struct Point<T> {
    x: T,
    y: T,
}

// getter
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// this method is only available to Points with f32 values for x, y
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// method is only available to Points having a generic type that implements the Clone and Display
// traits
impl<T: Clone + Display> Point<T> {
    fn some_func(&self) {
        println!("some func");
    }
}

// above can also be written as
impl<T> Point<T>
    where T: Display + Clone {
    fn some_func_again(&self) {
        println!("some func");
    }
}

fn main() {
    // struct with generic type T and method returning a generic type T
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p = Point { x: 3.124, y: 10.0 }; // both f32
    println!("p.x = {}", p.x());
    println!("p.distance_from_origin = {}", p.distance_from_origin());

    let p = Point { x: true, y: false };
    println!("p.x = {}", p.x());

    println!("---------");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    let result = largest(&number_list);
    println!("The largest number is {} (generic)", result);

    let char_list = vec!['y', 'm', 'z', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    let result = largest(&char_list);
    println!("The largest char is {} (generic)", result);
}

