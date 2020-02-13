// #![macro_use]

#[macro_export]
macro_rules! create_walker_struct {
    // Main entry
    (
        struct $name:ident {
            $($tt:tt)*
        }
    ) => {
        crate::create_walker_struct!(
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
        crate::create_walker_struct!(
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
        crate::create_walker_struct!($iname |$ty|);

        crate::create_walker_struct!(
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
        crate::create_walker_struct!(
            struct $iname {
                $($inner)*
            }
        );
        crate::create_walker_struct!(
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
        crate::create_walker_enum!(
            enum $iname {
                $($inner)*
            }
        );
        crate::create_walker_struct!(
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
        crate::create_walker! {
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
        crate::create_walker! {
            @struct
            name=[$name]
            @[$(|$field, $ty|)*]
        }
    };
}

// trace_macros!(true);

#[macro_export]
macro_rules! create_walker_enum {
    () => {};
    // Entry
    (
        enum $name:ident {
            $($tt:tt)*
        }
    ) => {
        crate::create_walker_enum!(
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
        crate::create_walker_enum!(
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
        crate::create_walker_enum!(
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
        crate::create_walker_enum!(
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
        crate::create_walker_enum!(
            enum $iname {
                $($variants)*
            }
        );

        crate::create_walker_enum!(
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
        crate::create_walker_struct!($var |$ty|);

        crate::create_walker_enum!(
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
        crate::create_walker_struct!(
            struct $iname {
                $($fields)*
            }
        );

        crate::create_walker_enum!(
            name=[$name]
            raw=[$(|$raw|)*]
            @[
                |$var, $iname|
                $(|$a, $b|)*
            ]
            $($tt)*
        );
    };
    // Base create_walker case:
    (
        name=[$name:ident]
        raw=[$(|$raw:ident|)*]
        @[$(|$a:ident, $b:ty|)*]
    ) => {
        crate::create_walker! {
            @enum
            name=[$name]
            raw=[$(|$raw:ident|)*]
            @[$(|$a, $b|)*]
        }
    };
}

#[macro_export]
macro_rules! create_walker {
    () => {};
    (
        @struct
        name=[$name:ident]
        @[$(|$field:tt, $ty:ty|)*]
    ) => {
        paste::item! {
            pub fn [<walk_ $name:lower>](v: &mut impl Visitor, [<$name:lower>]: &$name) {
                $(
                    paste::expr! { v.[<visit_ $ty:lower>](&[<$name:lower>].$field); }
                )*
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
            pub fn [<walk_ $name:lower>](v: &mut impl Visitor, [<$name:lower>]: &$name) {
                match [<$name:lower>] {
                    $(
                        $name::$variant(value) => v.[<visit_ $name:lower _ $variant:lower>](&value),
                    )*
                    $(
                        $name::$raw => v.[<visit_ $name:lower _ $raw:lower>]()
                    ),*
                }
            }

            $(
                fn [<walk_ $name:lower _ $variant:lower>](v: &mut impl Visitor, [<$variant:lower>]: &$ty) {
                    v.[<visit_ $variant:lower>]([<$variant:lower>]);
                }
            )*

            $(
                fn [<walk_ $name:lower _ $raw:lower>](v: &mut impl Visitor) {
                    v.[<visit_ $name:lower _ $raw:lower>]();
                }
            )*
        }
    };
    ($($tt:tt)*) => { crate::create_walker_enum!($($tt)*); }
}
