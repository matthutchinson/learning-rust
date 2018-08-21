// normal, with seperate variables
//
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// with tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// top level area method, works on a Rectangle struct param
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// with structs, having Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// struct methods
impl Rectangle {
    // think like a class method, e.g. let square = Rectangle::square(10);
    fn square(size :u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // think like an instance method, operating on ref to &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let square = Rectangle::square(55);

    println!(
        "The area of the rectangle is {} square pixels (called function).",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    let area = rect1.area();

    println!("The area of the rectangle is {} square pixels (called method).", area);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the SQUARE is {} square pixels (called function).",
        square.area()
    );
}
