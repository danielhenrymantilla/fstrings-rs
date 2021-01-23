#[macro_use]
extern crate fstrings;

fn main() {
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
        assert_eq!(f!("{hi}, {name}!", hi = "Hello"), "Hello, World!",);

        // You can override / shadow the named arguments
        assert_eq!(f!("Hello, {name}!", name = "Earth"), "Hello, Earth!",);

        // You can use field access (but no method calls!)
        let foo = Foo { name }; /* where */
        struct Foo<T> {
            name: T,
        }
        assert_eq!(f!("Hello, {foo.name}!"), "Hello, World!",);

        // This also works with tuple indexing.
        let ft_and_name = (42, name);
        assert_eq!(f!("Hello, {ft_and_name.1}!"), "Hello, World!",);

        // You can use fstrings to debug by appending a `=` after the
        // interpolated expression.
        let x = 0b_101010;
        assert_eq!(f!("In this context {x=}"), "In this context x = 42",);
    }
}
