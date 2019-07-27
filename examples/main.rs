#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate fstrings;

fn main ()
{
    let x = 42;
    eprintlnf!(f"{x: ^10}");
}
