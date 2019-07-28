# `::fstrings`

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/danielhenrymantilla/fstrings-rs)
[![Latest version](https://img.shields.io/crates/v/fstrings.svg)](https://crates.io/crates/fstrings)
[![Documentation](https://docs.rs/fstrings/badge.svg)](https://docs.rs/fstrings)
[![License](https://img.shields.io/crates/l/fstrings.svg)](https://github.com/danielhenrymantilla/fstrings-rs/blob/master/LICENSE)

## Basic fstring interpolation in Rust

The interpolation works as follows:

 1. if the (template) string literal contains a named parameters
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

    // advanced_cases
    {
        // it remains compatible with classic formatting parameters
        assert_eq!(
            f!("{hi}, {name}!", hi = "Hello"),
            "Hello, World!",
        );

        // you can override / shadow the named arguments
        assert_eq!(
            f!("Hello, {name}!", name = "Earth"),
            "Hello, Earth!",
        );
    }
}
```
