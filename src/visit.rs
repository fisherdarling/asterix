// #![macro_use]

// #[macro_use]
#[macro_export]
macro_rules! create_visitor {
    (
        enum $name:ident {
            $($tt:tt)*
        }
    ) => {
    };
    (
        types=[
            $($tt:tt)+
        ]
    ) => {
        // crate::create_walker!(
        //     $($tt)*
        // );

        pub trait Visitor {
            crate::create_visitor!(
                $($tt)+
            );
        }
    };
    (
        {
            @struct
            name=[$name:ident]
            @[$(|$field:ident, $ty:tt|)*]
        }
        $($tt:tt)*
    ) => {
        paste::item! {
            fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: &$name) where Self: Sized {
                $(
                    paste::expr! { [<walk_ $ty:lower>](self, &[<$name:lower>].$field); }
                )*
            }
        }

        crate::create_visitor!($($tt)*);
    };
    (
        {
            @enum
            name=[$name:ident]
            raw=[$($raw:ident)*]
            @[$(|$variant:ident, $ty:tt|)*]
        }
        $($tt:tt)*
    ) => {
        paste::item! {
            fn [<visit_ $name:lower>](&mut self, [<$name:lower>]: &$name) where Self: Sized {
                match [<$name:lower>] {
                    $(
                        $name::$variant(v) => [<walk_ $name:lower _ $variant:lower>](self, &v),
                    )*
                    $(
                        $name::$raw => [<walk_ $name:lower _ $raw:lower>](self),
                    ),*
                }
            }

            $(
                fn [<visit_ $name:lower _ $variant:lower>](&mut self, [<$variant:lower>]: &$ty) where Self: Sized {
                    [<walk_ $ty:lower>](self, [<$variant:lower>]);
                }
            )*
            $(
                fn [<visit_ $name:lower _ $raw:lower>](&mut self) {}
            )*
        }

        crate::create_visitor!($($tt)*);
    };
}

// create_visitor!(
//     types=[
//         {
//             @struct
//             name=[Bar]
//             @[
//                 |lhs, Expr|
//                 |rhs, Expr|
//             ]
//         }
//         {
//             @enum
//             name=[Baz]
//             raw=[C]
//             @[
//                 |A, Bar|
//                 |B, Expr|
//             ]
//         }
//         {
//             @struct
//             name=[Expr]
//             @[]
//         }
//     ]
// );

// create_walker!(
//     {
//         @struct
//         name=[Expr]
//         @[]
//     }
//     {
//         @struct
//         name=[Bar]
//         @[
//             |lhs, Expr|
//             |rhs, Expr|
//         ]
//     }
//     {
//         @enum
//         name=[Baz]
//         raw=[C]
//         @[
//             |A, Bar|
//             |B, Expr|
//         ]
//     }
// );
