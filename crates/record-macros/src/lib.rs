extern crate proc_macro;

use itertools::Itertools;
use proc_macro::{TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, parse::Parse, Token, punctuated::Punctuated, token::Comma, Ident, Expr};

struct BasicAssign {
    ident: Ident,
    expr: Expr
}
impl Parse for BasicAssign {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let expr = input.parse()?;
        Ok(BasicAssign { ident, expr })
    }
}

struct Assignments(Punctuated<BasicAssign, Comma>);
impl Parse for Assignments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse_terminated(<BasicAssign as Parse>::parse, Token![,]).map(Assignments)
    }
}

/// Expand an expression `instruct!{ x = 7, y = 9 }`
///
/// # Missing features
///
/// - `ref`
/// - `mut`
#[proc_macro]
pub fn instruct(input: TokenStream) -> TokenStream {
    // Parse input into a list of assignments.
    let assignments: Vec<BasicAssign> = parse_macro_input!(input as Assignments)
        .0
        .into_iter()
        // Normalize order.
        // Any order will do.
        .sorted_by_key(|assign| assign.ident.to_string())
        .collect();
    // Reject if there are duplicate assignments.
    let mut prev: Option<&BasicAssign> = None;
    for assign in &assignments {
        if let Some(prev) = prev {
            if prev.ident == assign.ident {
                panic!("Duplicate field {}", prev.ident);
            }
        }
        prev = Some(assign);
    }

    // Now that we are satisfied that everything is normalized, we can generate the code.
    let fields = assignments.into_iter()
        .map(|BasicAssign { ident, expr }| {
            quote!{
                {
                    use record::Field;

                    // Define a local structure representing the field.
                    #[allow(non_camel_case_types)]
                    struct #ident<T>(T);

                    // Make it an instance of `Field`.
                    impl<T> Field<T> for #ident<T> {
                        const NAME: &'static str = stringify!(#ident);
                        fn take(self) -> T {
                            self.0
                        }
                    }

                    // This is the value we're looking for.
                    #ident(#expr)
                }
            }
        });

    let mut tokens = proc_macro2::TokenStream::new();
    tokens.append_separated(fields, quote!{,});

    // Turn it into a tuple.
    quote!{
        (#tokens)
    }.into()
}

