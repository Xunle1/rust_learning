use tests::tests;

mod common;

#[test]
fn test_add_two() {
    common::setup();
    let v = tests::add_two(1);
    assert_eq!(v, 3);
}