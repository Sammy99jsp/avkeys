#![feature(const_trait_impl, const_cmp, derive_const, const_option, const_convert)]
//!
//! AvdanOS helper library for parsing,
//! and validating keyboard shortcuts and keys.
//!
//! See rexeports for more information.
//!

mod key;

pub use avkeys_macros::AvKeybind;
pub use key::{AvKey, AvKeyParameter, KeyCode};
use avkeys_macros::keycodes;
use colored::Colorize;

keycodes! {
    //! 
    //! Keycodes from the linux header file:
    //! [/usr/include/linux/input-event-codes.h]
    //! 
    
    Escape      => 1     match [Esc],
    Digit1      => 2     match ['1', Dig1],
    Digit2      => 3     match ['2', Dig2],
    Digit3      => 4     match ['3', Dig3],
    Digit4      => 5     match ['4', Dig4],
    Digit5      => 6     match ['5', Dig5],
    Digit6      => 7     match ['6', Dig6],
    Digit7      => 8     match ['7', Dig7],
    Digit8      => 9     match ['8', Dig8],
    Digit9      => 10    match ['9', Dig9],
    Digit0      => 11    match ['0', Dig0],
    Minus		=> 12    match ['-'],
    Equal		=> 13    match ['='],
    Backspace   => 14    ,
    Tab			=> 15    match ['↹'],
    
    Q           => 16    ,
    W           => 17    ,
    E           => 18    ,
    R           => 19    ,
    T           => 20    ,
    Y           => 21    ,
    U           => 22    ,
    I           => 23    ,
    O           => 24    ,
    P           => 25    ,
    LeftBrace   => 26    match ['['],
    RightBrace  => 27    match [']'],
    Enter       => 28    ,

    LeftCtrl    => 29    match [Ctrl],

    A			=> 30    ,
    S			=> 31    ,
    D			=> 32    ,
    F			=> 33    ,
    G			=> 34    ,
    H			=> 35    ,
    J			=> 36    ,
    K			=> 37    ,
    L			=> 38    ,
    Semicolon   => 39    match [';'],
    Apostrophe  => 40    match ['\''],
    Grave       => 41    match ['`'],
    LeftShift   => 42    match [Shift],
    BackSlash   => 43    , // TODO: @Sammy99jsp add the character for this

    Z           =>  44   ,
    X           =>  45   ,
    C           =>  46   ,
    V           =>  47   ,
    B           =>  48   ,
    N           =>  49   ,
    M           =>  50   ,
    Comma       =>  51   match [','],
    Dot         =>  52   match ['.'],
    Slash       =>	53   match ['/'],
    RightShift  =>	54  ,
    KeyPadAsterisk  =>  55,
    LeftAlt     =>	56  match [Alt],
    Space       =>	57  ,
    CapsLock    =>	58  ,

    F1          =>	59  ,
    F2          =>	60  ,
    F3          =>	61  ,
    F4          =>	62  ,
    F5          =>	63  ,
    F6          =>	64  ,
    F7          =>	65  ,
    F8          =>	66  ,
    F9          =>	67  ,
    F10         =>	68  ,
    NumLock     =>	69  ,
    ScrollLock  =>	70  ,

    KeyPad7     =>	71  ,
    KeyPad8     =>	72  ,
    KeyPad9     =>	73  ,
    KeyPadMinus =>  74  ,
    KeyPad4     =>	75  ,
    KeyPad5     =>	76  ,
    KeyPad6     =>	77  ,
    KeyPadPlus  =>  78  ,
    KeyPad1     =>	79  ,
    KeyPad2     =>	80  ,
    KeyPad3     =>	81  ,
    KeyPad0     =>	82  ,
    KeyPadDot   =>  83  ,

    /* Keys 85..=86 Omitted */

    F11			=>  87  ,
    F12			=>  88  ,

    /* Keys 89..=95 Omitted */
    KeyPadEnter =>	96  ,
    RightCtrl   =>	97  ,
    KeyPadSlash =>	98  ,
    
    SysRq       =>	99  ,
    RightAlt    =>	100 ,

    /* Key 101 Omitted */

    Home        =>	102 ,
    UpArrow     =>	103 ,
    PageUp      =>	104 ,
    LeftArrow   =>	105 ,
    RightArrow  =>	106 ,
    End         =>	107 ,
    DownArrow   =>	108 ,
    PageDown    =>	109 ,
    Insert      =>	110 ,
    Delete      =>	111 ,

    Macro       =>  112 ,
    Mute        =>  113 ,
    VolumeDown  =>  114 ,
    VolumeUp    =>  115 ,

    ///
    /// SC System Power Down
    /// 
    Power       =>  116 ,	
    KeyPadEqual =>  117 ,
    KeyPadPlusMinus =>  118,
    Pause       =>  119 ,

    /* Key 120 Omitted */

    KeyPadComma =>  121 ,

    /* Keys 122..=124 Omitted */

    LeftMeta    =>  125 match [Meta, Logo, Win],
    RightMeta   =>  126 ,

    /* Keys 127..= 137 Omitted */

    ///
    /// AL Integrated Help Center 
    /// 
    Help        =>  138 ,
    
    ///
    /// Menu (show menu).
    /// 
    Menu        =>  139 ,

    /* Key 141 Omitted */
    Sleep       =>  142 ,
    Wakeup      =>  143 ,

    /* Keys 144..=152 */

    ///
    /// Display orientation for e.g. tablets.
    /// 
    RotateDisplay => 153 match [Direction],

    /* Keys 154..=162 Omitted */

    NextSong      => 163,
    PlayPause     => 164,
    PreviousSong  => 165,

    /* Keys 166..=223 Omitted */

    BrightnessDown  =>	224,
    BrightnessUp    =>	225,

    /* Keys 226..=248 Omitted */
}

impl Into<AvKey> for Key {
    fn into(self) -> AvKey {
        AvKey::Key(self.into())
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        self.name()
            .iter()
            .min_by_key(|n| n.len())
            .unwrap()
            .to_string()
    }
}

impl std::fmt::Display for AvKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AvKey::Key(k) => Key::lookup(*k)
                .and_then(|k|{
                    Some(k.to_string().blue())
                })
                .unwrap_or("ERR".strikethrough().red()),
            AvKey::Parameter(p) => format!("{{{}}}", p.to_string()).yellow(),
        })
    }
}

impl TryInto<String> for AvKey {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            AvKey::Key(k) => Key::lookup(k)
                .map(|k| Ok(k.to_string()))
                .unwrap_or(Err(())),
            AvKey::Parameter(p) => Ok(p.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::AvKey;

    use crate::Key;

    #[test]
    fn to_string_try_test() {
        let k : AvKey = Key::lookup("LeftCtrl").unwrap().into();
        println!("{k}");
    }
}