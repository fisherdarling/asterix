// // pub struct Foo;
// // pub struct Bar;

// // macro_rules! create_walk_impl {
// //     // Create a walk impl for an enum.
// //     // We take in the completed stack
// //     // to reduce our own parsing work
// //     (
// //         @enum
// //         name=[$name:ident]
// //         raw=[$(|$raw:ident|)*]
// //         @[$(|$a:ident, $b:tt|)*]
// //     ) => {

// //     };
// //     (   
// //         @struct
// //         name=[$name:ident]
// //         @[$(|$field:ident, $ty:tt|)*]
// //     ) => {
// //         paste::item! {
// //             fn [<walk_ $name>](v: &mut Visitor, casey::lower!($name): &$name) {
                        
// //             }
// //         }
// //     };
// // }

// // create_walk_impl!(
// //     @struct
// //     name=[Bar]
// //     @[]
// // );

// macro_rules! create_visitor {
//     (
//         @[$(|$item:ident $(, $param:ident)*|)*]
//     ) => {
//         paste::item_with_macros! {
//             pub trait Visitor {
//                 $(
//                     fn [<visit_ $item>](&mut self, $(casey::lower!($param): &$param),*) {
//                         paste::expr! {
//                             [<self . visit_ $item]
//                         }
//                     }
//                 )*
//             }
//         }
//     };
// }