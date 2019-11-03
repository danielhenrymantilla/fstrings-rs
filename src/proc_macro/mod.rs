extern crate proc_macro;
use ::proc_macro::TokenStream;
use ::quote::{
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

#[allow(dead_code)] // dumb compiler does not see the struct being used...
struct Input {
    format_literal: LitStr,
    positional_args: Vec<Expr>,
    named_args: Vec<(Ident, Expr)>,
}

impl Parse for Input {
    fn parse (input: ParseStream) -> Result<Self>
    {
        let format_literal = input.parse()?;
        let mut positional_args = vec![];
        loop {
            if input.parse::<Option<Token![,]>>()?.is_none() {
                return Ok(Self {
                    format_literal,
                    positional_args,
                    named_args: vec![],
                });
            }
            if  input.peek(Ident) &&
                input.peek2(Token![=]) &&
                input.peek3(Token![=]).not()
            {
                // Found a positional parameter
                break;
            }
            positional_args.push(input.parse()?);
        }
        let named_args =
            Punctuated::<_, Token![,]>::parse_terminated_with(
                input,
                |input| Ok({
                    let name: Ident = input.parse()?;
                    let _: Token![=] = input.parse()?;
                    let expr: Expr = input.parse()?;
                    (name, expr)
                }),
            )?
            .into_iter()
            .collect()
        ;
        Ok(Self {
            format_literal,
            positional_args,
            named_args,
        })
    }
}

#[::proc_macro_hack::proc_macro_hack] pub
fn format_args_f (input: TokenStream) -> TokenStream
{
    #[allow(unused)]
    const FUNCTION_NAME: &str = "format_args_f";

    debug_input!(&input);

    let Input {
        mut format_literal,
        mut positional_args,
        mut named_args,
    } = parse_macro_input!(input);

    let s = format_literal.value();
    let ref mut out_format_literal = String::with_capacity(s.len());

    let mut iterator = s.char_indices().peekable();
    while let Some((i, c)) = iterator.next() {
        out_format_literal.push(c);
        if c != '{' {
            continue;
        }
        // encountered `{`, let's see if it was `{{`
        if let Some(&(_, '{')) = iterator.peek() {
            let _ = iterator.next();
            out_format_literal.push_str("{{");
            continue;
        }
        let (end, colon_or_closing_brace) =
            iterator
                .find(|&(_, c)| c == '}' || c == ':')
                .expect(concat!(
                    "Invalid format string literal\n",
                    "note: if you intended to print `{`, ",
                    "you can escape it using `{{`",
                ))
        ;
        // We use defer to ensure all the `continue`s append the closing char.
        let mut out_format_literal = defer(
            &mut *out_format_literal,
            |it| it.push(colon_or_closing_brace),
        );
        let out_format_literal: &mut String = &mut *out_format_literal;
        let mut arg = s[i + 1 .. end].trim();
        if let Some("=") = arg.get(arg.len().saturating_sub(1) ..) {
            assert_eq!(
                out_format_literal.pop(),  // Remove the opening brace
                Some('{'),
            );
            arg = &arg[.. arg.len() - 1];
            out_format_literal.push_str(arg);
            out_format_literal.push_str(" = {");
        }
        if arg.is_empty() {
            continue;
        }

        enum Segment { Ident(Ident), LitInt(LitInt) }
        let segments: Vec<Segment> = {
            impl Parse for Segment {
                fn parse (input: ParseStream<'_>)
                  -> Result<Self>
                {
                    let lookahead = input.lookahead1();
                    if lookahead.peek(Ident) {
                        input.parse().map(Segment::Ident)
                    } else if lookahead.peek(LitInt) {
                        input.parse().map(Segment::LitInt)
                    } else {
                        Err(lookahead.error())
                    }
                }
            }
            match ::syn::parse::Parser::parse_str(
                Punctuated::<Segment, Token![.]>::parse_separated_nonempty,
                arg,
            )
            {
                | Ok(segments) => segments.into_iter().collect(),
                | Err(err) => return err.to_compile_error().into(),
            }
        };
        match segments.len() {
            | 0 => unreachable!("`parse_separated_nonempty` returned empty"),
            | 1 => {
                out_format_literal.push_str(arg);
                match {segments}.pop().unwrap() {
                    | Segment::LitInt(_) => {
                        // found something like `{0}`, let `format_args!`
                        // handle it.
                        continue;
                    },
                    | Segment::Ident(ident) => {
                        // if `ident = ...` is not yet among the extra args
                        if  named_args
                                .iter()
                                .all(|(it, _)| *it != ident)
                        {
                            named_args.push((
                                ident.clone(),
                                parse_quote!(#ident), // Expr
                            ));
                        }
                    },
                }
            },
            | _ => {
                ::std::fmt::Write::write_fmt(
                    out_format_literal,
                    format_args!("{}", positional_args.len()),
                ).expect("`usize` or `char` Display impl cannot panic");
                let segments: Punctuated<TokenStream2, Token![.]> =
                    segments
                        .into_iter()
                        .map(|it| match it {
                            | Segment::Ident(ident) => {
                                ident.into_token_stream()
                            },
                            | Segment::LitInt(literal) => {
                                literal.into_token_stream()
                            },
                        })
                        .collect()
                ;
                positional_args.push(parse_quote! {
                    #segments
                })
            }
        }
    }

    let named_args =
        named_args
            .into_iter()
            .map(|(ident, expr)| quote! {
                #ident = #expr
            })
    ;
    format_literal = LitStr::new(
        out_format_literal,
        format_literal.span(),
    );
    TokenStream::from(debug_output!(quote! {
        format_args!(
            #format_literal
            #(, #positional_args)*
            #(, #named_args)*
        )
    }))
}

fn defer<'a, T : 'a, Drop : 'a> (x: T, drop: Drop)
  -> impl ::core::ops::DerefMut<Target = T> + 'a
where
    Drop : FnOnce(T),
{
    use ::core::mem::ManuallyDrop;
    struct Ret<T, Drop> (
        ManuallyDrop<T>,
        ManuallyDrop<Drop>,
    )
    where
        Drop : FnOnce(T),
    ;
    impl<T, Drop> ::core::ops::Drop for Ret<T, Drop>
    where
        Drop : FnOnce(T),
    {
        fn drop (self: &'_ mut Self)
        {
            use ::core::ptr;
            unsafe {
                // # Safety
                //
                //   - This is the canonical example of using `ManuallyDrop`.
                let value = ManuallyDrop::into_inner(ptr::read(&mut self.0));
                let drop = ManuallyDrop::into_inner(ptr::read(&mut self.1));
                drop(value);
            }
        }
    }
    impl<T, Drop> ::core::ops::Deref for Ret<T, Drop>
    where
        Drop : FnOnce(T),
    {
        type Target = T;
        #[inline]
        fn deref (self: &'_ Self)
          -> &'_ Self::Target
        {
            &self.0
        }
    }
    impl<T, Drop> ::core::ops::DerefMut for Ret<T, Drop>
    where
        Drop : FnOnce(T),
    {
        #[inline]
        fn deref_mut (self: &'_ mut Self)
          -> &'_ mut Self::Target
        {
            &mut self.0
        }
    }
    Ret(ManuallyDrop::new(x), ManuallyDrop::new(drop))
}
