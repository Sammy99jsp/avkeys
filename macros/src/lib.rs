//!
//! Macros for defining keyboard shortcuts,
//! and syntax for keyboard keys/keybinds.
//!
//! A part of the [AvdanOS Project](https://avdanos.org).
//!

#![feature(proc_macro_diagnostic, iter_intersperse, proc_macro_span)]

mod key;
mod keycode;

use std::{slice, iter::empty};

use avkeys_common::{AvKeyDiscrim};

use convert_case::Casing;
use key::{ParsedKey, ParsedKeybind, KEY_PARAMS};
use keycode::{ParseKeyCodeDefinition, KeyIdentifier, KeyCodesCollection};
use proc_macro::{Diagnostic, Level, TokenStream};
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, ItemFn, punctuated::Punctuated, Token, parse_macro_input, parse::Parse, token::Brace, braced};

// FIXME(Sammy99jsp) Broken link - the Wiki page below (Key Name Aliases) does not exist yet!

///
/// ## \#\[AvKeybind\]
/// Helper macro for generating a `AvKeybind` implementation.
/// Generates a wrapper struct for a callback function,
/// which will handle (de)serialization, a user-overrided combination, etc.
///
/// ### Syntax
/// The `#[AvKeybind]` macro itself accepts a `+`-separated list of
/// key specifiers.
///
/// This could include:
/// * Key Names
/// * Linux Keycodes
/// * Key Parameters
///
/// #### Key Names: `A`, `Z`, `1`, `LCtrl`
/// These are based off the Linux input headers.
///
/// These names can be found
/// * [/usr/include/linux/input-event-codes.h]
/// * [mirror @ FreeDesktop.org](https://gitlab.freedesktop.org/libinput/libinput/-/blob/main/include/linux/linux/input-event-codes.h).
///
/// All keys should use PascalCase (uppercase camelCase)
/// for key names: e.g. "Ctrl" over "CTRL" and/or "ctrl".
///
/// ##### Aliases
/// 
/// The AvdanOS team may also add additional aliases (e.g. `Ctrl` -> `LCtrl`) for ease-of-use.
/// Check [the wiki]() for an updating list of these aliases.
///
/// Some of these aliases may be punctuation,
/// so to use them from a macro context, escape them by
/// putting them in character literals : `':'`, `'\\'`, `','`, `'#'`
/// 
/// | **Example** | `Ctrl+Alt+Del` |
/// |-------------|----------------|
/// |             |                |
///
/// #### Linux Key Codes: `[16]`, `[63]`
/// These can be found in the Linux headers [/usr/include/linux/input-event-codes.h].
///
/// These must be surrounded by \[square\] brackets.
///
/// | **Example** | `Ctrl+Alt+[111]` |
/// |-------------|---------------------|
/// |             |                     |
///
/// #### Key Paramaters: `{d}`, `{f}`
/// Key parameters allow for numerous similar key combinations to have a shared action.
///
/// For example, `Ctrl+1` to `Ctrl+9` could switch the active tab to `1` to `9`, depending
/// on the number key the user pressed. Instead of defining 9 (or 10) separate combinations,
/// we can define one combination, `Ctrl+{d}` where `{d}` represents any digit key.
///
/// Look at `AvKeyParameter` for more information on key parameters.  
///
/// In the callback function, you can optionally add this key parameter into the callback
/// function, using the example syntax below.
///
/// **Example**
///
/// ```ignore
/// ///
/// /// Pet the cute kitty-cat `times` amount of times.
/// ///
/// #[AvKeybind(Ctrl+Shift+{d})]
/// pub fn PetKitty(state : &mut (...), times : d) {
///     for i in 1..(times.value()) {
///         println!("Petted the kitty: {times}");
///     }
/// }
/// ```
///
///
/// ### Full Example
/// ```ignore
/// use av_macros::AvKeybind;
/// use Avdan::AvKeybind;
///
/// ///
/// /// Opens a spotlight-search inspired prompt.
/// ///
/// #[AvKeybind(Logo+Space)]
/// pub fn AvSearch(state : &mut ()) {
///     todo!("Make the best search app, ever.")
/// }
/// ```
///
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn AvKeybind(attrs: TokenStream, body: TokenStream) -> TokenStream {
    // 1. Parse Default Keybind

    let keybind: ParsedKeybind = match syn::parse(attrs).map_err(|err| {
        syn::Error::new(
            err.span(),
            "Expected a + seperated non-trailing list of keys here:\n\
                Full Example: #[AvKeybind(Ctrl+[111]+{d})]",
        )
    }) {
        Ok(v) => v,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    // 2. Parse Implementation function.

    let func: ItemFn = match syn::parse(body).map_err(|err| {
        syn::Error::new(
            err.span(),
            "Expected a function declaration here.\n\
                Full Example:\n\n\
                #[AvKeybind(Ctrl+Alt+Del)]\n\
                pub fn PowerOptions(state: &mut (...)) {\n\
                \u{202f}   todo!(\"Whatever!\")\n\
                }",
        )
    }) {
        Ok(v) => v,
        Err(err) => return err.into_compile_error().into(),
    };

    // 2a. Enforce public access
    if !matches!(func.vis, syn::Visibility::Public(_)) {
        Diagnostic::spanned(
            func.vis.span().unwrap(),
            Level::Error,
            "Keybinds must be declared `pub`",
        )
        .emit();

        return quote! {}.into();
    }

    // 2b. Validate callback signature.

    // 2b (i) Validate Key Parameter Names
    match keybind.validate_parameter_names() {
        Some(err) => return err,
        None => {}
    };

    // Key Parameters in the Function Delcaration

    // 2b (ii) Validate Key Parameters are in function signature
    //         Ensure all key parameters are present in the function signature.
    match keybind.validate_func_sign_against_key_params(&func.sig) {
        Some(err) => return err,
        None => {}
    };

    // 3. Generate stuff

    // 3a. Generate key parameter bindings
    let pre_assignments: proc_macro2::TokenStream =
        match keybind.generate_key_parameter_assignments(&func.sig) {
            Err(err) => return err,
            Ok(stuff) => stuff,
        }
        .into();

    // 3b.
    // Keep original rustdoc comments by
    //      also relaying the original attributes macros (aka `#[doc = "..."]`).
    let attrs = func.attrs.iter();
    let params = func.sig.inputs.iter();
    let keybind_name = func.sig.ident;

    let default_keys = keybind.iter()
        .map(ParsedKey::to_lookup)
        .collect::<Vec<_>>();

    let default_keys_count = default_keys.len();

    let body = func.block;

    // FIXME(Sammy99jsp):   Auto-suggestions do not always behave
    //                      well inside this macro -- although this
    //                      could be a wider issue with proc-macros in general.
    let body = body.stmts.iter().map(|n| {
        let span = n.span();

        quote_spanned! {
            span => #n
        }
    });

    let vis = func.vis;

    let keybind_default_const = keybind_name.to_string()
        .to_case(convert_case::Case::ScreamingSnake) + "_CONST";

    let keybind_default_const = syn::Ident::new(&keybind_default_const, Span::call_site());

    quote! {
        #(#attrs)*
        #vis struct #keybind_name(Option<Vec< ::avkeys_common::AvKey >>);

        const #keybind_default_const : [::avkeys_common::AvKey ; #default_keys_count]= [
            #(#default_keys),*
        ];

        impl AvKeybind for #keybind_name {
            fn default_keys() -> &'static [::avkeys_common::AvKey]
                where Self : Sized
            {
                &#keybind_default_const
            }

            fn keys(&self) -> &[::avkeys_common::AvKey] {
                self.0.as_ref()
                    .map(|v| v.as_slice())
                    .unwrap_or(Self::default_keys())
            }

            fn run(&self, state : &mut (), __params__ : Vec<usize>) {
                #pre_assignments
                ::std::mem::drop(__params__);
                #(#body)*
            }
        }
    }
    .into()
}

///
/// ## keycodes!
/// 
/// Generates an enum of keycodes,
/// and some matching/parsing functions.
/// 
/// ### Syntax &mdash; Keycode Definition
/// The `keycodes!` macro takes a collection of
/// keycode definitions, which are in the folowing format:
/// 
/// #### Format
/// `<PrimaryName> => <code> [ match [<KeyDiscrim>, ...] ]`
/// 
/// | Token         | Description                      | Example |
/// | :---          | :-----------------------------   | :---    |
/// | `PrimaryName` | The main name used for this key &mdash; it should be [identifier-safe](https://doc.rust-lang.org/reference/identifiers.html).<br><br>It is recommended to use similar naming to [/usr/include/linux/input-event-codes.h].    | `LeftCtrl` |
/// | `code`        | The linux keycode associated with this key.<br>See [/usr/include/linux/input-event-codes.h] for more more info. | `23`, `0x67` |
/// | `match` *(Optional)* | Use this `match` keyword in conjunction with the alias array to define aliases for this key. |  |
/// | `KeyDiscrim` *(Optional)* | Any of: a char literal; an integer literal; an identifier. Adding a char or int literal will add a case to the `TryFrom` of this enum | `';'`, `','`, `12`, `0x56`, `Ident` |
/// 
/// #### Rustdoc
/// This macro will auto-generate code documentation for
/// any key aliases.
/// 
/// Any original `///` rustdoc comments will be kept.
/// 
/// In the future, this macro may also export these alias definitions
/// to an external file for auto-gen'd end user documentation.
/// 
/// #### Example Definitions
/// * `Escape => 1 match [Esc, ]`
/// * `Digit1 => 2 match [1, '1', ]`
/// * `Digit0 => 10`
/// 
/// ### Example
/// ```ignore
/// keycodes! {
///     Escape => 1 match [Esc, ]
///     Digit1 => 2 match [1, '1'],
///     Digit0 => 10,
/// }
/// ```
#[proc_macro]
pub fn keycodes(body : TokenStream) -> TokenStream {
    
    let aliases : KeyCodesCollection = match syn::parse(body) {
        Ok(r) => r,
        Err(_) => return quote!{
            compile_error!("Expected match syntax:\n{\n    Key1 => 2,\n    // ...\n}")
        }.into()
    };

    let definitions = aliases
        .iter()
        .flat_map(|k| {
            let code = k.code();
            let pri = k.primary();
            
            k.aliases().map(|alias| {
                match alias {
                    KeyIdentifier::Ident(alias_ident) => {
                        // If primary alias...
                        let attrs = k.attrs();
                        let spacing = (k.attrs().count() > 0)
                            .then(|| quote! { #[doc = "***"] })
                            .unwrap_or_default();

                        if alias_ident.to_string() == pri.to_string() {
                            let doc_comment = if k.aliases().count() > 1 {
                                format!("Aliases ({}): {}", k.aliases().count() -1, k.aliases().filter(|al| al.to_string() != pri.to_string()).map(|al| format!("`{}`", al.to_string()))
                                    .intersperse(",".to_string()).collect::<String>())
                            } else {
                                "".to_string()
                            };

                            return quote! {
                                #(#attrs)*
                                #spacing
                                #[doc = #doc_comment]
                                #alias_ident,
                            }
                        } 

                        let doc_comment = format!("Alias of `{}`", pri.to_string());

                        quote! {
                            #(#attrs)*
                            #spacing
                            #[doc = #doc_comment]
                            #alias_ident,
                        }
                        
                    },
                    KeyIdentifier::LitInt(i) => {
                        quote! {}
                    },
                    KeyIdentifier::LitChar(char) => {
                        quote! {}
                    },
                }
            })
        });
    
    let lookup_str = aliases
        .iter()
        .flat_map(|k| {
            let p = match k.primary() {
                KeyIdentifier::LitInt(i) => {
                    i.span().unwrap().error("Expected an identifier here").emit();
                    panic!();
                },
                KeyIdentifier::Ident(iden) => iden,
                KeyIdentifier::LitChar(c) => {
                    c.span().unwrap().error("Expected an identifier here").emit();
                    panic!();
                },
            };
            
            k.aliases()
                .map(move |a| {
                    let (s, span) = match a {
                        KeyIdentifier::LitInt(i) => (i.to_string(), i.span()),
                        KeyIdentifier::Ident(ident) => (ident.to_string(), ident.span()),
                        KeyIdentifier::LitChar(s) => (s.value().to_string(), s.span()),
                    };

                    let raw_byte_str = syn::LitByteStr::new(s.as_bytes(), span);

                    quote! {
                        #raw_byte_str => Some(Self::#p),
                    }
                })
        });
    
    let lookup_ints = aliases
        .iter()
        .flat_map(|k| {
            let p = match k.primary() {
                KeyIdentifier::LitInt(i) => {
                    i.span().unwrap().error("Expected an identifier here").emit();
                    panic!();
                },
                KeyIdentifier::Ident(iden) => iden,
                KeyIdentifier::LitChar(c) => {
                    c.span().unwrap().error("Expected an identifier here").emit();
                    panic!();
                },
            };

            k.aliases()
                .map(move |a| match a {
                    KeyIdentifier::LitInt(i) => Some((i, p)),
                    KeyIdentifier::Ident(_) => None,
                    KeyIdentifier::LitChar(_) => None,
                })
        })
        .filter_map(|k| k)
        .map(|(i, p)| {
            quote! {
                #i => Some(Self::#p),
            }
        });

    let lookup_chars = aliases
        .iter()
        .flat_map(|k| {
            let p = match k.primary() {
                KeyIdentifier::LitInt(i) => {
                    i.span().unwrap().error("Expected an identifier here").emit();
                    panic!();
                },
                KeyIdentifier::Ident(iden) => iden,
                KeyIdentifier::LitChar(c) => {
                    c.span().unwrap().error("Expected an identifier here").emit();
                    panic!();
                },
            };

            k.aliases()
                .map(move |a| match a {
                    KeyIdentifier::LitInt(_) => None,
                    KeyIdentifier::Ident(_) => None,
                    KeyIdentifier::LitChar(c) => Some((c, p)),
                })
        })
        .filter_map(|k| k)
        .map(|(c, p)| {
            quote! {
                #c => Some(Self::#p),
            }
        });

    let ident_lookups = aliases
        .iter()
        .flat_map(|k| {
            let p = k.code();
            k.aliases()
                .filter_map(move |a| match a {
                    KeyIdentifier::Ident(ident) => Some((p, ident)),
                    _ => None,
                })
        })
        .map(|(code, ident)| {
            quote! {
                Key::#ident => #code
            }
        });

    quote! {
        #[derive(Debug, Clone, Copy)]
        pub enum Key {
            #(#definitions)*
        }

        impl Key {
            const fn lookup_const<I : ~const Into< ::avkeys_common::AvKeyDiscrim >>(a : I) -> Option<Self> {
                let a : avkeys_common::AvKeyDiscrim = a.into();
            
                match a {
                    ::avkeys_common::AvKeyDiscrim::Str(s) => match s.as_bytes() {
                        #(#lookup_str)*
                        _ => None
                    },
                    ::avkeys_common::AvKeyDiscrim::Int(i) => match i {
                        #(#lookup_ints)*
                        _ => None
                    },
                    ::avkeys_common::AvKeyDiscrim::Char(c) => match c {
                        #(#lookup_chars)*
                        _ => None
                    }
                }
            } 
        }

        impl const From<Key> for ::avkeys_common::KeyCode {
            fn from(value: Key) -> Self {
                match value {
                    #(#ident_lookups),*
                }
            }
        }

    }.into()
}