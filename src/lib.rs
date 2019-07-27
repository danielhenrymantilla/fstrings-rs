// #[::proc_macro_hack::proc_macro_hack]
pub use proc_macro::{*
    // fformat_args,
};

// macro_rules! mk_macros {( [$dol:tt]
//     $(
//         $ident:ident as $fident:ident
//     ),* $(,)?
// ) => (
//     $(
//         #[macro_export]
//         macro_rules! $fident {(
//             $dol($dol args:tt)*
//         ) => (
//             $ident!("{}", fformat_args!($dol($dol args)*))
//         )}
//     )*
// )}

// mk_macros! { [$]
//     print as fprint,
//     println as fprintln,
//     eprint as efprint,
//     eprintln as efprintln,
// }
