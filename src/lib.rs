//!
//! AvdanOS helper library for parsing,
//! and validating keyboard shortcuts and keys.
//! 

pub use avkeys_macros::AvKeybind;

#[cfg(test)]
mod tests {
    use super::AvKeybind;

    #[test]
    fn test_impl() {
        /// 
        /// Switches focus to the `i`-th item on the taksbar.
        /// 
        #[AvKeybind(Logo+{d})]
        pub fn SwitchFocusToWindow(state : &mut (), mut i : d) {
            if i == 0 { i = 9; }
            todo!("Switch focus to taskbar index {i}");
        }
    }
}