use std::iter::zip;

use itertools::Itertools;
use proc_macro::TokenStream;
use quote::{quote, format_ident, TokenStreamExt};
use syn::{parse_macro_input, parse::Parse, Token, token::{Comma, SelfValue}, Ident, Expr, braced, parenthesized, punctuated::Punctuated, ReturnType, Block, FieldsNamed, Generics, TypeParam, parse_quote, LitStr};


struct DestructFields {
    fields: Punctuated<Ident, Comma>
}
impl Parse for DestructFields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields = input.parse_terminated(Ident::parse, Token![,])?;

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

        Ok(DestructFields { fields })
    }
}

pub enum Destruct {
    DestructExpression(DestructExpression),
    DestructFunction(DestructFunction),
}
impl Parse for Destruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![fn]) {
            DestructFunction::parse(input).map(Destruct::DestructFunction)
        } else {
            DestructExpression::parse(input).map(Destruct::DestructExpression)
        }
    }
}
impl Destruct {
    pub fn transform(input: TokenStream) -> TokenStream {
        let destructuration = parse_macro_input!(input as Self);
        match destructuration {
            Destruct::DestructExpression(e) => e.transform(),
            Destruct::DestructFunction(e) => e.transform(),
        }
    }
}

/// A destruct expression, e.g. `destruct!{let {a, b} = foo}`
pub struct DestructExpression {
    fields: DestructFields,
    expr: Expr,
}
impl Parse for DestructExpression {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![let]>()?;
        let content;
        braced!(content in input);
        let fields = content.parse()?;
        input.parse::<Token![=]>()?;
        let expr = input.parse()?;
        Ok(DestructExpression { fields, expr })
    }
}
impl DestructExpression {
    fn transform(self) -> TokenStream {
        let fields: Vec<_> = self.fields.fields.into_iter()
            .sorted_by_cached_key(|field| field.to_string())
            .collect();
        let expr = self.expr;
        let mut tokens = quote!{
            let (#(#fields),*) = #expr;
        };
        let assigns: Vec<_> = fields.into_iter().map(|field| {
            let field_name = LitStr::new(&field.to_string(), field.span());
            quote!{
                {
                    fn assert_type<T, U>(_: &T) where T: obstruct::Field::<U, NAME=#field_name> {
                        // Won't compile if we have the wrong type.
                    }
                    assert_type(&#field)
                }
                let #field = obstruct::Field::<_>::take(#field);
            }
        }).collect();
        tokens.append_all(assigns);
        eprintln!("YORIC: Generated {}", tokens);
        tokens.into()
    }
}


/// A destruct function declaration, e.g. `destruct!{fn foo([self,] { a: 8, b: 9 }}`
pub struct DestructFunction {
    /// Function identifier.
    ident: Ident,

    generics: Generics,

    /// If this is a method, `self`.
    maybe_self: Option<SelfValue>,

    /// Optional comma after `self`.
    maybe_comma: Option<Comma>,

    /// Actual arguments.
    bindings: FieldsNamed,

    return_type: ReturnType,
    body: Block,
}
impl Parse for DestructFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // fn foo([self,] AnonymousStructDeclaration)
        input.parse::<Token![fn]>()?;     // `fn`
        let ident: Ident = input.parse()?;// function name

        let generics: Generics = input.parse()?;

        let paren_content;
        let _paren = parenthesized!(paren_content in input);

        // Optional `self`.
        let maybe_self = paren_content.parse::<Option<Token![self]>>()?;
        let maybe_comma = paren_content.parse::<Option<Token![,]>>()?;

        // Named arguments.
        let bindings = paren_content.parse::<FieldsNamed>()?;

        // Out of parens.
        // Optional result type.
        let return_type = input.parse::<ReturnType>()?;

        // FIXME: `where`
        // Function body.
        let body = input.parse::<Block>()?;

        Ok(DestructFunction { ident, generics, maybe_self, maybe_comma, bindings, return_type, body })
    }
}


impl DestructFunction {
    fn transform(self) -> TokenStream {
        let Self {
            ident,
            mut generics,
            maybe_self,
            maybe_comma,
            bindings,
            return_type,
            body,
        } = self;

        let args: Vec<_> = bindings.named.into_iter()
            .sorted_by_cached_key(|field| field.ident.as_ref().unwrap().to_string())
            .collect();
        let arg_idents: Vec<_> = args.iter().map(|field| field.ident.as_ref().unwrap()).collect();
        let arg_names: Vec<_> = arg_idents.iter().map(|ident| ident.to_string()).collect();

        // Generate new generic types for our named fields.
        let new_generics: Vec<TypeParam> = arg_names.iter()
            .map(|name| {
                let ident = format_ident!{"Obstruct_{}", name};
                parse_quote! {
                    #ident
                }
            })
            .collect();

        for (arg_name, (arg, generic_type)) in zip(&arg_names, zip(&args, &new_generics)) {
            let field_name = LitStr::new(arg_name.as_str(), arg.ident.as_ref().unwrap().span());
            let field_type = &arg.ty;
            let constrained = parse_quote!{
                #generic_type: obstruct::Field<#field_type, NAME=#field_name>
            };
            generics.params.push(constrained)
        }

        // Generate constraints for our named fields

        let result = quote! {
            // Outer function
            fn #ident #generics(#maybe_self #maybe_comma args: (#( #new_generics ),*) ) #return_type {
                use obstruct::Field;
                let (#(#arg_idents),*) = args;
                #(
                    let #arg_idents = #arg_idents.take();
                )*

                // FIXME: Self?

                // Inner function
                fn #ident( #(#args),* ) #return_type #body

                // And inner function call
                #ident(#(#arg_idents),*)
            }
        };
        eprintln!("YORIC: Generated code {}", result);
        result.into()
    }
}