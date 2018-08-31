# Learning Rust

* `fn main() { }`
* 4 spaces, no tabs for indentation
* ; at end of lines
* ! indicates a macro
* `let mut guess = String::new();` mutable variable as new empty String
* `&guess` reference to guess variable
* interpolation with {} in strings
* `extern crate rand;` use a crate
* `Err(_) => { }` catch all, match all error types
* `const MAX_POINTS: u32 = 100_000;` constant definition
* shadow a variable by defining with let again, still immutable, can also change type and re-use same name
* mutable variables cannot have their type changed

## Data Types

Every value is a certain Data Type, a **scalar** or **compound**. The compiler tries to infer types, when it can't, it will fail and ask.

### Scalar Types

* Integer (`u32`, `i32` etc.)
* Floating Point (`5.3`)
* Boolean (`bool` true/false)
* Character (`char`, `'z'` single quotes)

### Compound Types

* Tuple e.g. `let tup:(i32, f64, u8) = (500, 6.4, 1);` 
	* and `let (x, y, z) = tup;` 
	* or `let x = tup.0;`
* Array - all elements must be of the same type and fixed length, allocated on the stack (not heap).
* Vector - like an array, but can grow/shrink in size on the heap.

## Functions

* Function signatures must declare the type of each parameter.
* Statements do not return a value, expressions do. Calling a function is an expression.
* Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
* An if statement is an expression.

E.g. taking a value, and returning it:

	fn takes_and_gives_back(a_string: String) -> String {
      a_string  // no semi-colon here so a_string is returned
	}

## Result and .expect

`io::stdin().read_line` returns a Result (an `io::Result`). 

Rust has a number of types named `Result` in the standard library: a generic Result and specific versions for submodules, such as `io::Result`.

Results are enums with `Ok` and `Err` variants. You can call `.expect` on Result types. Causes the program to crash (panic) and display a message e.g.

	io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

## Ownership

Memory is managed through a system of ownership with a set of rules the compiler checks.

1. Each value in Rust has a variable that’s called its owner.
2. Can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

**Stack** - e.g. dishes, push and pop, fast, no memory lookup or pointer following, fixed and known size and location.

**Heap** - unknown size, allocating a value to the heap returns a pointer, stored on the stack and must be followed to retrieve the value.

Mutable string from a string literal:

	let mut s = String::from("hello");

String `s` can be mutated (exists on the Heap), literal `"Hello"` cannot (it exists on the Stack).

With Rust, memory is automatically returned once the variable that owns it goes out of scope (from the heap). Rust automatically calls a special `drop` function to do this.

	let s1 = String::from("hello");
	let s2 = s1;
	println!("{}, world!", s1);

The above will raise an error when referencing s1 in `println`, *why*? Rust considers s1 no longer valid when two pointers are pointing to the same location. Rust will do a shallow copy and move of the String data on the stack (the String data on the heap is not copied).

See [Figure 4-4](https://doc.rust-lang.org/book/second-edition/img/trpl04-04.svg): Representation in memory after s1 has been invalidated.

Rust will never automatically create 'deep' copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

If you do want to deep copy, use `.clone()` (may be expensive) e.g.

	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1 = {}, s2 = {}", s1, s2);

Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. 

These types have a special annotation called the `Copy` trait, e.g. integers, boolean, floating points, chars, tuples (only if all types have the `Copy` trait).

## Ownership, Functions & Return Values

Passing a variable to a function will move or copy, just as assignment does. Returning values can also transfer ownership.

	// take a String and return one
	fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
			a_string  // a_string is returned and moves out to the calling function
	}

Or in this tuple return:

	// somewhere in fn main()
	let ss1 = String::from("hello");
	let (ss2, len) = calculate_length(ss1);
	
	fn calculate_length(s: String) -> (String, usize) {
			let length = s.len(); // len() returns the length of a String

	    (s, length)
	}

The issue with the tuple code is that we have to return the String to the calling function so we can still use the String after the call to `calculate_length`, because the String was moved into `calculate_length`.

It can be a pain to always have the ceremony of a function taking a value and passing it back (passing ownership, shallow copy and move). What if we want to let a function use a value but not take ownership? Use references.

## References

References let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. Like variables, references are immutable by default.

E.g. instead of the tuple, pass by reference like so:

	fn main() {
		let s1 = String::from("hello");
		let len = calculate_length(&s1);

		println!("The length of '{}' is {}.", s1, len);
	}
	
	fn calculate_length(s: &String) -> usize { // s is a reference to a String
			s.len()
	} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
	
	// &s1, create a reference that refers to the value of s1 but doesn't own it

We call having references as function parameters _borrowing_. You cannot modify something you are borrowing, references are immutable. It is not possible to modify `s` in the `calculate_length` fn above.

**UNLESS**, we define it as a mutable reference, like so:

	fn main() {
			let mut s = String::from("hello");

	    change(&mut s);
	}
	
	fn change(some_string: &mut String) {
			some_string.push_str(", world");
	}

One *big restriction*: you can have only one mutable reference to a particular piece of data in a particular scope - to prevent race conditions. Wrap multiple mutable references to the same data in { } to isolate scopes. Also:

* Cannot have 1 mutable and 1 immutable reference to same data (same reason as above).
* Multiple immutable references are OK (since they are just reading data).

### Slices

* Another data type that does not have ownership (like references).
* Slices let you *reference* a contiguous sequence of elements in a collection, rather than the whole collection.

For example string slices `&str[]`, as a reference to a portion of a String:

	let s = String::from("hello world");
	let hello = &s[0..5];
	let world = &s[6..11];

* The slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index.
* `..` is the Rust range syntax
* `3..` drop last index, to go to end of range, start index to go from start, `..` is valid as the entire range
* String literals `let s = "Some String"` are string slices, i.e. `&str` and hence are immutable.

Array slices also exist, e.g.

	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];
	// this slice has the type &[i32]

## Structs

Name and package together multiple related values into a meaningful group. Similar to tuples, but each data value (with a data type) is named.

		// struct definition, with 4 fields
		struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
		}

		// struct instance
		let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
		};
		// use comma after last name, fields can be out of order

If the instance is mutable `let mut user1 = ...` then we can change a field value like so:

		user1.email = String::from("anotheremail@example.com");

Use Struct Update Syntax `..` to use set of values from one struct in another e.g.

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    // where user1 is a User instance with default vals for active and sign_in_count

Tuple Structs, like structs with a named type but without named fields e.g.

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

It's possible for struct fields to be references (or string/array slices) but doing this requires the use of Rust lifetimes (that ensure data referenced by the struct is valid for as long as the struct is; see later notes).

### Deriving Traits

Rust has provided a number of traits for us to use with the derive annotation that can add useful behaviour to our custom types. E.g. The Debug trait to allow us to inspect structs with `println` like so:

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle { width: 40, height: 60 }
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // or this for multi-line inspect

### Methods on Struct types with `impl`

NOTE: you can have multiple `impl` blocks for the same struct.
E.g. adding the area method to the Rectangle struct

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    // call method with
    let rect1 = Rectangle { width: 30, height: 50 };
    let area = rect1.area();

### Associated struct functions

Functions (not methods) associated with the struct but don’t have an instance of the struct to work with (think; like class methods in Ruby). These are often used for constructors that will return a new instance of the struct, called with the `::` syntax e.g.

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }

    // call this with
    let sq = Rectangle::square(3);


## ENUMs

Allow you to define a type by enumerating its possible values.

* so you can encode meaning along with data
* `Option` enum, a value can be either something or nothing
* match expression (pattern matching) on Enums

```
enum IpAddrKind {
  V4,
  V6,
}
let four = IpAddrKind::V4;		

// and
fn route(ip_type: IpAddrKind) { }
route(IpAddrKind::V4);
	
// and
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
	
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}

// without structs, using just enums
enum IpAddr {
		  V4(u8, u8, u8, u8),
		  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
```

* attach data to each variant of the enum directly, no need for an extra struct.
* put any kind of data inside an enum variant: strings, numeric, or structs.
* you can define methods on enums, like you do with structs:

```
enum Message {
		Quit,
    Move { x: i32, y: i32 }, // an anonymous struct
		Write(String),
		ChangeColor(i32, i32, i32),
}

impl Message {
		fn call(&self) {
        // method body would be defined here
		}
}

let m = Message::Write(String::from("hello"));
m.call();
```

* `Option` is another enum defined by the standard library, included in the prelude.
* It encodes the common scenario in which a value could be something or it could be nothing, in order for the compiler to detect and handle cases where a null value could cause an error.

```
enum Option<T> {
    Some(T),
    None,
}
// enum representing concept of a value being present or absent
```

`<T>` syntax is a a generic type parameter, meaning the Some variant of the Option enum can hold one piece of data of *any type*.

    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

If we use None rather than Some, we need to tell Rust what type of Option<T> we have (hence the <i32>). And you have to convert an Option<T> to a T before you can perform T operations with it. How do you do that? See [these methods](https://doc.rust-lang.org/std/option/enum.Option.html) on `Option<T>`.

You'll want some code to run if the Option<T> is a Some value, and code to run when it is a None value. You can do this checking with enum matching.

### Enum Matching

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter.
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            _ => {
              println("catch all!");
            },
        }
    }
    // then call
    let val:u32 = value_in_cents(Coin::Dime);

Matching Some, None with Option<T> and enum matching:

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);  // Some(6)
let none = plus_one(None); // None
```

The match expression must handle all variants of the enum. The `_` placeholder is a catch all, and matches all other variants. If we only care about 1 variant, instead of match, you can use the `if let` control flow:

These are equivalent:

```
let some_u8_value = Some(0u8);

// do nothing for any Some value, other than 3
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

if let Some(3) = some_u8_value {
    println!("three");
}
```

I.e. like a match that runs code when the value matches one pattern and then ignores all other values.

## Modules (and mod)

You can extract functions (and other code, like structs and enums) into different modules. A module is a namespace that contains definitions of functions or types, and you can choose whether those definitions are visible outside their module (public) or not (private).

* `mod` keyword declares a new module.
* `pub` makes an item public (it's private by default).
* `use` keyword brings modules, or the definitions inside modules, into scope.

Use `mod.rs` directory in module folders as base e.g.

```
lib.rs // (contains `mod foo;`)
└── foo
  ├── bar.rs (contains the declarations in `foo::bar`)
  └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

Or in lib.rs arrange like this in a single file:

```
mod foo {
  fn connect() {
  }

  mod bar {
    fn do_something() {
      ::foo::connect();
    }
  }
}
// call foo::connect();
// or foo::bar::do_something();
```

To make a module's functions public, use `pub` e.g.

    pub mod client { }
    pub fn connect() { }

Public functions in libraries wont have warnings about not being used.

* If an item is public, it can be accessed through any of its parent modules.
* If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.
* Precede with `::` to walk up to root function.

### The `use` keyword

Bring module names into scope with `use` e.g.

```
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// call with a::series::of:nested_modules();
// or with `use` e.g.

use a::series::of;

fn main() {
    of::nested_modules();
}

// you can even bring just the function into scope with use a::series:of::nested_modules;
// and then call nested_modules();
// use * to bring all items in a namespace into scope
use a::*
// use `super` to move up one module in the hierarchy from our current
// module, e.g. common use case in tests:

pub mod client;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

## Common Collections

* contain multiple values
* stored on the heap, can grow/shrink in size
* e.g. vector, string, hash map

### Vector

* store more than one value in a single data structure in sequence in memory
* can only store values of the same type
* create with; `let v: Vec<i32> = Vec::new();`
* `Vec<T>` (from std lib) can hold a value of any type, but we must pass which type when creating a new vector.
* OR this macro: `let v = vec![1, 2, 3]`;    // type is inferred
* add to vector with push; e.g v.push(8); // pop the last item with v.pop();
* vectors are indexed by 0

Get values at index with:

	let v = vec![1, 2, 3, 4, 5]; // vec! macro
	let third: &i32 = &v[2];
	let third: Option<&i32> = v.get(2);
	// attempting to get unknown index raises a panic error
	// can't hold an index reference to a mutable vector, and then change that vector, since changing it might shift it in memory

Iterate over values with:

	let mut v = vec![100, 32, 57];
	
	for i in &mut v {
	    println!("{}", i);
	}
	
	// or change each value with
	for i in &mut v {
	    *i += 50;
	}
	// we have to use the dereference operator (*) to get to the value in i (before we can use the += operator)

To get around having to store same type, use an enum (with multiple types).

### String

* `str` is a string type encoded in the core language, `String` is in the std lib.
* a growable, mutable, owned, UTF-8 encoded string type.
* `&str` is a string slice
* create a new empty string with:

```
let mut s = String::new();
// or
let s = "initial contents".to_string();  // using to_string on a literal
// or
let s = String::from("initial contents");
// add to a string (pushing a string slice)
let s2 = "bar";
s.push_str(s2);
// or push a single char with push (and single quotes)
s.push('p');
// adding strings together
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// s1 is dropped since + takes ownership
let s = s1 + "-" + &s2 + "-" + &s3;
// or use this
let s = format!("{}-{}-{}", s1, s2, s3);
// this does not take ownership (and works like println)
```

You can't index into a String because UTF-8 and byte storage in memory is different with non-character unicode scalar values.

You can slice on character boundaries, but you have to know where they are.. be careful, Rust will panic if you string slice on a non-char boundary.

Best way to do this is to use the chars method, and iterate over chars yourself e.g.

```
for c in "नमस्ते".chars() {
    println!("{}", c);
}

// gives
न
म
स
 ्
त
 े
```

Or use bytes to get the raw bytes, but remember that valid Unicode scalar values may be made up of more than 1 byte.


### Hash Maps

* `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.
* Via a hashing function which decides how to place these in memory (heap).
* use `insert()` to add key/values, e.g.

```
// we must pull feature from the collections lib
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

```
// or create from vectors zipped together and collected
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
// underscores means Rust can infer the type for keys and values here

// get a value from the HashMap with
let team_name = String::from("Blue");
let score = scores.get(&team_name);
// result for score will be `Some(&10)`, or None if key does not exist since `get` returns an `Option<&V>` you'll need to handle both cases

// iterate with:
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// overwrite
scores.insert(String::from("Blue"), 100);

// only insert if key doesn't already exist
scores.entry(String::from("Blue")).or_insert(50);
// use `entry` and `or_insert`, useful in something like this;

let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
// or_insert method returns a mutable reference (`&mut V`) so we have to de-reference to change it.
```

* All keys must have same type, and all values too.
* Vars with `Copy` trait, are copied to hash (like `i32`), otherwise Hash will take ownership of the var.
* By default uses a cryptographically secure hashing function (helps prevent against DDoS attacks), you can specify a different `hasher` (having a `BuildHasher` trait); or grab another hasher from an existing crate.

See code challenges for more examples.


## Error Handling

Two categories, recoverable and unrecoverable. In general its good practice to return `Result` type, and only panic, unwrap, expect etc. when prototyping code quickly e.g. in example code or tests.

Use `panic` for invalid or contradictory values, or missing values are passed to your code, e.g. someone calling into your lib passes values that don’t make sense i.e. inputs to function are out of bounds or nonsense - these constraints should be explained in your docs.

NOTE: for validation of input, you might want to create your own struct with new method that panic's when initialised with invalid data (see guessing game example).

### Unrecoverable

For example from a bug, memory access violation - the `panic!` macro stops execution. With panic!, your program will print a failure message, unwind and clean up the stack, and then quit.

Unwinding cleans up memory by walking back up through each function in the stack trace. To exit immediately use `abort` (OS must clean up memory), to do this update `Cargo.toml` with:

	[profile.release]
	panic = 'abort'

This is usually fine and more performant to do (says John Carmack).

Run code with `RUST_BACKTRACE=1` for a panic backtrace to see where a deeper panic originated from your code.

### Recoverable

Show error to user and retry or continue `Result<T, E>`. Handle the Result struct returned from calling potentially error throwing methods.

	enum Result<T, E> {
			Ok(T),
		  Err(E),
	}

* `T` and `E` are generic type parameter (see next section on Generics).
* `T` type from an `OK` result, `E` type of error from an `Err` result.

Various ways to deal with `Result`

* Use `unwrap` or `expect` (`unwrap` will return value inside OK or panic, `expect` does the same with a custom error message)
* Use match and handle each enum variant e.g.

```
let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    },
};

// to handle different kinds of errors use `error.kind() == ErrorKind::NotFound` in match arm (known as a match guard)

let f = match f {
    Ok(file) => file,
    // ref is needed here so error is not 'moved' into guard condition
    Err(ref error) if error.kind() == ErrorKind::NotFound => {
        // create the file if it doesn't exist
        match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!(
                    "Tried to create file but there was a problem: {:?}",
                    e
                )
            },
        }
    },
    Err(error) => {
        panic!(
            "There was a problem opening the file: {:?}",
            error
        )
    },
};
```

**Note**: `&` matches a reference and gives you its value, but `ref` matches a value and gives you a reference.

### Propagating Errors

There are a couple of ways to do this:

* Have your method return a `Result<String, io::Error>` with a match like so:

```
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // return early from method with the error
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

* Use the `?` to propagate; ? placed after a `Result` means value inside `Ok` for Ok results will be returned OR `Err(e)` returns from the enclosing function immediately, meaning the above could be chained up and re-written as:

```
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // the ?'s ensure errors are returned early
    File::open("hello.txt")?.read_to_string(&mut s)?;
    // return the Ok result, no chance of an error now
    Ok(s)
}

// caveat, you can only use `?` in functions that return a `Result` otherwise use `match` or `unwrap`, `expect` etc.
```

## Generics, Traits and Lifetimes

### Generics

* We've see these already e.g. `Hash<K, V>`, `Result<T, E>`, `Option<T>`, `Vec<T>`.
* They define a 'generic' type instead of a concrete type.
* Like how function args have unknown parameters at compile time, and concrete values at runtime.
* To make a method param take (and return) generics instead of concrete types:

```
// taking char slice, returning char
fn largest_char(list: &[char]) -> char { .. }

// becomes this (taking slices of generic T, returning T generic type)
fn largest<T>(list: &[T]) -> T { .. }

// this could work if the things you do in the function are OK to perform on the generic type, if they are not, Rust will error and explain why.		
```
		
Structs can take generic type params too e.g.

    struct Point<T> {
        x: T,
        y: T,
    }

    fn main() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }

    // however below will fail since generic types must be the same T
    let wont_work = Point { x: 5, y: 4.0 };

    // to have different generic types, use T, U like so:
    struct Point<T, U> {
        x: T,
        y: U,
    }

Enums with generic types e.g. `Option<T>` and `Result<T, E>`

		enum Option<T> {
		    Some(T),       // presence of some value of any type
        None,          // no value present
    }

    enum Result<T, E> {
        Ok(T),        // Ok result with any type of value
        Err(E),       // Err result with an Error type value
    }

In method definitions for structs & enums (methods, not functions since they take `&self`):

    struct Point<T> {
        x: T,
        y: T,
    }

    // getter for x returns whatever the generic T type of x is
    // note the impl<T> here, meaning function available to any Point<T>
    impl<T> Point<T> {
        fn x(&self) -> &T {
		        &self.x
        }
    }
    
    // note no impl<f32> here, since function only available to Point<f32>'s
    impl Point<f32> {
			  fn distance_from_origin(&self) -> f32 {
				    (self.x.powi(2) + self.y.powi(2)).sqrt()
	      }
    }

Use of generics has no performance implications, Rust compiler will always expand out the generic types to concrete ones at compile time.

### Traits

A trait tells the Rust compiler about functionality a particular type has and can share with other types.

* use traits to define shared behaviour in an abstract way.
* use trait bounds to specify that a generic can be any type that has certain behaviour.
* like interfaces in other languages.
* trait definitions group method signatures together to define a set of
  behaviours.

E.g. a summary trait definition letting us call summarise on various types (e.g. `NewsArticle`, `BlogPost`, `Tweet` etc.

    // public Summary trait, having a summarize method signature
    pub trait Summary {
        fn summarise(&self) -> String;
        fn to_xml(&self) -> String;
    }
    
    // each type implementing this trait must implement the method signatures the compiler will enforce this

E.g. implementing this on a `NewsArticle` with `impl Summary for`

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn to_xml(&self) -> String {
          format!("<news>{}</news>", self.content);
        }
    }

    // we can now call this like so;
    let news_article = NewsArticle {
      headline: String::from("hello"),
      ...
    }
    println!("{}", news_article.summarize());

OR instead of just a trait method signature, you can have a default one like so:

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    // and then just this to implement it;
    impl Summary for NewsArticle {}

#### Trait Bounds

We can use _trait bounds_ to constrain generic types to ensure the type will be limited to those that implement a particular trait and behaviour. The compiler checks that all the concrete types used with our code provide the correct behaviour.

* trait bounds ensure that a type has the behaviour we want.
* trait bounds exist for generic type parameters e.g.

```
// here is a trait bound, the item must implement the Summary trait
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarise());
}
```

You specify multiple trait bounds on a generic type using the + syntax e.g.

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

// can be hard to read, so we can also use a where clause syntax like this:

fn some_function<T, U>(t: T, u: U) -> i32
  where T: Display + Clone,
        U: Clone + Debug
{
```

We can use trait bounds to implement methods conditionally for types that implement the specified traits e.g.

```
struct Point<T> {
    x: T,
    y: T,
}

// only have some_func implemented on Point's where the generic type passed has the Clone and Display traits
impl<T: Clone + Display> Point<T> {
    fn some_func(&self) {
        println!("some func");
    }
}
```

**NOTE**: `PartialOrd` is used for comparison `Display` trait for printing

The std lib has some blanket trait implementations, e.g. the `ToString` trait is applied to anything having the `Display` trait, so we can call `.to_string()` on a variety of types.`

### Lifetimes

* Distinct to Rust, a unique concept.
* Ensure that references are valid as long as we need them.
* Every reference in rust has a lifetime, the scope for which a ref is valid.
* Lifetimes are usually implicit and inferred.
* In some cases we must annotate lifetimes of refs with generic lifetime parameters.
* A borrow checker compares scopes to determine whether all borrows are valid.
* Lifetime annotations don't change how long ref's live for, they just provide meaning to the borrow checker so it can make decisions.
* The names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types e.g. `&'a i32`, `&'a mut i32`
* One lifetime annotation doesn't make much sense since they are used to show how references relate to one another.

For example, the following won't compile (when passing in 2 string slices):

		fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

_Why?_ To return the reference, Rust can't know ahead of time, the lifetime scope of x or y. The borrow checker doesn't know which scopes to compare. The compiler will suggest it needs a lifetime parameter here like this:

    // all the references in the signature must have the same lifetime 'a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

When we pass concrete refs to the function, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. And the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

**TLDR**: Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

E.g. this will fail, since result is a dangling reference and has a lifetime that hasn't been connected to any parameter lifetimes.

    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules (exclusion and inferred rules); to save you writing `'` everywhere as boiler plate repetitive code.

* Parameter lifetimes are 'input' lifetimes.
* Return lifetimes are 'output' lifetimes.
* Syntax for structs is like so:

```
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let i = ImportantExcerpt { part: novel.as_str() };
    println!("excerpt is {:?}", i);
}

// so an instance of ImportantExcerpt can’t outlive the reference it holds in its part field
```

* Lifetime Annotations in Struct `impl` Method Definitions - because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for e.g.

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

Special `'static` lifetime, e.g. applied to string literals by default.

    let s: &'static str = "I have a static lifetime.";

Text of this string is stored directly in the binary of your program, which is always available (never dropped).

Try to fix lifetime problems without using static first, do you really want var to live for the entire lifetime of the program?

Example using traits, lifetimes and generic types altogether:

    use std::fmt::Display;

    // pass ann with type T that has the Display trait, so we can println it
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }


## Tests

* Run tests with `cargo test`.
* A test in Rust is a function that’s annotated with the `#[test]` attribute.
* Attributes are metadata about pieces of Rust code e.g. `#[derive(Debug)]` -
  `derive` is the attribute.
* To change a function to be a test, add `#[test]` before it.
* Rust can also compile any code examples that appear in our API documentation.
* `assert!` marco, Boolean `true` test passes, `false` test fails.
* Add another string arg to show custom test error message.

```
#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle { length: 8, width: 7 };
    let smaller = Rectangle { length: 5, width: 1 };

    assert!(!smaller.can_hold(&larger));
}
```

* other macros, `assert_eq!(x, y)` and `assert_ne!(x, y)`
* for these values being compared must implement the `PartialEq` and `Debug` traits.
* add `#[derive(PartialEq, Debug)]` to your struct or enum for defaults.
* `#[ignore]` and `#[should_panic(expected = "")]` attributes are available.

By default `cargo test` run all the tests in parallel and capture output generated during test runs. See `cargo test --help` for more options E.g. to run tests consecutively (no threading) use:

    cargo test -- --test-threads=1

Use this to avoid capturing STDIO, and instead show `println`'s:

    cargo test -- --nocapture

Pass the name of a test(s) in command to run individual tests

    cargo test {keyword or test function name}

Run only ignored tests with: 

    cargo test -- --ignored

Run a particular test file in the `tests` dir:

    cargo test --test integration_test

### Unit Tests

Small and more focused, testing one module in isolation at a time, and can test private interfaces. I.e. test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.

**Convention**: create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`. The `cfg` attribute means configuration, tests will only run when calling `cargo test` (not build).

Because unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

### Integration Tests

Entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

**Convention**: Create a `tests` directory at the top level of our project directory, next to `src`. Cargo will compile each of the test files in this directory as an individual crate.

Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output; so to have common helper functions for setup, place them in `tests/common/mod.rs` for example.

**NOTE**: integration tests for binary crates; add a `lib.rs` and `main.rs` to src, and have main call to use the lib AND have `tests` directory use/test the lib.

## I/O Command line project

Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.

E.g. iterator for command args from the environment

    use std::env;
    let args: Vec<String> = env::args().collect();
    // first arg is always name of the binary file

Cloning sometimes makes sense when giving up a little performance to gain simplicity is a worthwhile trade-off.

Handle a Result (OK or Err) with `unwrap_or_else` like so:

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // where Config::new returns a `Result<Config, &'static str>`
    // and `&'static str` is the type for String literal
    // unwrap_or_else is defined on Result<T, E> by std lib
    // the arg here is a closure, an anonymous function handling the
    // unwrapped err

Grab env vars with:

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    // without is_err this will return an Err variant if var isnt set
    // is_err() on a Result here, will return true if it is not set
    // so in that case our search will be case_sensitive

Use `eprintln!("")` to write text to stderr.


## Cargo

* [Documentation](https://doc.rust-lang.org/cargo/)
* [Crates.io](https://crates.io)
* `cargo new hello_cargo --bin` (a binary project)
* `cargo build`
* `cargo build --release`
* `cargo check`