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

# How it works

The core of `obstruct` is a trait:

```rust
trait Field<T> {
    const NAME: &'static str;
    fn take(self) -> T;
}
```

Associated const `NAME` is used to perform type assertions and catch typoes.

Every use of `instruct!` or `call!` is converted into an ordered tuple of fields,
with type-level information to ensure that we can perform type-checking on
field names.

```ignore
#![feature(associated_const_equality)]
use obstruct::{call, instruct, destruct};

let rgb = instruct!{ red: 0, green: 1, blue: 2 };

// is essentially equivalent to

struct blue<T>(T);
struct green<T>(T);
struct red<T>(T);
impl<T> Field<T> for blue<T> {
   const NAME: &'static str = "blue";
   fn take(self) -> T {
     self.0
   }
}
impl<T> Field<T> green<T> {
   const NAME: &'static str = "green";
   fn take(self) -> T {
     self.0
   }
}
impl<T> Field<T> red<T> {
   const NAME: &'static str = "red";
   fn take(self) -> T {
     self.0
   }
}

let rgb = (blue(2), green(1), red(0));
```

Similarly, when you call `destruct!`, fields are, once again ordered, so

```ignore

#![feature(associated_const_equality)]
use obstruct::{call, instruct, destruct};

destruct!{let {red, green, blue} = rgb};

// is essentially equivalent to

let (blue, green, red) = rgb;
{
    fn assert_type<T, U>(_: &T) where T: Field<T, NAME="blue"> {}
    assert_type(&blue);
}
let blue = blue.0;

{
    fn assert_type<T, U>(_: &T) where T: Field<T, NAME="green"> {}
    assert_type(&green);
}
let green = green.0;

{
    fn assert_type<T, U>(_: &T) where T: Field<T, NAME="red"> {}
    assert_type(&green);
}
let red = red.0;
```

# Additional features

- [X] Destructuring support for `ref`.
- [X] Destructuring support for `mut`.
- [X] Destructuring support for `_`.
- [X] Destructuring support for `aliases`.
- [X] Destructuring support for irrefutable patterns.

# Limitations

- I haven't checked how well `call!` and `destruct!` work with methods.
- No `let else` yet.
- No pattern-matching of any kind. No idea how to implement *that*.
- For the time being, everything produced by `instruct!` is a Voldemort type. This means that we cannot write
```compile_fail
if foo {
   instruct!{ red: 0 }
} else {
   instruct!{ red: 1 }
}
```

# See also

- [structx](https://github.com/oooutlk/structx) offers similar features. I haven't checked how it works yet.
