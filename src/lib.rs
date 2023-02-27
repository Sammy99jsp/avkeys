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
    ///
    /// Main modifier key.
    /// 
    LeftControl  => 12345 match [LCtrl, Ctrl, '⌘'],

    ///
    /// Toggles the case of a character,
    /// and gets symbols I guess.
    /// 
    LeftShift    => 12343 match [LShift, Shift],

    ///
    /// Escapes the matrix.
    /// 
    /// Yep, it's that easy.
    /// 
    Escape       => 1     match [Esc,],

    ///
    /// Usually the the Windows key.
    /// 
    Logo         => 123   match [Win, Windows, ],

    ///
    /// Period or Dot `.` Key
    /// 
    Period       => 2345 match [Dot, ],
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
        #[AvKeybind(Ctrl+Shift+Esc)]
        pub fn TaskMonitor(state: &mut ()) {}

        ///
        /// Cause everyone wants 'em...
        /// 
        #[AvKeybind(Win+Dot)]
        pub fn EmojiPicker(state: &mut ()) {

        }
    }
}
