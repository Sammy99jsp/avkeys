#![feature(const_trait_impl, const_cmp, derive_const, const_option)]
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

    /// Escape Key
    Escape =>     1 match [Esc],

    // DIGIT KEYS //

    /// Digit Key 1
    Digit1   =>   2 match [1, ],

    /// Digit Key 2
    Digit2   =>   3 match [2, ],

    /// Digit Key 3
    Digit3   =>   4 match [3, ],

    /// Digit Key 4
    Digit4   =>   5 match [4, ],

    /// Digit Key 5
    Digit5   =>   6 match [5, ],

    /// Digit Key 6
    Digit6   =>   7 match [6, ],

    /// Digit Key 7
    Digit7   =>   8 match [7, ],

    /// Digit Key 8
    Digit8   =>   9 match [8, ],

    /// Digit Key 9
    Digit9   =>  10 match [9, ],

    /// Digit Key 0
    Digit0   =>  11 match [0, ],
    
    Minus  =>  12 match ['-', ],
    Equal  =>  13 match ['=', ],
    
    Backspace => 14, 
    
    Tab    =>  15,
}

#[cfg(test)]
mod tests {
    use crate::Keys;

    use super::AvKeybind;

    #[test]
    fn test_impl() {
        ///
        /// Switches focus to the `i`-th item on the taksbar.
        ///
        #[AvKeybind(Logo+':'+{d})]
        pub fn SwitchFocusToWindow(state: &mut (), mut i: d) {
            if i == 0 {
                i = 9;
            }
            todo!("Switch focus to taskbar index {i}");
        }
    }

    #[test]
    fn test_lookup_const() {
        const k : Keys = Keys::lookup_const('-').unwrap();
        println!("{k:?}");
    }
}

