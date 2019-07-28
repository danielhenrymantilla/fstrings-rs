#![cfg_attr(feature = "nightly",
    feature(external_doc)
)]
#![cfg_attr(feature = "nightly",
    doc(include = "../README.md")
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "See [crates.io](https://crates.io/crates/fstrings)"
)]
#![cfg_attr(not(feature = "nightly"),
    doc = "for more info about this crate."
)]

#![no_std]

macro_rules! mk_macros {( @with_dollar![$dol:tt]=>
    $(
        #[doc = $doc_string:literal]
        $printlnf:ident
            => $println:ident!($($stream:ident,)? ...)
        ,
    )*
) => (
    $(
        #[doc = $doc_string]
        #[macro_export]
        macro_rules! $printlnf {(
            $($dol $stream : expr,)? $dol($dol args:tt)*
        ) => (
            $println!($($dol $stream,)? "{}", format_args_f!($dol($dol args)*))
        )}
    )*
)}

mk_macros! { @with_dollar![$]=>
    #[doc = "Like [`print!`](https://doc.rust-lang.org/std/macro.print.html), but with basic f-string interpolation."]
    print_f
        => print!(...)
    ,
    #[doc = "Like [`println!`](https://doc.rust-lang.org/std/macro.println.html), but with basic f-string interpolation."]
    println_f
        => println!(...)
    ,
    #[doc = "Like [`eprint!`](https://doc.rust-lang.org/std/macro.eprint.html), but with basic f-string interpolation."]
    eprint_f
        => eprint!(...)
    ,
    #[doc = "Like [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html), but with basic f-string interpolation."]
    eprintln_f
        => eprintln!(...)
    ,
    #[doc = "Like [`format!`](https://doc.rust-lang.org/std/macro.format.html), but with basic f-string interpolation."]
    format_f
        => format!(...)
    ,
    #[doc = "Shorthand for [`format_f`]."]
    f
        => format!(...)
    ,
    #[doc = "Like [`write!`](https://doc.rust-lang.org/std/macro.write.html), but with basic f-string interpolation."]
    write_f
        => write!(stream, ...)
    ,
    #[doc = "Like [`writeln!`](https://doc.rust-lang.org/std/macro.writeln.html), but with basic f-string interpolation."]
    writeln_f
        => writeln!(stream, ...)
    ,
}

/// Like [`format_args!`](https://doc.rust-lang.org/std/macro.format_args.html), but with basic f-string interpolation.
#[::proc_macro_hack::proc_macro_hack(fake_call_site)]
pub use proc_macro::format_args_f;
