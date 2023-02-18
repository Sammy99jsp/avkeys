//!
//! Macros for defining keyboard shortcuts,
//! and syntax for keyboard keys/keybinds.
//! 
//! A part of the [AvdanOS Project](https://avdanos.org).
//! 

#![feature(proc_macro_diagnostic, iter_intersperse)]

mod key;

use avkeys_common::{parameters as KeyParams, AvKey, AvKeybind};

use key::{ParsedKeybind, ParsedKey, KEY_PARAMS};
use proc_macro::{TokenStream, Diagnostic, Level};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, ItemFn};

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
/// Though technically case-insensitive, we recommend using PascalCase (uppercase camelCase)
/// for key names: e.g. "Ctrl" over "CTRL", "ctrl".
/// 
/// The AvdanOS team may also add additional aliases (e.g. `Ctrl` -> `LCtrl`) for ease-of-use.
/// Check [the wiki]() for an updating list of these aliases. 
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
/// pub fn PetKitty(state : &mut (...), times : DigitParameter) {
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
pub fn AvKeybind(attrs : TokenStream, body : TokenStream) -> TokenStream {


    // 1. Parse Default Keybind 

    let keybind : ParsedKeybind = match syn::parse(attrs)
        .map_err(|err| 
            syn::Error::new(err.span(), 
                "Expected a + seperated non-trailing list of keys here:\n\
                Full Example: #[AvKeybind(Ctrl+[111]+{d})]"
            )
        ) 
    {
        Ok(v) => v,
        Err(err) => {
            return err.into_compile_error().into(); 
        },
    };

    // 2. Parse Implementation function.

    let func : ItemFn = match syn::parse(body)
        .map_err(|err| 
            syn::Error::new(err.span(), 
                "Expected a function declaration here.\n\
                Full Example:\n\n\
                #[AvKeybind(Ctrl+Alt+Del)]\n\
                pub fn PowerOptions(state: &mut (...)) {\n\
                \u{202f}   todo!(\"Whatever!\")\n\
                }"
            )
        )
    {
        Ok(v) => v,
        Err(err) => {
            return err.into_compile_error().into()
        }
    };

    // 2a. Enforce public access
    if !matches!(func.vis, syn::Visibility::Public(_)) {
        Diagnostic::spanned(
            func.vis.span().unwrap(), Level::Error, 
            "Keybinds must be declared `pub`"
        ).emit();

        return quote! {}.into();
    }

    
    // 2b. Validate callback signature.

    // 2b (i) Validate Key Parameter Names
    match keybind.validate_parameter_names() {
        Some(err) => return err,
        None      => {}
    };


    // Key Parameters in the Function Delcaration 
    
    // 2b (ii) Validate Key Parameters are in function signature
    //         Ensure all key parameters are present in the function signature.
    match keybind.validate_func_sign_against_key_params(&func.sig) {
        Some(err) => return err,
        None      => {}
    };

    // 3. Generate stuff

    // 3a. Generate key parameter bindings
    let pre_assignments : proc_macro2::TokenStream = match keybind.generate_key_parameter_assignments(&func.sig) {
        Err(err) => return err,
        Ok(stuff) => stuff,
    }.into();


    // 3b.
    // Keep original rustdoc comments by
    //      also relaying the original attributes macros (aka `#[doc = "..."]`).
    let attrs  = func.attrs.iter();
    let params = func.sig.inputs.iter();
    let keybind_name = func.sig.ident;


    let body = func.block;


    // FIXME(Sammy99jsp):   Auto-suggestions do not always behave
    //                      well inside this macro -- although this
    //                      could be a wider issue with proc-macros in general.
    let body = body.stmts.iter().map(|n|{
        let span = n.span();

        quote_spanned! {
            span => #n
        }
    });

    quote! {        
        #(#attrs)*
        struct #keybind_name(Option<Vec< ::avkeys_common::AvKey >>);

        impl ::avkeys_common::AvKeybind for #keybind_name {
            fn default_keys() -> &'static [::avkeys_common::AvKey]
                where Self : Sized
            {
                todo!("FIXME(Sammy99jsp) inside macro, lol.");
            }

            fn keys(&self) -> &[::avkeys_common::AvKey] {
                &self.0.as_ref()
                    .map(|v| v.as_slice())
                    .unwrap_or(&[/* Replace this!!! */])
            }

            fn run(&self, state : &mut (), __params__ : Vec<usize>) {
                #pre_assignments
                ::std::mem::drop(__params__);
                #(#body)*
            }
        }
    }.into()
}