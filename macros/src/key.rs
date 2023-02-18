use std::{collections::HashMap,};

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{LitInt, token::{Brace, Bracket}, parse::{Parse, ParseBuffer}, braced, bracketed, punctuated::Punctuated, Token, Signature, spanned::Spanned, PatType, TypePath, FnArg};

lazy_static! {
    pub static ref KEY_PARAMS : HashMap<&'static str, &'static str> = {
        HashMap::from_iter([
                ("d", "$crate::KeyParams::DigitKey"),
                ("f", "$crate::KeyParams::FunctionKey")
        ].into_iter())
    };
}
///
/// AvKey that is being parsed.
/// 
/// Can either be a Key Name, Key Code, or Key Parameter.
/// 
pub enum ParsedKey {
    Name(Span, String),
    Code(Bracket, LitInt),
    Parameter(Brace, syn::Ident)
} 


impl Parse for ParsedKey {

    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        
        if input.peek(syn::token::Bracket) {
            let inside;
            let brackets = bracketed!(inside in input);
            return Ok(Self::Code(brackets, inside.parse()
                .map_err(|err| syn::Error::new(err.span(), 
                    "Expected a key code here (any integer literal e.g. `11`, `124`)\n\
                    Full Example: `#[AvKeybind(Ctrl+Alt+[111])]`"
                ))?
            ));
        }

        if input.peek(syn::token::Brace) {
            let inside;
            let brace = braced!(inside in input);
            return Ok(
                Self::Parameter(
                    brace, 
                    inside.parse()
                        .map_err(|err| syn::Error::new(err.span(),
                            "Expected a key parameter here (e.g. `d`, `f`)\n\
                            Full Example: `#[AvKeybind(Logo+{d})]`")
                        )?
                )
            )
        }

        // Anything else, including LitInt since `1` is a valid keycode.

        if input.peek(syn::LitInt) {
            let int : LitInt = input.parse().unwrap();
            return Ok(Self::Name(int.span(), int.to_string()));
        }

        if input.peek(syn::Ident) {
            let ident : syn::Ident = input.parse().unwrap();
            return Ok(Self::Name(ident.span(), ident.to_string())); 
        }

        Err(
            input.error("Expected either a Name (`1`, `A`, `Delete`); Code (`[12]`, `[111]`); \
            Key Parameter (`{d}`, `{f}`).\nFull Example: `#[AvKeybind(Ctrl+[111]+{f})]`")
        )
    }
}

///
/// Parsed macro representation of AvKeybind.
/// 
pub struct ParsedKeybind(Punctuated<ParsedKey, Token![+]>);

impl ParsedKeybind {
    pub fn iter(&self) -> impl Iterator<Item = &ParsedKey> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = ParsedKey> {
        self.0.into_iter()
    }

    pub fn validate_parameter_names(&self) -> Option<TokenStream> {
        let mut possible_parameter_errors = self
            .iter()
            .filter(|k| matches!(k, ParsedKey::Parameter(_, _)))
            .filter_map(|k| match k {
                ParsedKey::Parameter(_, ident) => {
                    let p_type = ident.to_string();
                    if KEY_PARAMS.get(&p_type.as_str()).is_none() {
                        // No recognised key paramater by that identifier.
                        Some(syn::Error::new(ident.span(), format!("Unknown key parameter '{p_type}'.\nExpected one of: {}",
                            KEY_PARAMS.keys().map(|k| format!("`{}`, ", k)).collect::<String>()))
                        )
                    } else {
                        None
                    }
                },
                _ => None
            });

        let e = possible_parameter_errors.next();
        e.map(
            |mut e| {
                possible_parameter_errors.for_each( |err| e.extend(err));
                e
            }
        ).map(|e| e.into_compile_error().into())
    }

    pub fn parameters_present(&self) -> impl Iterator<Item = String>  +'_ {
        self.iter()
            .filter_map(|k| match k {
                ParsedKey::Parameter(b, p) => Some(p.to_string()),
                _ => None
            })
    }

    pub fn key_parameter_types_delcared_in_fn<'a>(&self, sig : &'a Signature) -> impl Iterator<Item = &'a TypePath> + 'a  {
        sig.inputs.iter()
            .filter_map(|param| match param{
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(ty) => match &*ty.ty {
                    syn::Type::Path(p) => Some(p),
                    _ => None,
                }
            })
    }

    pub fn generate_key_parameter_assignments<'a>(&self, sig : &'a Signature) -> Result<TokenStream, TokenStream>  {
        let v = sig.inputs.iter()
            .filter_map(|param| match param{
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(ty) => match &*ty.ty {
                    syn::Type::Path(p) => Some((ty, p)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        
        if v.len() == 0 {
            return Ok(quote! {}.into());
        }


        if v.len() > self.parameters_present().count() {
            if self.parameters_present().count() == 0 {
                return Err(
                    syn::Error::new(v[0].0.span(), "Unexpected extra function parameters.\nDid you forget to specify key parameters in #[AvKeybind(...)]?")
                        .into_compile_error().into()
                )
            }

            let mut iter = (&v[(v.len() - self.parameters_present().count())..])
                .iter()
                .map(|(p, t)| syn::Error::new(p.pat.span(), "Excess key parameter defined here.\nPlease remove it."));

            return Err(iter.next()
                .map(|mut err| {err.extend(iter); err})
                .unwrap().into_compile_error().into())
        }

        let iter_v = v.iter()
            .enumerate()
            .map(|(i, (arg, t))| {
                match &*arg.pat {
                    syn::Pat::Ident(ident) => Ok(ident),
                    _ => Err(
                        syn::Error::new(arg.pat.span(), "Expected identifier for key parameter name, try `key_param1` instead.")
                    ),
                }
            });

        // Last-minute errors
        if iter_v.clone()
            .any(|ref r| r.is_err())
        {
            let mut errs = iter_v
                .filter_map(Result::err);

            let err = errs.next()
                .map(|mut err|{ err.extend(errs); err })
                .unwrap();

            return Err(err.into_compile_error().into());
        }

        let iter_v = iter_v.filter_map(Result::ok)
            .enumerate()
            .map(|(i, a)| {
                let attrs = a.attrs.iter();
                quote! {
                    #(#attrs)*
                    let #a = __params__[#i];
                }
            });
        
        Ok(quote! {
            #(#iter_v)*
        }.into())
    }

    pub fn validate_func_sign_against_key_params(&self, sig : &Signature) -> Option<TokenStream> {

        let params = self.key_parameter_types_delcared_in_fn(sig).collect::<Vec<_>>();

        let results = self.parameters_present()
            .enumerate()
            .map(
                |(i, declared_param)|
                    params.get(i)
                        .and_then(|param_in_fn| 
                            param_in_fn.path.get_ident()
                        )
                        .map(|param| {
                            if param.to_string() == declared_param {
                                Ok(param)
                            } else {
                                Err(syn::Error::new(
                                    param.span(), 
                                    format!(
                                        "Expected key parameter `{declared_param}` here, got `{}`",
                                        param.to_string()
                                    )
                                ))
                            }
                        })
                        .unwrap_or(Err(
                            syn::Error::new(
                                sig.inputs.span(),
                                format!("Expected key parameter `{declared_param}` in function delcaration.\n\
                                Append `key_param{} : {declared_param}` to the end of the parameter list.", i + 1)
                            )
                        ))
            ).collect::<Vec<_>>();

        if results.iter().any(Result::is_err) {
            let mut r_iter = results.into_iter()
                .filter_map(|r| r.err());

            let err = r_iter.next()
                .map(|mut err| {err.extend(r_iter); err})
                .unwrap();

            return Some(err.into_compile_error().into());
        }

        None
    }
}

impl Parse for ParsedKeybind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(
            Punctuated::<ParsedKey, Token![+]>::parse_separated_nonempty(input)?
        ))
    }
}