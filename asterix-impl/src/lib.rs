extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Result,
};

mod context;
use context::Context;

#[proc_macro]
pub fn ast(input: TokenStream) -> TokenStream {
    let context = parse_macro_input!(input as Context);
    context.create_ast().into()
}
