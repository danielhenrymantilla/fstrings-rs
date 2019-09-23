extern crate proc_macro; use ::proc_macro::TokenStream;
use ::proc_quote::{
    quote,
    ToTokens,
};
use ::proc_macro2::{
    TokenStream as TokenStream2,
};
use ::syn::{*,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
};
use ::std::ops::Not;
use regex::Regex;

#[macro_use]
mod macros;

struct FmtArg {
    ident: Option<Ident>,
    expr: Expr,
}

impl Parse for FmtArg {
    fn parse (input: ParseStream) -> Result<Self>
    {
        Ok(FmtArg {
            ident: {
                if  input.peek(Ident) &&
                    input.peek2(Token![=]) &&
                    input.peek3(Token![=]).not()
                {
                    let ident: Ident = input.parse().unwrap();
                    let _: Token![=] = input.parse().unwrap();
                    Some(ident)
                } else {
                    None
                }
            },
            expr: input.parse()?,
        })
    }
}

impl ToTokens for FmtArg {
    fn to_tokens (self: &'_ Self, out: &'_ mut TokenStream2)
    {
        if let Some(ref ident) = self.ident {
            out.extend(quote! { #ident = });
        }
        self.expr.to_tokens(out);
    }
}

#[allow(dead_code)] // dumb compiler does not see the struct being used...
struct Args {
    format_literal: LitStr,
    extra_args: Vec<FmtArg>,
}

impl Parse for Args {
    fn parse (input: ParseStream) -> Result<Self>
    {
        let format_literal = input.parse()?;
        let extra_args =
            if input.parse::<Option<Token![,]>>()?.is_some() {
                Punctuated::<FmtArg, Token![,]>::parse_terminated(input)?
                    .into_iter()
                    .collect()
            } else {
                Vec::new()
            }
        ;
        Ok(Self {
            format_literal,
            extra_args,
        })
    }
}

#[::proc_macro_hack::proc_macro_hack] pub
fn format_args_f (input: TokenStream) -> TokenStream
{
    #[allow(unused)]
    const FUNCTION_NAME: &str = "format_args_f";
    let mut expr_cnt = 0;

    debug_input!(&input);
    let Args {
        mut format_literal,
        mut extra_args,
    } = parse_macro_input!(input);
    let s = format_literal.value();
    let mut iterator = s.chars().peekable();
    let mut curly_bracket_count = 0;
    let mut frmt = String::new();
    let mut item = String::new();

    let re_fmt = Regex::new(r":([xX]?\?|[oxXpbeE])?(\d+)?$").unwrap();
    // identify any trailing formatting traits
    // see: https://doc.rust-lang.org/std/fmt/#formatting-traits
    // ? ⇒ Debug
    // x? ⇒ Debug with lower-case hexadecimal integers
    // X? ⇒ Debug with upper-case hexadecimal integers
    // o ⇒ Octal
    // x ⇒ LowerHex
    // X ⇒ UpperHex
    // p ⇒ Pointer
    // b ⇒ Binary
    // e ⇒ LowerExp
    // E ⇒ UpperExp

    frmt.push('"');
    while let Some(c) = iterator.next() {
        if c != '{' {
            frmt.push(c);
            continue;
        } else {
            // encountered `{`, let's see if it was `{{`
            if let Some(&'{') = iterator.peek() {
                let _ = iterator.next();
                frmt.push('{');
                continue;
            }
            curly_bracket_count += 1;
            while let Some(c) = iterator.next() {
                if c == '{' {
                    curly_bracket_count += 1;
                    item.push(c);
                    continue;
                } else if c == '}' {
                    curly_bracket_count -= 1;
                    if curly_bracket_count == 0 {
                        let s = item.as_str();
                        let (mut arg, fmt) =
                            if let Some(fmt_match) = re_fmt.find(s)
                        {
                            (s[.. fmt_match.start()].trim(),
                             Some(fmt_match.as_str()))
                        } else {
                            (s.trim(),
                             None)
                        };
                        let trailing_eq = arg.ends_with('=');
                        if trailing_eq {
                            arg = &arg[.. arg.len() - 1];
                        }
                        let exp = match parse_str::<Expr>(arg) {
                            | Ok(expr) => expr,
                            | Err(_) => continue,
                        };
                        let id = format!("expr_{}_", expr_cnt);
                        expr_cnt += 1;
                        extra_args.push(FmtArg {
                            expr: parse_quote!(#exp),
                            ident: Some(parse_str::<Ident>(&id)
                                        .unwrap())
                        });
                        if trailing_eq {
                            frmt.push_str(arg);
                            frmt.push('=');
                        }
                        frmt.push('{');
                        frmt.push_str(id.as_str());
                        if let Some(m) = fmt {
                            frmt.push_str(m);
                        }
                        frmt.push('}');
                        item.clear();
                        break;
                    } else {
                        item.push(c);
                        continue;
                    }
                } else {
                    item.push(c);
                    continue;
                }
            }
        }
    }

    frmt.push('"');
    format_literal = parse_str::<LitStr>(frmt.as_str()).unwrap();

    TokenStream::from(debug_output!(quote! {
        format_args!(#format_literal #(, #extra_args)*)
    }))
}
