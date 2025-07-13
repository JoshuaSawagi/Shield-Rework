#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod common;

#[skyline::main(name = "shieldstop")]
pub fn main() {
    common::install();
}