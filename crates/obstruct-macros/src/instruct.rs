//! Implementation of macros to create an anonymous struct

use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse::Parse, Token, Ident, Expr, braced};

/// A binding `foo: expr`.
#[derive(Debug)]
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

fn check_for_duplicates(bindings: &[BasicBind]) -> Result<(), syn::Error> {
    // Check for duplicates.
    let mut prev: Option<&BasicBind> = None;
    for binding in bindings {
        if let Some(prev) = prev {
            if prev.ident == binding.ident {
                return Err(syn::Error::new(
                    binding.ident.span(),
                    format!("Duplicate field {}", prev.ident)))
            }
        }
        prev = Some(binding);
    }
    Ok(())
}

/// The contents of an anonymous struct creation, e.g. `instruct!{ x: 7, 8: 9 }`.
pub struct InstructStruct(Vec<BasicBind>);
impl Parse for InstructStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let bindings: Vec<BasicBind> = input.parse_terminated(<BasicBind as Parse>::parse, Token![,])?
            .into_iter()
            // Normalize order.
            // Any order will do.
            .sorted_by_cached_key(|assign| assign.ident.to_string())
            .collect();
        check_for_duplicates(&bindings)?;

        Ok(Self(bindings))
    }
}

impl InstructStruct {
    /// Emit the code to represent this anonymous struct.
    ///
    /// We rewrite `let foo = instruct!{ x: 7, y: 9 };` into
    ///
    /// ```ignore
    /// let foo = {
    ///    use obstruct::Field;
    ///
    ///    // One struct per field.
    ///    // As we don't have type information, these structs are generic.
    ///    struct x<T>(T);
    ///    struct y<T>(T);
    ///
    ///    // Each struct implements `Field` for its field name.
    ///    impl<T> Field<T> for x<T> {
    ///       const NAME: &'static str = "x";
    ///       fn take(self) -> T {
    ///         self.0
    ///       }
    ///    }
    ///    impl<T> Field<T> for y<T> {
    ///       const NAME: &'static str = "y";
    ///       fn take(self) -> T {
    ///         self.0
    ///       }
    ///    }
    ///
    ///    (x(7), y(9))
    /// };
    /// ```
    pub fn transform(self) -> TokenStream {
        // Parse input.
        let fields = self.0.into_iter()
            .map(|BasicBind { ident, expr }| {
                quote!{
                    {
                        use obstruct::Field;

                        // Define a local structure representing the field.
                        #[allow(non_camel_case_types)]
                        #[derive(Clone)]
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
}



/// The contents of an anonymous struct function call, e.g. `call!{ foo, {x: 7, 8: 9} }` or `call!{ foo.bar, { x: 7, y: 9} }` .
///
/// Missing features:
/// - passing regular arguments
pub struct InstructFunctionCall {
    /// The callee (e.g. `foo` or `foo.bar`).
    callee: Expr,

    /// Named arguments
    args: InstructStruct,
}
impl Parse for InstructFunctionCall {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Callee
        let callee: Expr = input.parse()?;
        let _ = input.parse::<Token![,]>()?;

        // Arguments
        let braces_content;
        let _braces = braced!(braces_content in input);

        let bindings: Vec<BasicBind> = braces_content.parse_terminated(<BasicBind as Parse>::parse, Token![,])?
            .into_iter()
            // Normalize order.
            // Any order will do.
            .sorted_by_cached_key(|assign| assign.ident.to_string())
            .collect();

        // Check for duplicates.
        check_for_duplicates(&bindings)?;

        Ok(InstructFunctionCall { callee, args: InstructStruct(bindings) })
    }
}
impl InstructFunctionCall {
    /// Emit the code to represent this anonymous struct.
    ///
    /// We rewrite `let foo = call!{bar,  {x: 7, y: 9 }};` into
    ///
    /// ```ignore
    /// let foo = bar(
    ///     {
    ///       // One struct per arg.
    ///       // As we don't have type information, these structs are generic.
    ///       struct x<T>(T);
    ///
    ///       // Each struct implements `Field` for its field name.
    ///       impl<T> Field<T> for x<T> {
    ///          const NAME: &'static str = "x";
    ///          fn take(self) -> T {
    ///            self.0
    ///          }
    ///       }
    ///
    ///       x(7)
    ///     },
    ///     {
    ///       struct y<T>(T);
    ///       impl<T> Field<T> for y<T> {
    ///          const NAME: &'static str = "9";
    ///          fn take(self) -> T {
    ///            self.0
    ///          }
    ///       }
    ///
    ///       y(9)
    ///     }
    /// );
    /// ```
    pub fn transform(self) -> TokenStream {
        let fields = self.args.0.into_iter()
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

        let mut fields_tokens = proc_macro2::TokenStream::new();
        fields_tokens.append_separated(fields, quote!{,});

        let callee = self.callee;
        let result = quote!{
            #callee ((#fields_tokens))
        };
        result.into()
    }
}
