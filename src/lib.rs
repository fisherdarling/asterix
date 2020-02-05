#![feature(proc_macro_hygiene)]
#![feature(trace_macros)]
#![feature(concat_idents)]
#![allow(unused)]

#[macro_use]
pub mod ast;

pub mod visit;