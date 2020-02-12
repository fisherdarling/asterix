//! ast!(
//!     Float: |f32|,
//!     Integer: i32,
//!     BinOp: struct {
//!         op: Op,
//!         lhs: Expr,
//!         rhs: Expr,
//!     },
//!     Expr: enum {
//!         BinOp,
//!         B: |i32|,
//!         C: struct {
//!             inner: usize,
//!         }
//!     },
//! );
//!

#[macro_export]
macro_rules! wrapper_struct {
    // Main entry
    (
        struct $name:ident {
            $($tt:tt)*
        }
    ) => {
        crate::wrapper_struct!(
            name=[$name]
            @[]
            $($tt)*
        );
    };
    // Standard lhs: ty syntax
    // Push to the stack, nothing to do
    (
        name=[$name:ident]
        @[$(|$a:ident, $b:tt|)*]
        $lhs:ident : $ty:ty,
        $($tt:tt)*
    ) => {
        crate::wrapper_struct!(
            name=[$name]
            @[
                |$lhs, $ty|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Single element tuple struct syntax, lhs: |ty|
    // Generate a wrapper
    (
        name=[$name:ident]
        @[$(|$a:ident, $b:tt|)*]
        $lhs:ident : $iname:ident |$ty:ty|,
        $($tt:tt)*
    ) => {
        crate::wrapper_struct!($iname |$ty|);

        crate::wrapper_struct!(
            name=[$name]
            @[
                |$lhs, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Struct syntax as a field. Generate
    // the struct and push the ident to the stack.
    //
    // lhs: struct Name {
    //     $($inner:tt)*
    // }
    (
        name=[$name:ident]
        @[$(|$a:ident, $b:tt|)*]
        $lhs:ident : struct $iname:ident {
            $($inner:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::wrapper_struct!(
            struct $iname {
                $($inner)*
            }
        );

        crate::wrapper_struct!(
            name=[$name]
            @[
                |$lhs, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Field is a new enum:
    // foo: enum Bar { ... }
    (
        name=[$name:ident]
        @[$(|$a:ident, $b:tt|)*]
        $lhs:ident : enum $iname:ident {
            $($inner:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::wrapper_enum!(
            enum $iname {
                $($inner)*
            }
        );

        crate::wrapper_struct!(
            name=[$name]
            @[
                |$lhs, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Tuple struct base case:
    // $name |$ty|
    (
        $name:ident |$ty:ty|
    ) => {
        #[derive(Debug, Clone, derive_more::AsRef, derive_more::AsMut, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
        #[from(forward)]
        pub struct $name(pub $ty);

        impl $name {
            pub fn new(t: $ty) -> Self {
                Self(t)
            }

            pub fn inner(&self) -> &$ty {
                &self.0
            }

            pub fn inner_mut(&mut self) -> &mut $ty {
                &mut self.0
            }

            pub fn into_inner(self) -> $ty {
                self.0
            }
        }
    };
    // (
    //     $name:ident |$ty:ty|
    // ) => {
    //     {
    //         @struct
    //         name=[$name]
    //         @[|0, $ty|]
    //     }
    // };
    // Field struct base case:
    // At this point the stack is full,
    // And there are no more tokens.
    (
        name=[$name:ident]
        @[$(|$field:ident, $ty:tt|)*]
    ) => {
        #[derive(Debug, Clone, getset::Getters, getset::Setters, getset::MutGetters)]
        pub struct $name {
            $(
                #[get]
                #[get_mut]
                pub $field: $ty,
            )*
        }

        impl $name {
            pub fn new($($field: $ty,)+) -> Self {
                Self {
                    $($field),+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! wrapper_enum {
    () => {};
    // Entry
    (
        enum $name:ident {
            $($tt:tt)*
        }
    ) => {
        crate::wrapper_enum!(
            name=[$name]
            raw=[]
            @[]
            $($tt)*
        );
    };
    // Variant with no type, this is the only time we modify raw:
    //
    // Foo,
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
        $var:ident,
        $($tt:tt)*
    ) => {
        crate::wrapper_enum!(
            name=[$name]
            raw=[
                |$var|
                $(|$raw|)*
            ]
            @[$(|$a, $b|)*]
            $($tt)*
        );
    };
    // Name-is-Type variant:
    // |Foo|,
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
        |$var:ident|,
        $($tt:tt)*
    ) => {
        crate::wrapper_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $var|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Variant with type:
    // Foo(isize),
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
        $var:ident($ty:ty),
        $($tt:tt)*
    ) => {
        crate::wrapper_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $ty|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Variant is new enum:
    // Foo: enum Bar { ... }
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
        $var:ident: enum $iname:ident {
            $($variants:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::wrapper_enum!(
            enum $iname {
                $($variants)*
            }
        );

        crate::wrapper_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Variant is new tuple struct:
    // Foo |isize|,
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
        $var:ident |$ty:ty|,
        $($tt:tt)*
    ) => {
        crate::wrapper_struct!($var |$ty|);

        crate::wrapper_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $var|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Variant is new struct struct:
    // Foo: struct Bar { ... }
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
        $var:ident : struct $iname:ident {
            $($fields:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::wrapper_struct!(
            struct $iname {
                $($fields)*
            }
        );

        crate::wrapper_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Base case: stack is full, nothing
    // left to munch. Not marked as inner,
    // so we do visitor generation here:
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
    ) => {
        #[derive(Debug, Clone, derive_more::From)]
        pub enum $name {
            $(
                $a($b),
            )*
            $(
                $raw
            ),*
        }
    };
    // No visitor should be generated,
    // marked as @inner
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:tt|)*]
    ) => {
        #[derive(Debug, Clone, derive_more::From)]
        pub enum $name {
            $(
                $a($b),
            )*
            $(
                $raw
            ),*
        }
    };
}

/// This generates a simple AST:
///
/// There is some new syntax to facilitate construction
/// of new types. The macro creates an `ast` module, and places all
/// of the types in it.
///
/// To create a tuple struct of one element:
/// Foo |$type| => pub struct Foo(pub $type);
///
/// To create a struct of multiple elements,
/// and assign it to a field:
/// field: struct Baz { ... }
///
/// Creating an enum is always:
/// enum A { ... }
///
/// You can place that anywhere you can assign
/// a type to an identifier:
///
/// // As a variant:
/// A: enum Fuzz { ... }
///
/// // As a field:
/// field: enum Farx { ... }
///
/// To create a variant of a type, where the type name is the
/// same as the variant name:
/// |Lit| => Lit(Lit)
///
/// A variant which is just a label is:
/// Unit => Unit
///
/// Every Identifier following an `enum` or `struct`
/// is a completely new type in the scope of the macro.
/// Watch out for name collisions. This also means you can
/// create the type once, and use it later without redefining it.
#[macro_export]
macro_rules! create_ast {
    (
        $($variants:tt)+
    ) => {
        pub mod ast {
            crate::wrapper_enum!(
                name=[Ast]
                raw=[]
                @[]
                $($variants)*
            );

            // crate::create_walker!(
            //     enum Ast {
            //         $($variants)*
            //     }
            // );

            crate::create_visitor!(
                enum Ast {
                    $($variants)*
                }
            );

        }
    };
}

// trace_macros!(true);

// create_ast!(
//     Lit: enum Lit {
//         A(isize),
//         B(f64),
//         C(String),
//     },
// );

// create_ast!(
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
