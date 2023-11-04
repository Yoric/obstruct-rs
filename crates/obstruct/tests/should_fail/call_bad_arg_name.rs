#![feature(associated_const_equality)]
fn struct_bad_field_names() {
    // Check that the code won't build if the field names are incorrect

    const RED: &str = "red";
    const GREEN: f64 = 1.0;
    const BLUE: () = ();

    use obstruct_macros::{instruct, destruct};

    let structured = instruct! { red: RED, green: GREEN, blue: BLUE };

    destruct! { let {red, green, oops} = structured }; // Look, we've used oops!
    // This fails with an error message mentioning oops.
}
fn main() {}