use std::iter;

use syn::{punctuated::Punctuated, token::{Bracket, Brace}, LitInt, Ident, Token, parse::Parse, bracketed, LitStr, LitChar, braced, Item, Attribute};

syn::custom_punctuation!(EscapeCode, #);

#[derive(Clone)]
pub enum KeyIdentifier {
    LitInt(LitInt),
    Ident(Ident),
    LitChar(LitChar)
}

impl ToString for KeyIdentifier {
    fn to_string(&self) -> String {
        match self {
            KeyIdentifier::LitInt(i) => i.to_string(),
            KeyIdentifier::Ident(ident) => ident.to_string(),
            KeyIdentifier::LitChar(ch) => ch.value().to_string(),
        }
    }
}

impl Parse for KeyIdentifier {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitInt) {
            return Ok(Self::LitInt(input.parse()?));
        }

        if input.peek(Ident) {
            return Ok(Self::Ident(input.parse()?));
        }

        if input.peek(LitChar) {
            return Ok(Self::LitChar(input.parse()?));
        }

        Err(syn::Error::new(input.span(), "Expected either identifier; or int or char literal here."))
    }
}

///
/// Allows for multiple definitions
/// of aliases.
/// 
pub struct ParseKeyCodeAliases {
    keyword     : Token![match],
    brackets    : Bracket,
    aliases     : Punctuated<KeyIdentifier, Token![,]>
}

impl Parse for ParseKeyCodeAliases {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        Ok(
            Self {
                keyword : input.parse()?,
                brackets: bracketed!(inside in input),
                aliases : Punctuated::<KeyIdentifier, Token![,]>::parse_terminated(&inside)?
            }
        )
    }
}

impl ParseKeyCodeAliases {
    fn iter(&self) -> impl Iterator<Item = &KeyIdentifier> {
        self.aliases.iter()
    }

    fn into_iter(self) -> impl Iterator<Item = KeyIdentifier> {
        self.aliases.into_iter()
    }
}


///
/// A definition of a keycode (Name to u32 + aliases)
/// 
/// Example:
/// ```ignore
/// LeftCtrl => 2 as [LCtrl, Ctrl]
/// ```
/// 
pub struct ParseKeyCodeDefinition {
    attributes : Vec<Attribute>,
    primary : KeyIdentifier,
    arrow   : Token![=>],
    value   : LitInt,
    aliases : Option<ParseKeyCodeAliases>
}

impl ParseKeyCodeDefinition {
    pub fn code(&self) -> u32 {
        self.value.base10_digits().parse().unwrap()
    }

    ///
    /// Return all aliases (including primary) for this
    /// key.
    /// 
    pub fn aliases(&self) -> impl Iterator<Item = &KeyIdentifier> {
        self.aliases.as_ref()
            .map(|aliases|
                aliases.iter()
                    .collect()
            )
                .unwrap_or(vec![])
            .into_iter()
                .chain(std::iter::once(&self.primary))
    }

    pub fn attrs(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }

    pub fn primary(&self) -> &KeyIdentifier {
        &self.primary
    }
}

impl Parse for ParseKeyCodeDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attributes: input.call(Attribute::parse_outer)?,
            primary : input.parse()?,
            arrow   : input.parse()?,
            value   : input.parse()?,
            aliases : match input.peek(Token![match]) {
                true  => Some(input.parse()?),
                false => None
            }
        })
    }
}

pub struct KeyCodesCollection {
    contents     : Punctuated<ParseKeyCodeDefinition, Token![,]>
}

impl Parse for KeyCodesCollection {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            contents    : input.parse_terminated(ParseKeyCodeDefinition::parse)?,
        })
    }
}


impl KeyCodesCollection {
    pub fn iter(&self) -> impl Iterator<Item = &ParseKeyCodeDefinition> {
        self.contents.iter()
    }
}