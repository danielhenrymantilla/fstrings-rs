#[cfg(not(feature = "verbose-expansions"))]
macro_rules! debug_input {($expr:expr) => ($expr)}

#[cfg(feature = "verbose-expansions")]
macro_rules! debug_input {($expr:expr) => (
    match $expr { expr => {
        eprintln!("-------------------\n{} ! ( {} )", FUNCTION_NAME, expr);
        expr
    }}
)}

#[cfg(not(feature = "verbose-expansions"))]
macro_rules! debug_output {($expr:expr) => ($expr)}

#[cfg(feature = "verbose-expansions")]
macro_rules! debug_output {($expr:expr) => (
    match $expr { expr => {
        eprintln!("=>\n{}\n-------------------\n", expr);
        expr
    }}
)}
