# `::fstrings`

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/danielhenrymantilla/fstrings-rs)
[![Latest version](https://img.shields.io/crates/v/fstrings.svg)](https://crates.io/crates/fstrings)
[![Documentation](https://docs.rs/fstrings/badge.svg)](https://docs.rs/fstrings)
[![MSRV](https://img.shields.io/badge/MSRV-1.36.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![License](https://img.shields.io/crates/l/fstrings.svg)](https://github.com/danielhenrymantilla/fstrings-rs/blob/master/LICENSE)

  - Note, this crate has been rendered obsolete as of 1.58.0, wherein
    `let x = 42; println!("{x}");` acts as `let x = 42; println_f!("{x}");`
    here.

    A precursor/pioneer crate, for sure, and one which now has been subsumed
    by [RFC 2795](https://github.com/rust-lang/rfcs/pull/2795), which
    successfully championed the ideas behind this crate! 🏆

## Basic fstring interpolation in Rust

The interpolation works as follows:

 1. if the (template) string literal contains a named parameter
    (_e.g._ `{name}`)

 1. and no `name = value` argument is fed to the formatting call,

 1. then an automatic `name = name` argument is added, so that the variable is
    effectively interpolated from the current scope.

### Example

```rust
#[macro_use]
extern crate fstrings;

fn main ()
{
    let name = "World";

    // Usage is simple: just append `_f` to the name of any formatting macro
    println_f!("Hello, {name}!");

    assert_eq!(
        f!("Hello, {name}!"), // shorthand for String creation (Python-like)
        String::from("Hello, World!"),
    );

    // ## Advanced cases:
    {
        // It remains compatible with classic formatting parameters
        assert_eq!(
            f!("{hi}, {name}!", hi = "Hello"),
            "Hello, World!",
        );

        // You can override / shadow the named arguments
        assert_eq!(
            f!("Hello, {name}!", name = "Earth"),
            "Hello, Earth!",
        );

        // You can use field access (but no method calls!)
        let foo = Foo { name }; /* where */ struct Foo<T> { name: T }
        assert_eq!(
            f!("Hello, {foo.name}!"),
            "Hello, World!",
        );

        // This also works with tuple indexing.
        let ft_and_name = (42, name);
        assert_eq!(
            f!("Hello, {ft_and_name.1}!"),
            "Hello, World!",
        );

        // You can use fstrings to debug by appending a `=` after the
        // interpolated expression.
        let x = 0b_101010;
        assert_eq!(
            f!("In this context {x=}"),
            "In this context x = 42",
        );
    }
}
```
