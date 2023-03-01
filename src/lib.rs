#![feature(const_trait_impl, const_cmp, derive_const, const_option, const_convert)]
//!
//! AvdanOS helper library for parsing,
//! and validating keyboard shortcuts and keys.
//!
//! See rexeports for more information.
//!

pub use avkeys_macros::AvKeybind;

pub use avkeys_common::*;
use avkeys_macros::keycodes;

keycodes! {
    //! 
    //! Lol
    //! 
    
    ///
    /// What the actual heck.
    /// 
    Ctrl => 2 match [LeftCtl]
}

#[cfg(test)]
mod tests {}
