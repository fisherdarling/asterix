use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::Paren, spanned::Spanned,
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

    pub fn create_visitor(&self) -> TokenStream {
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

                // let variants = &e.variants;

                // Raw variants will be: visit_name_rawname()
                let raw_variants = e.variants.iter().filter(|v| v.ty.is_none());

                // New Type variants will be: visit_newtype(newtype)
                // Basic Type variants will be: visit_name_variantname(basic_type);
                let (new_type_variants, basic_type_variants): (Vec<_>, Vec<_>) =
                    e.variants.iter().filter(|v| v.ty.is_some()).partition(|v| {
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

                let raw_idents = raw_variants.clone().map(|v| &v.name);
                let new_type_idents = new_type_variants.iter().map(|v| &v.name);
                let new_type_types = new_type_variants.iter().map(|v| &v.ty).flatten();
                let basic_idents = basic_type_variants.iter().map(|v| &v.name);
                let basic_types = basic_type_variants.iter().map(|v| &v.ty).flatten();

                let raw_visit = raw_idents.clone().map(|i| {
                    format_ident!(
                        "visit_{}_{}",
                        name.to_string().to_lowercase(),
                        i.to_string().to_lowercase(),
                        span = i.span()
                    )
                });
                let new_type_visit = new_type_types.clone().map(|ty| {
                    let mut tokens = TokenStream::new();
                    ty.to_tokens(&mut tokens);
                    format_ident!("visit_{}", tokens.to_string().to_lowercase())
                });
                let basic_visit = basic_idents.clone().map(|i| {
                    format_ident!(
                        "visit_{}_{}",
                        name.to_string().to_lowercase(),
                        i.to_string().to_lowercase(),
                        span = i.span()
                    )
                });
                let basic_visit_func = basic_visit.clone();
                let basic_types_func = basic_types.clone();

                let raw_visit_func = raw_visit.clone();

                tokens.append_all(quote! {
                    fn #visit_name(&mut self, #name_lower: &'ast #name) where Self: Sized {
                        match #name_lower {
                            #(
                                #name::#new_type_idents(v) => self.#new_type_visit(&v),
                            )*
                            #(
                                #name::#basic_idents(v) => self.#basic_visit(&v),
                            )*
                            #(
                                #name::#raw_idents => self.#raw_visit(),
                            )*
                        }
                    }

                    #(
                        fn #basic_visit_func(&mut self, v: &'ast #basic_types_func) where Self: Sized {}
                    )*

                    #(
                        fn #raw_visit_func(&mut self) where Self: Sized {}
                    )*
                });
            }
            NewType::Struct(s) => {
                let name = &s.name;
                let name_lower =
                    format_ident!("{}", name.to_string().to_lowercase(), span = name.span());
                let visit_name = format_ident!(
                    "visit_{}",
                    name.to_string().to_lowercase(),
                    span = name.span()
                );

                let (new_type_field_names, new_type_visit): (Vec<_>, Vec<_>) = s.fields.iter().filter_map(|f| {
                    let mut tokens = TokenStream::new();
                    f.ty.to_tokens(&mut tokens);

                    if self.new_idents.contains(&tokens.to_string()) {
                        Some((&f.ident, format_ident!("visit_{}", tokens.to_string().to_lowercase())))
                    } else {
                        None
                    }
                }).unzip();

                let name_lower_func = name_lower.clone();
                let name_lower_repeat = (0..new_type_field_names.len()).map(|_| &name_lower);

                tokens.append_all(quote! {
                    fn #visit_name(&mut self, #name_lower: &'ast #name) where Self: Sized {
                        #(
                            self.#new_type_visit(&#name_lower_repeat.#new_type_field_names);
                        )*
                    }
                });
            }
            NewType::WrapperStruct(s) => {
                let name = &s.name;
                let name_lower =
                    format_ident!("{}", name.to_string().to_lowercase(), span = name.span());
                let visit_name = format_ident!(
                    "visit_{}",
                    name.to_string().to_lowercase(),
                    span = name.span()
                );

                let type_string = {
                    let mut tokens = TokenStream::new();
                    s.ty.to_tokens(&mut tokens);
                    tokens.to_string()
                };

                let name_lower_inner = name_lower.clone();
                let inner_call = if self.new_idents.contains(&type_string) {
                    let visit_new_type = format_ident!("visit_{}", type_string.to_lowercase(), span=s.ty.span());
                    Some(quote! { self.#visit_new_type(#name_lower_inner.inner()) })
                } else {
                    None
                };

                tokens.append_all(quote! {
                    fn #visit_name(&mut self, #name_lower: &#name) where Self: Sized {
                        #inner_call
                    }
                });
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn simple_enum() {
        let context: Context = parse_quote! {
            Lit |isize|,
            Expr |Lit|,
            Binop: struct Binop {
                lhs: Expr,
                rhs: Expr,
            },
            E: enum E {
                Alpha(isize), // Basic type
                Beta(Ident),  // New Type
                Gamma,        // Raw Type
            }
        };

        let visitor = Visitor::new(&context);
        let impl_tokens = visitor.create_visitor();

        println!("{}", impl_tokens);
    }
}