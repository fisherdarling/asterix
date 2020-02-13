// // #![macro_use]

// #![macro_use]

#[macro_export]
macro_rules! create_visitor_struct {
    // Main entry
    (
        struct $name:ident {
            $($tt:tt)*
        }
    ) => {
        crate::create_visitor_struct!(
            name=[$name]
            @[]
            $($tt)*
        );
    };
    // Standard lhs: ty syntax
    // Push to the stack, nothing to do
    (
        name=[$name:ident]
        @[$(|$a:ident, $b:ty|)*]
        $lhs:ident : $ty:ty,
        $($tt:tt)*
    ) => {
        crate::create_visitor_struct!(
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
        @[$(|$a:ident, $b:ty|)*]
        $lhs:ident : $iname:ident |$ty:ty|,
        $($tt:tt)*
    ) => {
        crate::create_visitor_struct!($iname |$ty|);

        crate::create_visitor_struct!(
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
        @[$(|$a:ident, $b:ty|)*]
        $lhs:ident : struct $iname:ident {
            $($inner:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::create_visitor_struct!(
            struct $iname {
                $($inner)*
            }
        );
        crate::create_visitor_struct!(
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
        @[$(|$a:ident, $b:ty|)*]
        $lhs:ident : enum $iname:ident {
            $($inner:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::create_visitor_enum!(
            enum $iname {
                $($inner)*
            }
        );
        crate::create_visitor_struct!(
            name=[$name]
            @[
                |$lhs, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    (
        $name:ident |$ty:ty|
    ) => {
        crate::create_visitor! {
            @struct
            name=[$name]
            @[|0, $ty|]
        }
    };
    // Field struct base case:
    // At this point the stack is full,
    // And there are no more tokens.
    (
        name=[$name:ident]
        @[$(|$field:tt, $ty:ty|)*]
    ) => {
        crate::create_visitor! {
            @struct
            name=[$name]
            @[$(|$field, $ty|)*]
        }
    };
}

// trace_macros!(true);

#[macro_export]
macro_rules! create_visitor_enum {
    () => {};
    // Entry
    (
        enum $name:ident {
            $($tt:tt)*
        }
    ) => {
        crate::create_visitor_enum!(
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
        @[$(|$a:ident, $b:ty|)*]
        $var:ident,
        $($tt:tt)*
    ) => {
        crate::create_visitor_enum!(
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
        @[$(|$a:ident, $b:ty|)*]
        |$var:ident|,
        $($tt:tt)*
    ) => {
        crate::create_visitor_enum!(
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
        @[$(|$a:ident, $b:ty|)*]
        $var:ident($ty:ty),
        $($tt:tt)*
    ) => {
        crate::create_visitor_enum!(
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
        @[$(|$a:ident, $b:ty|)*]
        $var:ident: enum $iname:ident {
            $($variants:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::create_visitor_enum!(
            enum $iname {
                $($variants)*
            }
        );

        crate::create_visitor_enum!(
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
        @[$(|$a:ident, $b:ty|)*]
        $var:ident |$ty:ty|,
        $($tt:tt)*
    ) => {
        crate::create_visitor_struct!($var |$ty|);

        crate::create_visitor_enum!(
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
        @[$(|$a:ident, $b:ty|)*]
        $var:ident : struct $iname:ident {
            $($fields:tt)*
        },
        $($tt:tt)*
    ) => {
        crate::create_visitor_struct!(
            struct $iname {
                $($fields)*
            }
        );

        crate::create_visitor_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Base create_visitor case:
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:ty|)*]
    ) => {
        crate::create_visitor! {
            @enum
            name=[$name]
            raw=[$(|$raw:ident|)*]
            @[$(|$a, $b|)*]
        }
    };
}

#[macro_export]
macro_rules! create_visitor {
    () => {};
    (
        @struct
        name=[$name:ident]
        @[$(|$field:tt, $ty:tt|)*]
    ) => {
        paste::item! {
            fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: &$name) where Self: Sized {
                paste::expr! { [<walk_$name:lower>](self, [<$name:lower>]) }
                // $(
                //     paste::expr! { v.[<visit_ $ty:lower>](&[<$name:lower>].$field); }
                // )*
            }
        }
    };
    (
        @enum
        name=[$name:ident]
        raw=[$($raw:ident)*]
        @[$(|$variant:ident, $ty:tt|)*]
    ) => {
        paste::item! {
            fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: &$name) where Self: Sized {
                match [<$name:lower>] {
                    $(
                        $name::$variant(value) => [<walk_ $name:lower _ $variant:lower>](self, &value),
                    )*
                    $(
                        $name::$raw => [<walk_ $name:lower _ $raw:lower>](self)
                    ),*
                }
            }

            $(
                fn [<visit_ $name:lower _ $variant:lower>](&mut self, [<$variant:lower>]: &$ty) where Self: Sized  {
                    [<walk_ $ty:lower>](self, [<$variant:lower>]);
                }
            )*

            $(
                fn [<visit_ $name:lower _ $raw:lower>](&mut self) where Self: Sized  {}
            )*

            // $(
            //     fn [<visit_ $name:lower _ $raw:lower>](&mut self) where Self: Sized  {}
            // )*
        }
    };
    ($($tt:tt)*) => { 
        pub trait Visitor {
            crate::create_visitor_enum!(
                $($tt)*
            ); 
        }
    }
}


// // #[macro_use]
// #[macro_export]
// macro_rules! create_visitor {
//     (
//         enum $name:ident {
//             $($tt:tt)*
//         }
//     ) => {
//         create_visitor!(

//         );
//     };
//     (
//         types=[
//             $($tt:tt)+
//         ]
//     ) => {
//         // crate::create_visitor!(
//         //     $($tt)*
//         // );

//         pub trait Visitor {
//             crate::create_visitor!(
//                 $($tt)+
//             );
//         }
//     };
//     (
//         {
//             @struct
//             name=[$name:ident]
//             @[$(|$field:ident, $ty:tt|)*]
//         }
//         $($tt:tt)*
//     ) => {
//         paste::item! {
//             fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: &$name) where Self: Sized {
//                 $(
//                     paste::expr! { [<walk_ $ty:lower>](self, &[<$name:lower>].$field); }
//                 )*
//             }
//         }

//         crate::create_visitor!($($tt)*);
//     };
//     (
//         {
//             @enum
//             name=[$name:ident]
//             raw=[$($raw:ident)*]
//             @[$(|$variant:ident, $ty:tt|)*]
//         }
//         $($tt:tt)*
//     ) => {
//         paste::item! {
//             fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: &$name) where Self: Sized {
//                 match [<$name:lower>] {
//                     $(
//                         $name::$variant(v) => [<walk_ $name:lower _ $variant:lower>](self, &v),
//                     )*
//                     $(
//                         $name::$raw => [<walk_ $name:lower _ $raw:lower>](self),
//                     ),*
//                 }
//             }

//             $(
//                 fn [<visit_ $name:lower _ $variant:lower>](&mut self, [<$variant:lower>]: &$ty) where Self: Sized {
//                     [<walk_ $ty:lower>](self, [<$variant:lower>]);
//                 }
//             )*
//             $(
//                 fn [<visit_ $name:lower _ $raw:lower>](&mut self) {}
//             )*
//         }

//         crate::create_visitor!($($tt)*);
//     };
// }

// // create_visitor!(
// //     types=[
// //         {
// //             @struct
// //             name=[Bar]
// //             @[
// //                 |lhs, Expr|
// //                 |rhs, Expr|
// //             ]
// //         }
// //         {
// //             @enum
// //             name=[Baz]
// //             raw=[C]
// //             @[
// //                 |A, Bar|
// //                 |B, Expr|
// //             ]
// //         }
// //         {
// //             @struct
// //             name=[Expr]
// //             @[]
// //         }
// //     ]
// // );

// // create_visitor!(
// //     {
// //         @struct
// //         name=[Expr]
// //         @[]
// //     }
// //     {
// //         @struct
// //         name=[Bar]
// //         @[
// //             |lhs, Expr|
// //             |rhs, Expr|
// //         ]
// //     }
// //     {
// //         @enum
// //         name=[Baz]
// //         raw=[C]
// //         @[
// //             |A, Bar|
// //             |B, Expr|
// //         ]
// //     }
// // );
