pub use obstruct_macros::{call, destruct, instruct};

pub trait Field<T> {
    const NAME: &'static str;
    fn take(self) -> T;
}


#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;