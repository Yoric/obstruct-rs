//! Tests for proc macros.

#[test]
fn test_instruct_destruct_good() {
    // Check that we can define basic anonymous structs
    // and destructure them in arbitrary order (with a `let`).
    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{destruct, instruct};

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
fn test_instruct_destruct_ref() {
    // Check that we can define basic anonymous structs
    // and destructure them in arbitrary order (with a `let ref`).
    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{destruct, instruct};

    let structured = instruct! { red: RED, green: GREEN, blue: BLUE };

    destruct! { let ref {red, green, blue} = structured }; // Arbitrary order.

    assert_eq!(red, &RED);
    assert_eq!(green, &GREEN);
    assert_eq!(blue, &BLUE);

    destruct! { let ref {red, blue, green} = structured }; // Arbitrary order.

    assert_eq!(red, &RED);
    assert_eq!(green, &GREEN);
    assert_eq!(blue, &BLUE);

    destruct! { let ref {blue, green, red} = structured }; // Arbitrary order.

    assert_eq!(red, &RED);
    assert_eq!(green, &GREEN);
    assert_eq!(blue, &BLUE);
}

#[test]
fn test_instruct_destruct_alias() {
    // Check that we can define basic anonymous structs
    // and destructure them in arbitrary order (with aliases).
    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{destruct, instruct};

    let structured = instruct! { red: RED, green: GREEN, blue: BLUE };

    destruct! { let ref {red: another_red, green, blue} = structured }; // Arbitrary order.

    assert_eq!(another_red, &RED);
    assert_eq!(green, &GREEN);
    assert_eq!(blue, &BLUE);

    destruct! { let ref {red, blue: another_blue, green} = structured }; // Arbitrary order.

    assert_eq!(red, &RED);
    assert_eq!(green, &GREEN);
    assert_eq!(another_blue, &BLUE);

    destruct! { let ref {blue, green: another_green, red} = structured }; // Arbitrary order.

    assert_eq!(red, &RED);
    assert_eq!(another_green, &GREEN);
    assert_eq!(blue, &BLUE);

    destruct! { let ref {blue, green: _, red} = structured }; // Arbitrary order.

    assert_eq!(red, &RED);
    assert_eq!(blue, &BLUE);
}

#[test]
fn test_instruct_destruct_inner_ref() {
    // Check that we can define basic anonymous structs
    // and destructure them in arbitrary order (with inner ref).
    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{destruct, instruct};

    let structured = instruct! { red2: RED, green2: GREEN, blue2: BLUE };

    destruct! { let {ref red2, ref blue2, ref green2} = structured }; // Arbitrary order.

    assert_eq!(red2, &RED);
    assert_eq!(green2, &GREEN);
    assert_eq!(blue2, &BLUE);

    destruct! { let {ref red2, green2, blue2} = structured.clone() }; // Arbitrary order.

    assert_eq!(red2, &RED);
    assert_eq!(green2, GREEN);
    assert_eq!(blue2, BLUE);

    destruct! { let {ref red2, ref blue2, ref green2} = structured }; // Arbitrary order.

    assert_eq!(red2, &RED);
    assert_eq!(green2, &GREEN);
    assert_eq!(blue2, &BLUE);
}


#[test]
fn test_instruct_destruct_inner_refmut() {
    // Check that we can define basic anonymous structs
    // and destructure them in arbitrary order (with mut).
    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{destruct, instruct};

    let structured = instruct! { red4: RED, green4: GREEN, blue4: BLUE };

    destruct! { let {ref mut red4, ref mut blue4, ref mut green4} = structured.clone() }; // Arbitrary order.

    assert_eq!(*red4, RED);
    assert_eq!(*green4, GREEN);
    assert_eq!(*blue4, BLUE);
    *red4 = RED;
    *blue4 = BLUE;
    *green4 = GREEN;

    assert_eq!(*red4, RED);
    assert_eq!(*green4, GREEN);
    assert_eq!(*blue4, BLUE);
}

#[test]
fn test_instruct_destruct_inner_basic_pat() {
    // Check that we can use trivial patterns.
    const RED: &str = "red";
    const GREEN: Option<f64> = Some(1.0);
    const BLUE: () = ();

    use obstruct_macros::{destruct, instruct};

    let structured = instruct! { red4: RED, green4: GREEN, blue4: BLUE };

    destruct! { let {red4, blue4: (), green4} = structured.clone() }; // Arbitrary order.
    assert_eq!(red4, RED);
    assert_eq!(green4, GREEN);
}

#[test]
fn test_instruct_call() {
    // Check that we can define basic functions with named arguments.
    use obstruct_macros::{call, destruct};
    const A: i32 = 15;
    const B: f64 = 17f64;
    const C: &str = "Out of order";

    destruct! {fn test_foo({a: i32, b: f64, c: &'static str}) -> (i32, f64, &'static str) { (a, b, c) }};

    let (a, b, c) = call! { test_foo, { c: C, b: B, a: A}}; // Arbitrary order
    assert_eq!(a, A);
    assert_eq!(b, B);
    assert_eq!(c, C);

    let (a, b, c) = call! { test_foo, { a: A, b: B, c: C}}; // Arbitrary order
    assert_eq!(a, A);
    assert_eq!(b, B);
    assert_eq!(c, C);
}
