use rust_notes::*;

mod common;

#[test]
fn exploration() {
    common::setup();

    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
#[ignore]
fn another() {
    panic!("Make this test fail");
}

#[test]
#[should_panic(expected = "Guess value must be between 1 and 100")]
fn greater_than_100() {
    Guess::new(200);
}
