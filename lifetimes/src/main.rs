// fn main() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     // x doesn't live long enough to be used here, since r is ref x that is dropped in the scope
//     // above, x was 'deallocated' in the scope
//     // r 'lives longer' than x here
//     println!("r: {}", r);
// }
//
// no lifetime parameters set, rust can't tell how long return reference will live for, since it
// could be one of two possiblities x and y
//
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// when this happens, we need to annotate params to indicate their lifetime relationship with the
// return value (see longest method below)


use std::fmt::Display;

// with a generic lifetime parameter 'a
// all the references in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// method making use of lifetimes, generics and trait bounds (Display)
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {} - i'm calculating things...", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct with attribute holding a reference (making use of lifetime 'a)
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");  // String
    let string2 = "xyz";  // string literal is a str slice (a reference to a sequence of chars)

    // works
    let result = longest(string1.as_str(), string2); // as_str on string1 to return a str slice ref
    println!("The longest string is {}", result);

    // will work too, since string1 lives longer
    {
        let string3 = "abcdef";
        let result = longest(string1.as_str(), string3);
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let i = ImportantExcerpt { part: novel.as_str() };
    println!("excerpt is {:?}", i);

    // will not work, string1, 4 do not have same scope
    // string4 will not live long enough
    // let result;
    // {
    //     let string4 = String::from("xyz");
    //     result = longest(string1.as_str(), string4.as_str());
    // }
    // println!("The longest string is {}", result);


    let ann = String::from("hello there!");
    // ann must be any type T implementing the Display trait
    let result = longest_with_an_announcement("zxv", "pppp", ann);
    println!("The longest string (with announcement) is {}", result);
}
