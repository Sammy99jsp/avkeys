#![feature(const_trait_impl)]
//!
//! Common traits and structs to the
//! AvKey macro system, and library. 
//! 

pub mod codes;

pub type KeyCode = u32;

pub enum AvKeyDiscrim {
    Str(&'static str),
    Char(char),
    Int(u32),
}

impl const From<u32> for AvKeyDiscrim {
    fn from(value: u32) -> Self {
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