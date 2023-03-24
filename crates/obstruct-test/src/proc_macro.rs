#[test]
fn test() {
    use obstruct_macros::{instruct, destruct};

    let structured = instruct! { red: 0, green: 1.0, blue: 2 };

    destruct! { let {red, blue, green} = structured };

    assert_eq!(red, 0);
    assert_eq!(green, 1.0);
    assert_eq!(blue, 2);
}