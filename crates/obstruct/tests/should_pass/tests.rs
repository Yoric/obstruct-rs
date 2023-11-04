// Tests for proc macros.

#[test]
fn test_instruct_destruct_good() {
    // Check that we can define basic anonymous structs
    // and destructure them in arbitrary order.
    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{instruct, destruct};

    let structured = instruct! { red: RED, green: GREEN, blue: BLUE };

    destruct! { let {red, green, blue} = structured.clone() }; // Arbitrary order.

    assert_eq!(red, RED);
    assert_eq!(green, GREEN);
    assert_eq!(blue, BLUE);

    destruct! { let {red, blue, green} = structured.clone() }; // Arbitrary order.

    assert_eq!(red, RED);
    assert_eq!(green, GREEN);
    assert_eq!(blue, BLUE);

    destruct! { let {blue, green, red} = structured.clone() }; // Arbitrary order.

    assert_eq!(red, RED);
    assert_eq!(green, GREEN);
    assert_eq!(blue, BLUE);
}

#[test]
fn test_instruct_call() {
    // Check that we can define basic functions with named arguments.
    use obstruct_macros::{destruct, call};
    const A: i32 = 15;
    const B: f64 = 17f64;
    const C: &str = "Out of order";


    destruct!{fn test_foo({a: i32, b: f64, c: &'static str}) -> (i32, f64, &'static str) { (a, b, c) }};

    let (a, b, c) = call! { test_foo, { c: C, b: B, a: A}}; // Arbitrary order
    assert_eq!(a, A);
    assert_eq!(b, B);
    assert_eq!(c, C);

    let (a, b, c) = call! { test_foo, { a: A, b: B, c: C}}; // Arbitrary order
    assert_eq!(a, A);
    assert_eq!(b, B);
    assert_eq!(c, C);

}

