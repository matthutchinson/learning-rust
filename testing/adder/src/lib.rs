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

// running tests:
//
// run a specific test by name: cargo test exploration
//   or with any string to match on the test fn name
// use this to avoid running in parallel: cargo test -- --test-threads=1
// use this to show stdout from println: cargo test -- --nocapture
// only run ignored tests: cargo test -- --ignored
//
// run all tests in a specific (integration test) file: cargo test --test integration_test
//   where integration_test.rs exists in test/
//   note: Files in subdirectories of the tests directory don’t get compiled as separate crates or
//   have sections in the test output.

// a note on cfg:
// The attribute cfg stands for configuration and tells Rust that the following item should only be
// included given a certain configuration option. In this case, the configuration option is test,
// which is provided by Rust for compiling and running tests. By using the cfg attribute, Cargo
// compiles our test code only if we actively run the tests with cargo test.

#[cfg(test)]
mod tests {
    #[test] // attribute indicating this is a test function
   fn exploration() {
       // values being compared must implement the PartialEq and Debug traits
       // to compare structs/enums add these traits `#[derive(PartialEq, Debug)]`
       //
       // the optional last arg in these methods is a detailed failure message passed to format!
       assert_eq!(2 + 2, 4);
       assert_ne!(2 + 10, 4);
       // assert! passes if arg is true, fails if false
       assert!(2 + 2 == 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")] // expected is optional
    fn always_fail() {
        panic!("Make this test fail");
    }

    // need to bring the code under test in the outer module into the scope of the inner module
    // this allows us to use Rectangle struct above
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
        assert!(4 == add_two(2));
    }

    #[test]
    #[ignore] // ignore this test
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Hello Carol!"));
        assert!(
            result.contains("NotCarol"),
            "Greeting did not contain NotCarol, value was `{}`", result
        );
    }
}
