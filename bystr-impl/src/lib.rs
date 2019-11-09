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

fn byte_to_expr(b: u8, span: syn::export::Span) -> syn::Expr {
    syn::Expr::Lit(syn::ExprLit{
        attrs: vec!(),
        lit: syn::Lit::Byte(syn::LitByte::new(b, span)),
    })
}

#[proc_macro_hack]
pub fn bystr(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::LitStr);

    let mut elems = input.value().as_bytes().iter()
        .map(|b| byte_to_expr(*b, input.span()))
        .collect::<Vec<syn::Expr>>();
    elems.push(byte_to_expr(0, input.span()));

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
