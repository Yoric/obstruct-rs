# About

`obstruct` is an experimental implementation of anonymous structs and named arguments for Rust.

## Anonymous structs

Create an anonymous struct with `instruct!` and destructure it with `destruct!`:

```rust
#![feature(associated_const_equality)]
use obstruct::{instruct, destruct};

// Create an anonymous struct.
let structured = instruct! { red: 0, green: 1.0, blue: 2 };

// Destructure that struct.
destruct! { let {red, green, blue} = structured };
assert_eq!(red, 0);
assert_eq!(green, 1.0);
assert_eq!(blue, 2);
```

Note that this is not (just) a tuple: the order in which fields are specified does not matter!


```rust
#![feature(associated_const_equality)]
use obstruct::{instruct, destruct};

// Create an anonymous struct.
let structured = instruct! { red: 0, green: 1.0, blue: 2 };

// Destructure that struct.
destruct! { let {blue, green, red} = structured };
assert_eq!(red, 0);
assert_eq!(green, 1.0);
assert_eq!(blue, 2);
```

If you attempt to access a field that doesn't exist, you will get a compile-time error:


```compile_fail
#![feature(associated_const_equality)]
use obstruct::{instruct, destruct};

// Create an anonymous struct.
let structured = instruct! { red: 0, green: 1.0, blue: 2 };

// Destructure that struct.
destruct! { let {blue, green, oops} = structured };
//                            ^^^ --- will fail with a complex error message pointing at `oops`.
```

## Named arguments

Create a function accepting named parameters with `destruct!` and call it with `call!`:

```rust
#![feature(associated_const_equality)]
use obstruct::{call, instruct, destruct};

// Create a function accepting anonymous arguments.
destruct!(fn do_something({red: u8, green: &'static str, blue: ()}) {
    println!("Roses are {red}");
});


// Call this function
call!(do_something, {red: 0, green: "GREEN", blue: ()});

// Or equivalently
do_something(instruct! {red: 0, green: "GREEN", blue: ()});

```

Again, the order in which arguments are specified does not matter:

```rust
#![feature(associated_const_equality)]
use obstruct::{call, instruct, destruct};

// Create a function accepting anonymous arguments.
destruct!(fn do_something({red: u8, green: &'static str, blue: ()}) {
    println!("Roses are {red}");
});

do_something(instruct! {blue: (), green: "GREEN", red: 0});
```

Again, errors are caught at compile-time:

```compile_fail
#![feature(associated_const_equality)]
use obstruct::{call, instruct, destruct};

// Create a function accepting anonymous arguments.
destruct!(fn do_something({red: u8, green: &'static str, blue: ()}) {
    println!("Roses are {red}");
});

do_something(instruct! {blue: (), green: "GREEN", oops: 0});
//                                                 ^^^ --- will fail with a complex error message pointing at `oops`.


call!(do_something, {red: 0, green: "GREEN", oops: ()});
//                                           ^^^ --- will fail with a complex error message pointing at `oops`.
```
