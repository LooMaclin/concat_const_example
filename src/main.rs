#![feature(const_fn, const_fn_union, const_str_as_bytes, const_str_len, untagged_unions)]

#[macro_use]
extern crate const_concat;

const GREETING: &str = "Hello";
const PLACE: &str = "world";
const HELLO_WORLD: &str = const_concat!(GREETING, ", ", PLACE, "!");

fn main() {
    println!("{}", HELLO_WORLD);
}
