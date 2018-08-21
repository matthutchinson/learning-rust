#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    #[test] // attribute indicating this is a test function
   fn exploration() {
       // values being compared must implement the PartialEq and Debug traits
       // to compare structs/enums add these traits `#[derive(PartialEq, Debug)]`
       assert_eq!(2 + 2, 4);
       assert_ne!(2 + 10, 4);
       assert!(2 + 2 == 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn always_fail() {
        panic!("Make this test fail");
    }

    // need to bring the code under test in the outer module into the scope of the inner module
    // allows us to use Rectangle struct above
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore] // ignore this test
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Hello Carol!"));
        assert!(
            result.contains("NotCarol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}
