// #![feature(proc_macro_hygiene)]
// #![feature(trace_macros)]

pub use asterix_impl::ast;

// /*
// ast!(
//     Float: |i32|, // Wrapper Syntax => struct Float(i32);
//     Int: struct Int {
//         inner: isize
//     }, // Struct Syntax => struct Int { inner: isize }
//     Expr: enum Expr { // Enum Syntax, is recursive
//         Call(isize) => // Simple Variant Syntax,
//         |Lit| => // Shorthand Lit(Lit),
//         Binop {           // Inline struct syntax
//             op: enum Op { // All new types are created
//                 Plus,     // outside of the Enum scope
//                 Minus,    // New types look like:
//                 Divides,  // `enum $NAME` or `struct $NAME`
//             },
//             lhs: Box<Expr>,
//             rhs: Box<Expr>,
//         },
//     },
// );
// */
// ast!(
//     Lit: enum Lit {
//         Int(isize),
//         Float(f32),
//         Ident |String|,
//         Str |String|,
//     },
//     Expr: enum Expr {
//         Call: struct Call {
//             lhs: Ident,
//             rhs: Box<Expr>,
//         },
//         BinOp: struct BinOp {
//             op: enum Op {
//                 Plus,
//                 Minus,
//                 Times,
//                 Divide,
//             },
//             lhs: Box<Expr>,
//             rhs: Box<Expr>,
//         },
//         |Lit|,
//         Unit,
//     },
//     Stmt: enum Stmt {
//         |Expr|,
//         Assignment: struct Assignment {
//             lhs: Ident,
//             rhs: Option<Expr>,
//         },
//         Print(Option<Expr>),
//         Ret |Option<Expr>|, // TODO: Fix From impl
//     },
//     Decl: enum Decl {
//         |Stmt|,
//     },
//     Program: struct Program {
//         decls: Vec<Decl>,
//     },
// );
