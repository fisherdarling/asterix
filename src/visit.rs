pub struct Expr;
pub struct Bar {
    pub lhs: Expr,
    pub rhs: Expr,
}

pub enum Baz {
    A(Bar),
    B(Expr),
    C,
}

macro_rules! create_walker {
    () => {};
    (
        {
            @struct
            name=[$name:ident]
            @[$(|$field:ident, $ty:tt|)*]    
        }
        $($tt:tt)*
    ) => {
        paste::item! {
            pub fn [<walk_ $name:lower>](v: &mut impl Visitor, [<$name:lower>]: &$name) {
                $(
                    paste::expr! { v.[<visit_ $ty:lower>](&[<$name:lower>].$field); }
                )*
            }
        }

        create_walker!($($tt)*);
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
                    v.[<visit_ $ty:lower>]([<$variant:lower>]);
                }
            )*

            $(
                fn [<walk_ $name:lower _ $raw:lower>](v: &mut impl Visitor) {
                    v.[<visit_ $name:lower _ $raw:lower>]();
                }
            )*
        }

        create_walker!($($tt)*);
    };
}

macro_rules! create_visitor {
    () => {};
    (   
        types=[
            $($tt:tt)+
        ]
    ) => {
        pub trait Visitor {
            create_visitor!(
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

        create_visitor!($($tt)*);
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

        create_visitor!($($tt)*);
    };
}

create_visitor!(
    types=[
        {   
            @struct
            name=[Bar]
            @[
                |lhs, Expr|
                |rhs, Expr|
            ]
        }
        {
            @enum
            name=[Baz]
            raw=[C]
            @[
                |A, Bar|
                |B, Expr|
            ]
        }
        {
            @struct
            name=[Expr]
            @[]
        }
    ]
);

create_walker!(
    {
        @struct
        name=[Expr]
        @[]
    }
    {   
        @struct
        name=[Bar]
        @[
            |lhs, Expr|
            |rhs, Expr|
        ]
    }
    {
        @enum
        name=[Baz]
        raw=[C]
        @[
            |A, Bar|
            |B, Expr|
        ]
    }
);
