use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::Paren,
    Error, Ident, Result, Token, Type,
};

use crate::context::{Context, NewType};

pub struct Visitor<'c> {
    pub new_idents: HashSet<String>,
    context: &'c Context,
}

impl<'c> Visitor<'c> {
    pub fn new(context: &'c Context) -> Self {
        let new_idents: HashSet<String> = context
            .new_types
            .iter()
            .map(|n| n.name().to_string())
            .collect();

        Self {
            new_idents,
            context,
        }
    }

    pub fn create_visitor(&self, context: &Context) -> TokenStream {
        let mut tokens = TokenStream::default();

        let func_impl = self.func_impl();

        tokens.append_all(quote! {
            pub trait Visitor<'ast> {
                #func_impl
            }
        });

        tokens
    }

    pub fn func_impl(&self) -> TokenStream {
        let mut tokens = TokenStream::default();

        let functions: Vec<TokenStream> = self
            .context
            .new_types
            .iter()
            .map(|nt| self.single_func(nt))
            .collect();
        tokens.extend(functions);

        tokens
    }

    pub fn single_func(&self, new_type: &NewType) -> TokenStream {
        let mut tokens = TokenStream::default();

        match new_type {
            NewType::Enum(e) => {
                let name = &e.name;
                let name_lower =
                    format_ident!("{}", name.to_string().to_lowercase(), span = name.span());
                let visit_name = format_ident!(
                    "visit_{}",
                    name.to_string().to_lowercase(),
                    span = name.span()
                );
                
                let variants = &e.variants;
                let variants_to_recurse = &e.variants.iter().filter(|v| {
                    let type_string =
                        v.ty.as_ref()
                            .map(|t| {
                                let mut tokens = TokenStream::default();
                                t.to_tokens(&mut tokens);
                                tokens.to_string()
                            })
                            .unwrap_or_default();

                    self.new_idents.contains(&type_string)
                });

                tokens.append_all(quote! {
                    fn #visit_name(v: &mut impl Visitor, #name_lower: #name) {

                    }
                });
            }
            NewType::Struct(s) => {}
            NewType::WrapperStruct(s) => {}
        }

        tokens
    }
}
