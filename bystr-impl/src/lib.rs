//
// Copyright (c) Zach Marcantel. All rights reserved.
// Licensed under the GPLv3. See LICENSE file in the project root for full license information.
//

extern crate proc_macro;
extern crate proc_macro_hack;
use proc_macro_hack::proc_macro_hack;

#[macro_use]
extern crate quote;

use std::iter::FromIterator;

fn stringify(input: &syn::parse::ParseStream) -> syn::parse::Result<(String, syn::export::Span)> {
    if input.peek(syn::LitStr) {
        let val: syn::LitStr = input.parse()?;
        Ok((val.value(), val.span()))
    } else if input.peek(syn::Ident) {
        let val: syn::Ident = input.parse()?;
        Ok((val.to_string(), val.span()))
    } else {
        return Err(input.error("unexpected type for string-value"));
    }
}

struct StaticStr {
    len: usize,
    value: String,
    span: syn::export::Span,
}

impl syn::parse::Parse for StaticStr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut len: Option<(usize, syn::export::Span)> = None;

        // see if we have a length as first-arg
        if input.peek(syn::LitInt) {
            let parsed: syn::LitInt = input.parse()?;
            len = Some((parsed.base10_parse()?, parsed.span()));

            let _: syn::token::Comma = input.parse()?;
        }

        let (raw_str, span) = stringify(&input)?;

        // set `len` based on whether it was the first arg or not
        match len {
            None => {
                len = Some((raw_str.as_bytes().len() + 1, span));
            }
            Some((ref l, sp)) => {
                if *l <= raw_str.as_bytes().len() {
                    return Err(syn::parse::Error::new(
                        sp,
                        format!("defined string length {} is shorter than given string ({} + null-byte)",
                            l, raw_str.as_bytes().len())
                    ));
                }
            }
        }

        Ok(StaticStr{
            len: len.unwrap().0,
            value: raw_str,
            span: span,
        })
    }
}


fn byte_to_expr(b: u8, span: syn::export::Span) -> syn::Expr {
    let as_str = format!("{:#X}", b);
    syn::Expr::Lit(syn::ExprLit{
        attrs: vec!(),
        lit: syn::Lit::Int(syn::LitInt::new(&as_str, span)),
    })
}

#[proc_macro_hack]
pub fn bystr(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as StaticStr);

    let strbytes = input.value.as_bytes();
    let strlen = strbytes.len();

    let mut elems = strbytes.iter()
        .map(|b| byte_to_expr(*b, input.span))
        .collect::<Vec<syn::Expr>>();

    for _ in 0..(input.len - strlen) {
        elems.push(byte_to_expr(0, input.span));
    }

    let expr = syn::ExprArray{
        attrs: vec!(),
        bracket_token: syn::token::Bracket::default(),
        elems: syn::punctuated::Punctuated::from_iter(elems.into_iter()),
    };

    let result = quote! {
        #expr
    };

    result.into()
}
