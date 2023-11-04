#![feature(associated_const_equality)]

mod should_pass;

#[test]
fn test_should_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/should_fail/*.rs");
}
