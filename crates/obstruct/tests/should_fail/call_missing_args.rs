#![feature(associated_const_equality)]

fn test_missing_args() {
    use obstruct_macros::{destruct, call};
    const A: i32 = 15;
    const B: f64 = 17f64;
    const C: &str = "Out of order";


    destruct!{fn test_foo({a: i32, b: f64, c: &'static str}) -> (i32, f64, &'static str) { (a, b, c) }};

    let (a, b, c) = call! { test_foo, { b: B, a: A}}; // Missing argument.
    assert_eq!(a, A);
    assert_eq!(b, B);
    assert_eq!(c, C);
}


fn main() {}