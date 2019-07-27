#[cfg(not(feature = "verbose-expansions"))]
macro_rules! dbg_in {($expr:expr) => ($expr)}

#[cfg(feature = "verbose-expansions")]
macro_rules! dbg_in {($expr:expr) => (
    match $expr { expr => {
        eprintln!("{}!( {} )", FUNCTION_NAME, expr);
        expr
    }}
)}

#[cfg(not(feature = "verbose-expansions"))]
macro_rules! dbg_out {($expr:expr) => ($expr)}

#[cfg(feature = "verbose-expansions")]
macro_rules! dbg_out {($expr:expr) => (
    match $expr { expr => {
        eprintln!("=>\n{}", expr);
        expr
    }}
)}
