fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("plus_one(Some(5)) = {:?}", six);
    println!("plus_one(None) = {:?}", none);

    println!("-------");

    println!("plus_one_int_return(Some(5)) = {:?}", plus_one_int_return(five));
    println!("plus_one_int_return(None) = {:?}", plus_one_int_return(None));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn plus_one_int_return(x: Option<i32>) -> i32 {
    match x {
        Some(i) => (i + 1),
        None => 0,
    }
}
