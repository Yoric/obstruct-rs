extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod instruct;
mod destruct;

/// Expand an expression `instruct!{ x: 7, y: 9 }`
///
/// This expression is meant to be analogous to `FooBar { x: 7, y: 9 }`, except with an anonymous `struct`
/// instead of `FooBar`
#[proc_macro]
pub fn instruct(input: TokenStream) -> TokenStream {
    let contents = parse_macro_input!(input as instruct::InstructStruct);
    contents.transform()
}

/// Expand an expression `call!{ foo {x: 7, y: 9} }`
#[proc_macro]
pub fn call(input: TokenStream) -> TokenStream {
    let contents = parse_macro_input!(input as instruct::InstructFunctionCall);
    contents.transform()
}


/// Expand a pattern `destruct!{let {x, y} = foo}`
///
/// This pattern is meant to be analogous to `let FooBar {x, y} = foo`, except with an anonymous `struct`
///
/// # Missing features
///
/// - ref
/// - mut
/// - any kind of `let else`.
/// - inner patterns
/// - renamings
/// - `..`
/// - `_`
/// - default values
#[proc_macro]
pub fn destruct(input: TokenStream) -> TokenStream {
    destruct::Destruct::transform(input)
}
