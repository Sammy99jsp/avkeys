#![feature(const_trait_impl)]
//!
//! Common traits and structs to the
//! AvKey macro system, and library. 
//! 

pub use key::*;
mod key;
pub mod codes;


//
/// ## AvKeybinds
/// 
/// Macro-`impl`'d Trait for declaring compositor keybinds.
/// 
/// Keybinds, also known as keyboard shortucts, represents
/// a specical key combination that the compositor
/// listens for and executes a specific action when triggered.
/// 
/// This is used in conjunction with `av_macros::AvKeybind`
/// attribute macro to generate an implementation.
/// 
/// ### Examples
/// #### `Ctrl`+`Alt`+`Del` &mdash; Power Options
/// ```ignore
/// use av_macro::AvKeybind;
/// use Navda::AvKeybind;
/// 
/// ///
/// /// Open the in-built power options menu. 
/// ///
/// #[AvKeybind(Ctrl+Alt+Del)]
/// pub fn PowerOptions(state : &mut (...)) {
///     todo!("Launch fancy GUI here...")
/// }
/// ```
/// 
pub trait AvKeybind {
    
    ///
    /// Get the default key combination.
    /// 
    fn default_keys() -> &'static [AvKey]
        where Self : Sized; 


    ///
    /// Get the user-defined combination,
    /// or the default, if missing.
    /// 
    fn keys(&self) -> &[AvKey];

    ///
    /// Run the action associated with this keybind.
    /// 
    fn run(&self, state : &mut (), parameters: Vec<usize>);
}

pub enum AvKeyDiscrim {
    Str(&'static str),
    Char(char),
    Int(i32),
}

impl const From<i32> for AvKeyDiscrim {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl const From<&'static str> for AvKeyDiscrim {
    fn from(value: &'static str) -> Self {
        Self::Str(value)
    }
}

impl const From<char> for AvKeyDiscrim {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

