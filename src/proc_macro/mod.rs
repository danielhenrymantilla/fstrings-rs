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

    debug_input!(&input);

    let Args {
        format_literal,
        mut extra_args,
    } = parse_macro_input!(input);
    let s = format_literal.value();

    let mut iterator = s.char_indices().peekable();
    while let Some((i, c)) = iterator.next() {
        if c != '{' {
            continue;
        }
        // encountered `{`, let's see if it was `{{`
        if let Some(&(_, '{')) = iterator.peek() {
            let _ = iterator.next();
            continue;
        }
        let s = &s[i + 1 ..];
        let end =
            s   .chars()
                .position(|c| c == '}' || c == ':')
                .expect(concat!(
                    "Invalid format string literal\n",
                    "note: if you intended to print `{`, ",
                    "you can escape it using `{{`",
                ))
        ;
        let arg = s[.. end].trim();
        let ident = match parse_str::<Ident>(arg) {
            | Ok(ident) => ident,
            | Err(_) => continue,
        };
        // if `ident = ...` is not yet among the extra args
        if  extra_args
                .iter()
                .all(|arg| Some(&ident) != arg.ident.as_ref())
        {
            extra_args.push(FmtArg {
                expr: parse_quote!(#ident),
                ident: Some(ident),
            });
        }
    }

    TokenStream::from(debug_output!(quote! {
        format_args!(#format_literal #(, #extra_args)*)
    }))
}
