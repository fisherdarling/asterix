use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::Paren,
    Error, Ident, Result, Token, Type,
};

#[derive(Default, Debug)]
pub struct Context {
    pub new_types: Vec<NewType>,
    pub variants: Punctuated<Variant, Token![,]>,
}

impl Context {
    pub fn create_ast(self, visitor: Option<TokenStream>) -> proc_macro2::TokenStream {
        let ast_name = Ident::new("Ast", Span::call_site());
        let new_types = self.new_types;
        let ast = EnumType {
            name: ast_name,
            variants: self.variants,
        };

        quote! {
            pub mod ast {
                #(#new_types)*
                #ast
                #visitor
            }
        }
    }
}

fn flatten(new_type: &mut NewType) -> Vec<NewType> {
    // println!("{:?}", new_type);
    let mut types = Vec::new();
    types.push(new_type.clone());

    match new_type {
        NewType::Enum(e) => {
            for variant in &mut e.variants {
                if let Some(ref mut nt) = &mut variant.new_type {
                    types.append(&mut flatten(nt));
                }

                variant.new_type = None;
            }
        }
        NewType::Struct(s) => {
            for field in &mut s.fields {
                if let Some(ref mut nt) = &mut field.new_type {
                    types.append(&mut flatten(nt));
                }

                field.new_type = None;
            }
        }
        _ => (),
    }

    types
}

impl Parse for Context {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variants = input.parse_terminated(Variant::parse)?;
        let mut new_types = Vec::new();

        for variant in &mut variants {
            // println!("VARIANT: {:?}", variant);

            if let Some(nt) = &mut variant.new_type {
                // println!("NEW TYPE {:?}\n", nt);
                let flattened = flatten(nt);
                new_types.extend(flattened.into_iter());
            }

            variant.new_type = None;
        }

        Ok(Context {
            new_types,
            variants,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub new_type: Option<NewType>,
    pub name: Ident,
    pub ty: Option<Type>,
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;

        if let Some(ty) = &self.ty {
            tokens.append_all(quote! {
                #name(#ty)
            })
        } else {
            tokens.append_all(quote! {
                #name
            })
        }
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut new_type = None;
        let name: Ident;
        let mut ty: Option<Type> = None;

        // Shorthand Syntax |Lit|,
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![|]) {
            input.parse::<Token![|]>()?;
            name = input.parse::<Ident>()?;
            ty = Some(parse_quote!(#name));

            input.parse::<Token![|]>()?;
        } else if lookahead.peek(Ident) {
            name = input.parse::<Ident>()?;
            let lookahead = input.lookahead1();

            if lookahead.peek(Token![:]) {
                // NewType syntax
                input.parse::<Token![:]>()?;
                let parsed_new_type = input.call(NewType::parse)?;
                let new_name = parsed_new_type.name();
                ty = Some(parse_quote!(#new_name));
                new_type = Some(parsed_new_type);
            } else if lookahead.peek(Paren) {
                // Standard Variant syntax
                let content;
                parenthesized!(content in input);
                ty = Some(content.parse::<Type>()?);
            } else if lookahead.peek(Token![|]) {
                // Variant shorthand for Wrapper struct shorthand
                // If the Variant is the same name as the wrapper
                input.parse::<Token![|]>()?;

                new_type = Some(NewType::WrapperStruct(WrapperStruct {
                    name: name.clone(),
                    ty: input.parse::<Type>()?,
                }));

                ty = Some(parse_quote!(#name));
                input.parse::<Token![|]>()?;
            }
        } else {
            return Err(lookahead.error());
        }

        Ok(Variant { new_type, name, ty })
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub new_type: Option<NewType>,
    pub ident: Ident,
    pub ty: Type,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.ident;
        let ty = &self.ty;

        tokens.append_all(quote! {
            #id: #ty
        });
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut new_type = None;
        let ty;

        let ident = input.parse::<Ident>()?;

        input.parse::<Token![:]>()?;

        if input.fork().parse::<Type>().is_ok() {
            ty = input.parse::<Type>()?;

            Ok(Field {
                new_type,
                ident,
                ty,
            })
        } else {
            let nt = input.call(NewType::parse)?;
            let ident_ty: Ident = nt.name().clone();

            new_type = Some(nt);
            ty = parse_quote!(#ident_ty);

            Ok(Field {
                new_type,
                ident,
                ty,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum NewType {
    Enum(EnumType),
    Struct(StructType),
    WrapperStruct(WrapperStruct),
}

impl NewType {
    pub fn name(&self) -> &Ident {
        match self {
            Self::Enum(e) => &e.name,
            Self::Struct(s) => &s.name,
            Self::WrapperStruct(w) => &w.name,
        }
    }
}

impl Parse for NewType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![struct]) {
            let new_struct = input.call(StructType::parse)?;
            Ok(NewType::Struct(new_struct))
        } else if lookahead.peek(Token![enum]) {
            let new_enum = input.call(EnumType::parse)?;
            Ok(NewType::Enum(new_enum))
        } else if lookahead.peek(Ident) && input.peek2(Token![|]) {
            let new_wrapper = input.call(WrapperStruct::parse)?;
            Ok(NewType::WrapperStruct(new_wrapper))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumType {
    pub name: Ident,
    pub variants: Punctuated<Variant, Token![,]>,
}

impl Parse for EnumType {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![enum]>()?;
        let name = input.parse::<Ident>()?;

        let inner;
        braced!(inner in input);

        let variants = inner.parse_terminated(Variant::parse)?;

        Ok(EnumType { name, variants })
    }
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub name: Ident,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for StructType {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![struct]>()?;
        let name = input.parse::<Ident>()?;

        let inner;
        braced!(inner in input);

        let fields = inner.parse_terminated(Field::parse)?;

        Ok(StructType { name, fields })
    }
}

#[derive(Debug, Clone)]
pub struct WrapperStruct {
    pub name: Ident,
    pub ty: Type,
}

impl Parse for WrapperStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![|]>()?;
        let ty = input.parse::<Type>()?;
        input.parse::<Token![|]>()?;

        Ok(WrapperStruct { name, ty })
    }
}

impl ToTokens for NewType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            NewType::Enum(e) => e.to_tokens(tokens),
            NewType::Struct(s) => s.to_tokens(tokens),
            NewType::WrapperStruct(w) => w.to_tokens(tokens),
        }
    }
}

impl ToTokens for EnumType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, variants, .. } = self;

        let variant_iter = variants.iter();

        // let (names, types): (Vec<&Ident>, Vec<&Option<Type>>) = variants.iter().map(|v| (&v.name, &v.ty)).unzip();
        let (typed, raw): (Vec<&Variant>, Vec<&Variant>) =
            variants.iter().partition(|v| v.ty.is_some());
        // let (typed, raw) = (typed.iter().map(|v| &v.name), raw.iter().map(|v| &v.name));
        let (typed_names, types): (Vec<&Ident>, Vec<Type>) = typed
            .iter()
            .map(|v| (&v.name, v.ty.clone().unwrap()))
            .unzip();

        let typed_names_lower = typed_names.iter().map(|i| {
            let lower = i.to_string().to_lowercase();
            Ident::new(&lower, i.span())
        });

        let raw_names = raw.iter().map(|r| &r.name);
        let raw_names_lower = raw_names.clone().map(|i| {
            let lower = i.to_string().to_lowercase();
            Ident::new(&lower, i.span())
        });

        tokens.append_all(quote! {
            #[derive(Debug, Clone)]
            pub enum #name {
                #(#variant_iter),*
            }

            impl #name {
                #(
                    pub fn #typed_names_lower(e: #types) -> Self {
                        #name::#typed_names(e)
                    }
                )*

                #(
                    pub fn #raw_names_lower() -> Self {
                        #name::#raw_names
                    }
                )*
            }
        })
    }
}

impl ToTokens for StructType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, fields, .. } = self;

        let field_names: Vec<&Ident> = fields.iter().map(|f| &f.ident).collect();
        let field_types: Vec<&Type> = fields.iter().map(|f| &f.ty).collect();
        let field_names_mut: Vec<Ident> = fields
            .iter()
            .map(|f| {
                let name = format!("{}_mut", f.ident);
                Ident::new(&name, f.ident.span())
            })
            .collect();

        tokens.append_all(quote! {
            #[derive(Debug, Clone)]
            pub struct #name {
                #(#field_names : #field_types),*
            }

            impl #name {
                pub fn new(#(#field_names : #field_types),*) -> Self {
                    Self {
                        #(#field_names),*
                    }
                }

                #(
                    pub fn #field_names(&self) -> &#field_types {
                        &self.#field_names
                    }
                )*

                #(
                    pub fn #field_names_mut(&mut self) -> &mut #field_types {
                        &mut self.#field_names
                    }
                )*
            }
        });
    }
}

impl ToTokens for WrapperStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ty = &self.ty;

        tokens.append_all(quote! {
            #[derive(Debug, Clone)]
            pub struct #name(#ty);

            impl #name {
                pub fn new(inner: #ty) -> Self {
                    Self(inner)
                }

                pub fn inner(&self) -> &#ty {
                    &self.0
                }

                pub fn inner_mut(&mut self) -> &mut #ty {
                    &mut self.0
                }

                pub fn into_inner(self) -> #ty {
                    self.0
                }
            }

            impl From<#ty> for #name {
                fn from(i: #ty) -> Self {
                    Self(i)
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variant() {
        let variant: Variant = parse_quote! { Call(Lmao) };
        println!("{:#?}\n", variant);

        let variant: Variant = parse_quote! { |Call| };
        println!("{:#?}\n", variant);

        let variant: Variant = parse_quote! { Call |isize| };
        println!("{:#?}\n", variant);

        let variant: Variant = parse_quote! { Call: struct Call {} };
        println!("{:#?}\n", variant);
    }

    #[test]
    fn parse_field() {
        let field: Field = parse_quote! { a: usize };
        println!("{:#?}\n", field);

        let field: Field = parse_quote! { a: struct A {

        }};
        println!("{:#?}\n", field);

        let field: Field = parse_quote! { a: enum A {

        }};
        println!("{:#?}\n", field);
    }

    #[test]
    fn parse_context() {
        let context: Context = parse_quote! {
            Lit: enum Lit {
                Int(isize),
                Float(f32),
                Ident |String|,
                Str |String|,
            },
            Expr: enum Expr {
                Call: struct Call {
                    lhs: Ident,
                    rhs: Box<Expr>,
                },
                BinOp: struct BinOp {
                    op: enum Op {
                        Plus,
                        Minus,
                        Times,
                        Divide,
                    },
                    lhs: Box<Expr>,
                    rhs: Box<Expr>,
                },
                |Lit|,
                Unit,
            },
            Stmt: enum Stmt {
                |Expr|,
                Assignment(Assignment),
                Assignment: struct Assignment {
                    lhs: Ident,
                    rhs: Option<Expr>,
                },
                Print(Option<Expr>),
                Ret |Option<Expr>|,
            },
            Decl: enum Decl {
                |Stmt|,
            },
            Program: struct Program {
                decls: Vec<Decl>,
            },
        };

        for nt in context.new_types {
            println!("{}", nt.name());
        }
    }

    #[test]
    fn to_tokens() {
        let s_type: StructType = parse_quote! {
            struct A {
                lhs: usize,
                rhs: f32,
            }
        };
        let tokens = quote! { #s_type };
        println!("{}", tokens);

        let e_type: EnumType = parse_quote! {
            enum T {
                A(isize),
                B(Box<A>),
                C: struct D {
                    lhs: String,
                    rhs: String,
                }
            }
        };
        let tokens = quote! { #e_type };
        println!("{}", tokens);

        let w_type: NewType = parse_quote! {
            Ident |String|
        };
        let tokens = quote! { #w_type };
        println!("{}", tokens);
    }
}
