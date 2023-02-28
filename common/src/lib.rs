#![feature(const_trait_impl)]
//!
//! Common traits and structs to the
//! AvKey macro system, and library. 
//! 

pub use key::*;
mod key;
pub mod codes;


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

