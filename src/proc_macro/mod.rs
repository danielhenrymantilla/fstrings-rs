#![cfg_attr(debug_assertions,
    allow(unused_imports)
)]

extern crate proc_macro; use ::proc_macro::TokenStream;
use ::proc_quote::{
    quote,
    quote_spanned,
    ToTokens,
};
use proc_macro2::{
    Span,
    TokenStream as TokenStream2,
};
use ::syn::{*,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
};

#[macro_use]
mod macros;

enum FmtArg {
    Expr(Expr),
    IdentEqExpr {
        ident: Ident,
        expr: Expr,
    },
}

impl Parse for FmtArg {
    fn parse (input: ParseStream) -> Result<Self>
    {
        if input.peek(Ident) {
            let ident = input.parse().unwrap();
            let _: Token![=] = input.parse()?;
            let expr = input.parse()?;
            Ok(FmtArg::IdentEqExpr { ident, expr })
        } else {
            Ok(FmtArg::Expr(input.parse()?))
        }
    }
}

impl ToTokens for FmtArg {
    fn to_tokens (self: &'_ Self, out: &'_ mut TokenStream2)
    {
        out.extend(match self {//
            | &FmtArg::Expr(ref expr) => quote! {
                #expr
            },
            | &FmtArg::IdentEqExpr {
                ref ident,
                ref expr,
            } => quote! {
                #ident = #expr
            },
        });
    }
}

#[allow(dead_code)] // dumb compiler does not see the struct being used...
struct Args {
    f_string: bool,
    format_literal: LitStr,
    extra_args: Vec<FmtArg>,
}

impl Parse for Args {
    fn parse (input: ParseStream) -> Result<Self>
    {
        let f_string = {
            if let Some(letter) = input.parse::<Option<Ident>>()? {
                if letter != "f" {
                    return Err(Error::new(
                        letter.span(),
                        "Expected `f` or a string literal",
                    ));
                }
                true
            } else {
                false
            }
        };
        let format_literal = input.parse()?;
        let extra_args = if input.parse::<Option<Token![,]>>()?.is_some() {
            Punctuated::<FmtArg, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect()
        } else {
            Vec::new()
        };
        // if f_string {
        //     for arg in &extra_args {
        //         if let &FmtArg::IdentEqExpr { ref ident, .. } = arg {
        //             return Err(Error::new(
        //                 ident.span(),
        //                 "Cannot use named arguments with f-strings",
        //             ));
        //         }
        //     }
        // }
        Ok(Self {
            f_string,
            format_literal,
            extra_args,
        })
    }
}


macro_rules! mk_macros {(
    $(
        $ident:literal as $fident:ident
    ),* $(,)?
) => [$(
#[proc_macro] pub
fn $fident (input: TokenStream) -> TokenStream
{
    #[allow(unused)]
    const FUNCTION_NAME: &str = stringify!($fident);

    dbg_in!(&input);

    let Args {
        f_string: is_f_string,
        format_literal,
        mut extra_args,
    } = parse_macro_input!(input);
    if is_f_string {
        let s: &str = &*format_literal.value();
        let mut iterator = s.chars().enumerate().peekable();
        while let Some((i, c)) = iterator.next() {
            if c == '{' {
                if let Some(&(_, '{')) = iterator.peek() {
                    let _ = iterator.next();
                    continue;
                }
                let end =
                    s[i ..]
                        .chars()
                        .position(|c| c == '}' || c == ':')
                        .expect(concat!(
                            "Invalid format string literal\n",
                            "note: if you intended to print `{`, ",
                            "you can escape it using `{{`",
                        ))
                ;
                let arg = s[i + 1 .. end].trim();
                if arg.is_empty() {
                    continue;
                }
                let cur_ident: Ident = match parse_str(arg) {//
                    | Err(_) => continue,
                    | Ok(x) => x,
                };
                if extra_args
                        .iter()
                        .all(|x| match *x {//
                            | FmtArg::Expr(_) => {
                                true
                            },
                            | FmtArg::IdentEqExpr { ref ident, .. } => {
                                *ident != cur_ident
                            },
                        })
                {
                    extra_args.push(FmtArg::IdentEqExpr {
                        expr: parse_quote!(#cur_ident),
                        ident: cur_ident,
                    });
                }
            }
        }
    }

    let macro_name = Ident::new($ident, Span::call_site());

    TokenStream::from(dbg_out!(quote! {
        #macro_name!(#format_literal #(, #extra_args)*)
    }))
}
)*]}

mk_macros! {
    "print" as printf,
    "println" as printlnf,
    "eprint" as eprintf,
    "eprintln" as eprintlnf,
}

