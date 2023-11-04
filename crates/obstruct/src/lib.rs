#![feature(associated_const_equality)]

pub use obstruct_macros::{call, destruct, instruct};

/// A field in an anonymous struct.
pub trait Field<T> {
    const NAME: &'static str;
    fn take(self) -> T;
}

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
