extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

pub(crate) mod context;
mod visitor;

use context::Context;
use visitor::Visitor;

#[proc_macro]
pub fn ast(input: TokenStream) -> TokenStream {
    let context = parse_macro_input!(input as Context);
    let visitor = Visitor::new(&context);

    let visit_impl = visitor.create_visitor();
    let ast_impl = context.create_ast(Some(visit_impl));

    ast_impl.into()
}
