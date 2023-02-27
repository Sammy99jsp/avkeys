//!
//! Macro to read [/usr/include/linux/input-event-codes.h]
//! 

use std::{fs::File, io::Read};

use proc_macro::{TokenStream, Span};
use quote::quote;
use regex::Regex;




/// Auto generate consts from [/usr/include/linux/input-event-codes.h]
#[proc_macro]
pub fn keycodes(tkn : TokenStream) -> TokenStream {
    let mut file = File::open("/usr/include/linux/input-event-codes.h")
        .expect("/usr/include/linux/input-event-codes.h not present!");

    let mut body = String::new();

    file.read_to_string(&mut body)
        .expect("Cannot read from /usr/include/linux/input-event-codes.h");

    let line_expr = Regex::new(r#"#define (KEY_[0-9A-Za-z_]+)\s*((0x\d+)|(\d+))"#)
        .unwrap();

    let definitions = body
        .lines()
        .filter_map(|ln| line_expr.captures(ln))
        .map(|captures| {
            if captures.get(2).map(|a| a.as_str()) == Some("") {
                panic!("{:?}", captures);
            }

            captures.get(1)
                .and_then(|ident| {
                    captures.get(2)
                        .map(|val| {
                            (
                                syn::Ident::new(ident.as_str(), Span::call_site().into()), 
                                syn::LitInt::new(val.as_str(), Span::call_site().into()),
                            )
                        })
                })
        })
        .map(|o| {
            o.map(|(ident, value)| {
                quote! {
                    pub const #ident : u32 = #value;
                }
            })
        });

    quote! {
        #(#definitions)*
    }.into()
}