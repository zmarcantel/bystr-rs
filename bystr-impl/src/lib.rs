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

struct StaticStr {
    len: usize,
    value: syn::LitStr,
}

impl syn::parse::Parse for StaticStr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut len: Option<(usize, syn::export::Span)> = None;

        if input.peek(syn::LitInt) {
            let parsed: syn::LitInt = input.parse()?;
            len = Some((parsed.base10_parse()?, parsed.span()));

            let _: syn::token::Comma = input.parse()?;
        }

        let val: syn::LitStr = input.parse()?;
        match len {
            None => {
                len = Some((val.value().as_bytes().len() + 1, val.span()));
            }
            Some((ref l, sp)) => {
                if *l <= val.value().as_bytes().len() {
                    return Err(syn::parse::Error::new(
                        sp,
                        format!("defined string length {} is shorter than given string ({} + null-byte)",
                            l, val.value().as_bytes().len())
                    ));
                }
            }
        }

        Ok(StaticStr{
            len: len.unwrap().0,
            value: val,
        })
    }
}


fn byte_to_expr(b: u8, span: syn::export::Span) -> syn::Expr {
    syn::Expr::Lit(syn::ExprLit{
        attrs: vec!(),
        lit: syn::Lit::Byte(syn::LitByte::new(b, span)),
    })
}

#[proc_macro_hack]
pub fn bystr(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as StaticStr);

    let strval = input.value.value();
    let strbytes = strval.as_bytes();
    let strlen = strbytes.len();

    let mut elems = strbytes.iter()
        .map(|b| byte_to_expr(*b, input.value.span()))
        .collect::<Vec<syn::Expr>>();

    for _ in 0..(input.len - strlen) {
        elems.push(byte_to_expr(0, input.value.span()));
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
