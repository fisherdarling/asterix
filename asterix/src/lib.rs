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

// ast!(
//     Lit |isize|
// );

ast!(
    Lit |isize|,
    Expr: enum Expr {
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
        |Lit|
    }
);

use ast::*;

pub struct Interpreter;

impl<'ast> Visitor<'ast> for Interpreter {
    type Output = isize;

    fn visit_binop(&mut self, b: &BinOp) -> isize {
        let lhs = self.visit_expr(b.lhs());
        let rhs = self.visit_expr(b.rhs());

        match b.op() {
            Op::Plus => lhs + rhs,
            Op::Minus => lhs - rhs,
            Op::Times => lhs * rhs,
            Op::Divide => lhs / rhs,
        }
    }

    fn visit_lit(&mut self, lit: &Lit) -> isize {
        *lit.inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_expr() {
        let one = Box::new(Expr::lit(1));
        let one_p_one = BinOp::new(Op::Plus, one.clone(), one.clone());

        let two = Box::new(Expr::lit(2));
        let minus_two = BinOp::new(Op::Minus, Box::new(Expr::binop(one_p_one.clone())), two);

        let mut interpreter = Interpreter;

        let result = interpreter.visit_ast(&Ast::expr(one_p_one));
        println!("1 + 1:       {}", result);
        let result = interpreter.visit_ast(&Ast::expr(minus_two));
        println!("(1 + 1) - 2: {}", result);
    }
}