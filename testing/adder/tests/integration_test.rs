extern crate adder;

// common test helpers in here e.g. for setup
mod common;

#[test]
fn it_adds_two_intergration_test() {
    common::setup(); // currently does nothing
    assert_eq!(4, adder::add_two(2));
}
