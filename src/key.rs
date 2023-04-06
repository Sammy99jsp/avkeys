


///
/// Type of an input keycode.
/// 
/// See:
/// * Linux Headers @ [/usr/include/linux/input-event-codes.h]
/// * [A mirror](https://gitlab.freedesktop.org/libinput/libinput/-/blob/main/include/linux/linux/input-event-codes.h)
/// on FreeDesktop.org for a list of keycodes.
/// 
pub type KeyCode = u32;

///
/// ### Keyboard Keys
/// 
/// The `AvKey` enum represents a keyboard key in a key combination.
/// 
/// It supports:
/// * a fixed key, or
/// * a colllection of keys. 
/// 
#[derive(Debug, Clone, Copy)]
pub enum AvKey {
    ///
    /// A fixed physical key, using linux' keycodes.
    /// 
    Key(KeyCode),

    ///
    /// Represents a collection of related keys,
    /// to support one keybind implementation for multiple
    /// related key combinations.
    /// 
    /// See [AvKeyParameter] for more information.
    /// 
    Parameter(AvKeyParameter)
}

impl AvKey {
    ///
    /// Returns an option of whether this [AvKey] is a
    /// key parameter ([AvKeyParameter]).
    /// 
    /// ### Example
    /// ```ignore
    /// let keys = vec![AvKey::Key(2), AvKey::Parameter(AvKeyParameter::DigitKey)];
    /// let key_params = keys.into_iter()
    ///     .filter_map(AvKey::key_parameter)
    ///     .collect::<Vec<_>>();
    /// 
    /// assert_eq!(key_params, vec![AvKey::Parameter(AvKeyParameter::DigitKey)]);
    /// ```
    /// 
    pub fn key_parameter(&self) -> Option<AvKeyParameter> {
        match self {
            AvKey::Key(_) => None,
            AvKey::Parameter(p) => Some(p.clone()),
        }
    }

    ///
    /// Returns an option of wether this [AvKey] is
    /// a real key (has a keycode).
    ///
    /// ### Example
    /// ```ignore
    /// let keys = vec![AvKey::Key(2), AvKey::Parameter(AvKeyParameter::DigitKey)];
    /// let real_keys = keys.into_iter()
    ///     .filter_map(AvKey::key)
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(real_keys, vec![AvKey::Key(2)]);
    /// ```
    ///
    pub fn key(&self) -> Option<KeyCode> {
        match self {
            AvKey::Key(k) => Some(*k),
            AvKey::Parameter(_) => None,
        }
    }
}

///
/// ## Key Parameters
/// 
/// A way of capturing multiple keys (in the same category) at once,
/// 
/// ### Types
/// * [Digit Keys](parameters::DigitKey) (`0`..=`9`) `{d}` 
/// * [Function Keys](parameters::FunctionKey) (`F1`..=`F12`) `{f}` 
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvKeyParameter {
    ///
    /// ### Key Parameter `{d}` &mdash; Digit Key
    /// Used in place for any digit key (not keypad keys).
    /// 
    /// #### Syntax
    /// When declaring keybinds, use the `{d}` syntax to specify
    /// this key parameter.
    /// 
    /// #### Example
    /// ```ignore
    /// use av_macros::AvKeybind;
    /// use Navda::AvKeybind;
    /// 
    /// ///
    /// /// Switch the active window to the `d`-th item on the taskbar.
    /// ///
    /// #[AvKeybind(Logo+{d})]
    /// pub fn SwitchWindow(state : &mut (...), item : usize) {
    ///     let current   = state.taskbar.active();
    ///     if current == item {
    ///         return;
    ///     }
    ///     let new_focus = state.taskbar.nth(d);
    /// 
    ///     state.set_focused(new_focus);
    /// }
    /// ```
    /// 
    DigitKey,

    ///
    /// ### Key Parameter `{f}` &mdash; Function Key
    /// Used in place for any function key (`F1` to `F12`, inclusive).
    /// 
    /// #### Syntax
    /// When declaring keybinds, use the `{f}` syntax to specify
    /// this key parameter.
    /// 
    /// #### Example
    /// ```ignore
    /// use av_macros::AvKeybind;
    /// use Navda::AvKeybind;
    /// 
    /// ///
    /// /// Switch the active VTT to the `f`-th VTT.
    /// ///
    /// #[AvKeybind(Ctrl+Alt+{f})]
    /// pub fn SwitchWindow(state : &mut (...), item : usize) {
    ///     state.switch_vtt(item);
    /// }
    /// ```
    /// 
    FunctionKey
}

// Number Keys:                     0   1  2  3  4  5  6  7  8   9    
const DIGIT_KEYS : [KeyCode; 10] = [11, 2, 3, 4, 5, 6, 7, 8, 9, 10];
// Function Keys:                 F..  1   2   3   4   5   6   7   8   9   10  11  12
const FUNCTION_KEYS : [KeyCode; 12] = [59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 87, 88];


impl AvKeyParameter {
    ///
    /// Returns keys in this KeyParameter's bounds.
    /// 
    pub fn keys(&self) -> &'static [KeyCode] {
        match self {
            AvKeyParameter::DigitKey => &DIGIT_KEYS,
            AvKeyParameter::FunctionKey => &FUNCTION_KEYS,
        }
    }

    ///
    /// Returns a value associated with a specific key
    /// by the key parameter.
    /// 
    pub fn value(&self, key : KeyCode) -> Option<usize> {
        match self {
            AvKeyParameter::DigitKey => {
                DIGIT_KEYS
                    .iter().enumerate()
                    .find(|(_, k)| **k == key)
                    .map(|(i, _)| i)
            },
            AvKeyParameter::FunctionKey => {
                FUNCTION_KEYS
                    .iter().enumerate()
                    .find(|(_, k)| **k == key)
                    .map(|(i, _)| i + 1)
            },
        }
    }
}

impl From<AvKeyParameter> for String {
    fn from(p: AvKeyParameter) -> Self {
        match p {
            AvKeyParameter::DigitKey => "d",
            AvKeyParameter::FunctionKey => "f",
        }
        .to_string()
    }
}

impl TryFrom<String> for AvKeyParameter {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "d" => Ok(AvKeyParameter::DigitKey),
            "f" => Ok(AvKeyParameter::FunctionKey),
            _   => Err(value)
        }
    }
}
impl<'a> TryFrom<&'a str> for AvKeyParameter {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "d" => Ok(AvKeyParameter::DigitKey),
            "f" => Ok(AvKeyParameter::FunctionKey),
            _   => Err(value.to_string())
        }
    }
}

impl ToString for AvKeyParameter {
    fn to_string(&self) -> String {
        <Self as Into<String>>::into(self.clone())
    }
}



impl PartialEq for AvKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Key(l), Self::Key(r)) => l == r,
            (Self::Parameter(_), Self::Parameter(_)) => unimplemented!(),
            (Self::Key(ref l), Self::Parameter(r)) => r.keys().contains(l),
            (Self::Parameter(l), Self::Key(ref r)) => l.keys().contains(r)
        }
    }
}

impl Eq for AvKey {}