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
    LeftControl  => 12345 match [LCtrl, Ctrl, '⌘'],
    LeftShift    => 12343 match [LShift, Shift],
    Escape       => 1     match [Esc,],
}

#[cfg(test)]
mod tests {
    use crate::Key;

    use super::AvKeybind;

    #[test]
    fn test_impl() {
        ///
        /// Opens up a windows task manager clone.
        /// 
        /// Lol.
        ///
        #[AvKeybind(Ctrl+Shift+Esc)]
        pub fn TaskMonitor(state: &mut ()) {
            
        }
    }
}
