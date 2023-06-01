#![feature(const_trait_impl)]
//!
//! Common traits and structs to the
//! AvKey macro system, and library. 
//! 

pub mod codes;

#[cfg(feature = "parsing")]
pub mod parsed_key;

#[cfg(feature = "parsing")]
pub use parsed_key::*;

pub type KeyCode = u32;

pub enum AvKeyDiscrim<'a> {
    Str(&'a str),
    Char(char),
    Int(u32),
}

#[const_trait]
pub trait IntoAvKeyDiscrim<'a> {
    fn into_discrim(self) -> AvKeyDiscrim<'a>;
}

impl<'a> IntoAvKeyDiscrim<'a> for u32 {
    fn into_discrim(self) -> AvKeyDiscrim<'a> {
        AvKeyDiscrim::Int(self)
    }
}
impl<'a> IntoAvKeyDiscrim<'a> for &'a str {
    fn into_discrim(self) -> AvKeyDiscrim<'a> {
        AvKeyDiscrim::Str(self)
    }
}
impl<'a> IntoAvKeyDiscrim<'a> for char {
    fn into_discrim(self) -> AvKeyDiscrim<'a> {
        AvKeyDiscrim::Char(self)
    }
}


impl<'a> From<u32> for AvKeyDiscrim<'a> {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}
impl<'a> From<&'a str> for AvKeyDiscrim<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<char> for AvKeyDiscrim<'_> {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::AvKeyDiscrim;

    #[test]
    fn avkey_discrim() {
        let a = AvKeyDiscrim::Str("sdfgh");
    }
}