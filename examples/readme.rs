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
