extern crate proc_macro;

use itertools::Itertools;
use proc_macro::{TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, parse::Parse, Token, punctuated::Punctuated, token::Comma, Ident, Expr, braced};

struct BasicBind {
    ident: Ident,
    expr: Expr
}
impl Parse for BasicBind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(BasicBind { ident, expr })
    }
}

struct Structuration(Punctuated<BasicBind, Comma>);
impl Parse for Structuration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse_terminated(<BasicBind as Parse>::parse, Token![,]).map(Structuration)
    }
}

/// Expand an expression `instruct!{ x: 7, y: 9 }`
/// 
/// This expression is meant to be analogous to `FooBar { x: 7, y: 9 }`, except with an anonymous `struct`
/// instead of `FooBar`
#[proc_macro]
pub fn instruct(input: TokenStream) -> TokenStream {
    // Parse input into a list of bindings.
    let bindings: Vec<BasicBind> = parse_macro_input!(input as Structuration)
        .0
        .into_iter()
        // Normalize order.
        // Any order will do.
        .sorted_by_cached_key(|assign| assign.ident.to_string())
        .collect();
    // Reject if there are duplicate assignments.
    let mut prev: Option<&BasicBind> = None;
    for binding in &bindings {
        if let Some(prev) = prev {
            if prev.ident == binding.ident {
                panic!("Duplicate field {}", prev.ident);
            }
        }
        prev = Some(binding);
    }

    // Now that we are satisfied that everything is normalized, we can generate the code.
    let fields = bindings.into_iter()
        .map(|BasicBind { ident, expr }| {
            quote!{
                {
                    use obstruct::Field;

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
    let destructuration = parse_macro_input!(input as Destructuration);
    // FIXME: Sort!
    let fields: Vec<_> = destructuration.fields.into_iter()
        .sorted_by_cached_key(|field| field.to_string())
        .collect();
    // Reject if there are duplicate assignments.
    let mut prev: Option<String> = None;
    for ident in &fields {
        let ident = ident.to_string();
        if let Some(prev) = prev {
            if prev == ident {
                panic!("Duplicate field {}", prev);
            }
        }
        prev = Some(ident);
    }
    let expr = destructuration.expr;
    quote!{
        use obstruct::Field;
        let (#(#fields),*) = #expr;
        #(
            let #fields = #fields.take();
        )*
    }.into()
}

struct Destructuration {
    fields: Punctuated<Ident, Comma>,
    expr: Expr,
}
impl Parse for Destructuration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        eprintln!("Destructuration::parse");
        input.parse::<Token![let]>()?;
        eprintln!("Destructuration::parse => let");
        let content;
        braced!(content in input);
        eprintln!("Destructuration::parse => braces");
        let fields = content.parse_terminated(Ident::parse, Token![,])?;
        eprintln!("Destructuration::parse => fields");
        input.parse::<Token![=]>()?;
        eprintln!("Destructuration::parse => equals");
        let expr = input.parse()?;
        eprintln!("Destructuration::parse => expr");
        Ok(Destructuration { fields, expr })
    }
}
