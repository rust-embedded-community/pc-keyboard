//! Driver for a PS/2 keyboard.
//!
//! Only supports PS/2 Scan Code Set 2, on a UK English keyboard. See [the
//! OSDev Wiki](https://wiki.osdev.org/PS/2_Keyboard).
//!
//! Requires that you sample a pin in an interrupt routine and shift in the
//! bit. We don't sample the pin in this library, as that makes testing
//! difficult, and it means you have to make this object a global static mut
//! that the interrupt can access, which is unsafe.

#![cfg_attr(not(test), no_std)]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

#[cfg(not(test))]
use core::marker::PhantomData;

#[cfg(test)]
use std::marker::PhantomData;

// ****************************************************************************
//
// Modules
//
// ****************************************************************************

mod scancodes;
pub use scancodes::{ScancodeSet1, ScancodeSet2};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(Debug)]
pub struct Keyboard<T, S>
where
    T: KeyboardLayout<S>,
    S: ScancodeSet, 
{
    register: u16,
    num_bits: u8,
    decode_state: DecodeState,
    modifiers: Modifiers,
    _layout: PhantomData<T>,
    _set: PhantomData<S>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    BadStartBit,
    BadStopBit,
    ParityError,
    UnknownKeyCode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyCode {
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    PauseBreak,
    BackTick,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,
    Equals,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftSquareBracket,
    RightSquareBracket,
    Backslash,
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    SemiColon,
    Quote,
    Enter,
    ShiftLeft,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Fullstop,
    Slash,
    ShiftRight,
    ControlLeft,
    WindowsLeft,
    AltLeft,
    Spacebar,
    AltRight,
    WindowsRight,
    Menus,
    RightControl,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    UpArrow,
    LeftArrow,
    DownArrow,
    RightArrow,
    NumpadLock,
    NumpadSlash,
    NumpadStar,
    NumpadMinus,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadPlus,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad0,
    NumpadPeriod,
    NumpadEnter,
    /// Not on US keyboards
    HashTilde,
    // Scan code set 1 unique codes
    PrevTrack, 
    NextTrack, 
    Mute, 
    Calculator, 
    Play, 
    Stop, 
    VolumeDown, 
    VolumeUp, 
    WWWHome, 
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub state: KeyState,
}

pub trait KeyboardLayout<S> 
where 
    S: ScancodeSet
{
    /// Convert a Scan Code Set 2 byte to our `KeyCode` enum
    fn map_scancode(code: u8) -> Result<KeyCode, Error>;

    /// Convert a Scan Code Set 2 extended byte (prefixed E0) to our `KeyCode`
    /// enum.
    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error>;

    /// Convert a `KeyCode` enum to a Unicode character, if possible.
    /// KeyCode::A maps to `Some('a')` (or `Some('A')` if shifted), while
    /// KeyCode::AltLeft returns `None`
    fn map_keycode(keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey;
}

pub trait ScancodeSet {
    /// Convert a Scan Code set X byte to our 'KeyCode' enum
    fn map_scancode(code: u8) -> Result<KeyCode, Error>;
    
    /// Convert a Scan Code Set X extended byte (prefixed E0) to our `KeyCode`
    /// enum.
    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error>;
}

#[derive(Debug)]
pub struct Modifiers {
    pub lshift: bool,
    pub rshift: bool,
    pub numlock: bool,
    pub capslock: bool,
    pub alt_gr: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodedKey {
    RawKey(KeyCode),
    Unicode(char),
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

#[derive(Debug, Copy, Clone)]
enum DecodeState {
    Start,
    Extended,
    Release,
    ExtendedRelease,
}

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

const KEYCODE_BITS: u8 = 11;
const EXTENDED_KEY_CODE: u8 = 0xE0;
const KEY_RELEASE_CODE: u8 = 0xF0;

// ****************************************************************************
//
// Public Functions and Implementation
//
// ****************************************************************************

impl<T, S> Keyboard<T, S>
where
    T: KeyboardLayout<S>,
    S: ScancodeSet
{
    /// Make a new Keyboard object with the given layout.
    pub fn new(_layout: T, _set: S) -> Keyboard<T, S> {
        Keyboard {
            register: 0,
            num_bits: 0,
            decode_state: DecodeState::Start,
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                numlock: true,
                capslock: false,
                alt_gr: false
            },
            _layout: PhantomData,
            _set: PhantomData,
        }
    }

    /// Clears the bit register.
    ///
    /// Call this when there is a timeout reading data from the keyboard.
    pub fn clear(&mut self) {
        self.register = 0;
        self.num_bits = 0;
        self.decode_state = DecodeState::Start;
    }

    /// Processes a 16-bit word from the keyboard.
    ///
    /// * The start bit (0) must be in bit 0.
    /// * The data octet must be in bits 1..8, with the LSB in bit 1 and the
    ///   MSB in bit 8.
    /// * The parity bit must be in bit 9.
    /// * The stop bit (1) must be in bit 10.
    pub fn add_word(&mut self, word: u16) -> Result<Option<KeyEvent>, Error> {
        let byte = Self::check_word(word)?;
        self.add_byte(byte)
    }

    /// Processes an 8-bit byte from the keyboard.
    ///
    /// We assume the start, stop and parity bits have been processed and
    /// verified.
    pub fn add_byte(&mut self, byte: u8) -> Result<Option<KeyEvent>, Error> {
        let st = self.decode_state;
        self.clear();
        match st {
            DecodeState::Start => {
                // All keys start here
                let code = match byte {
                    KEY_RELEASE_CODE => {
                        self.decode_state = DecodeState::Release;
                        return Ok(None);
                    }
                    EXTENDED_KEY_CODE => {
                        self.decode_state = DecodeState::Extended;
                        return Ok(None);
                    }
                    e => T::map_scancode(e)?,
                };
                Ok(Some(KeyEvent::new(code, KeyState::Down)))
            }
            DecodeState::Extended => {
                // These are extended keys
                let code = match byte {
                    KEY_RELEASE_CODE => {
                        self.decode_state = DecodeState::ExtendedRelease;
                        return Ok(None);
                    }
                    e => T::map_extended_scancode(e)?,
                };
                Ok(Some(KeyEvent::new(code, KeyState::Down)))
            }
            DecodeState::Release => {
                // These are 'normal' keys being released
                let code = T::map_scancode(byte)?;
                Ok(Some(KeyEvent::new(code, KeyState::Up)))
            }
            DecodeState::ExtendedRelease => {
                // These are extended keys being release
                let code = T::map_extended_scancode(byte)?;
                Ok(Some(KeyEvent::new(code, KeyState::Up)))
            }
        }
    }

    /// Shift a bit into the register.
    ///
    /// Call this /or/ call `add_word` - don't call both.
    /// Until the last bit is added you get Ok(None) returned.
    pub fn add_bit(&mut self, bit: bool) -> Result<Option<KeyEvent>, Error> {
        self.register |= (bit as u16) << self.num_bits;
        self.num_bits += 1;
        if self.num_bits == KEYCODE_BITS {
            let word = self.register;
            self.add_word(word)
        } else {
            Ok(None)
        }
    }

    /// Processes a `KeyEvent` returned from `add_bit`, `add_byte` or `add_word`
    /// and produces a decoded key.
    ///
    /// For example, the KeyEvent for pressing the '5' key on your keyboard
    /// gives a DecodedKey of unicode character '5', unless the shift key is
    /// held in which case you get the unicode character '%'.
    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev {
            KeyEvent { code: KeyCode::ShiftLeft, state: KeyState::Down } => {
                self.modifiers.lshift = true;
                None
            }
            KeyEvent { code: KeyCode::ShiftRight, state: KeyState::Down } => {
                self.modifiers.rshift = true;
                None
            }
            KeyEvent { code: KeyCode::ShiftLeft, state: KeyState::Up } => {
                self.modifiers.lshift = false;
                None
            }
            KeyEvent { code: KeyCode::ShiftRight, state: KeyState::Up} => {
                self.modifiers.rshift = false;
                None
            }
            KeyEvent { code: KeyCode::CapsLock, state: KeyState::Down } => {
                self.modifiers.capslock = !self.modifiers.capslock;
                None
            }
            KeyEvent { code: KeyCode::NumpadLock, state: KeyState::Down } => {
                self.modifiers.numlock = !self.modifiers.numlock;
                None
            }
            KeyEvent { code: KeyCode::AltRight, state: KeyState::Down } => {
                self.modifiers.alt_gr = true;
                None
            }
            KeyEvent { code: KeyCode::AltRight, state: KeyState::Up } => {
                self.modifiers.alt_gr = false;
                None
            }
            KeyEvent { code: c, state: KeyState::Down } => {
                Some(T::map_keycode(c, &self.modifiers))
            }
            _ => None,
        }
    }

    fn get_bit(word: u16, offset: usize) -> bool {
        ((word >> offset) & 0x0001) != 0
    }

    fn has_even_number_bits(data: u8) -> bool {
        (data.count_ones() % 2) == 0
    }

    /// Check 11-bit word has 1 start bit, 1 stop bit and an odd parity bit.
    fn check_word(word: u16) -> Result<u8, Error> {
        let start_bit = Self::get_bit(word, 0);
        let parity_bit = Self::get_bit(word, 9);
        let stop_bit = Self::get_bit(word, 10);
        let data = ((word >> 1) & 0xFF) as u8;

        if start_bit {
            return Err(Error::BadStartBit);
        }

        if !stop_bit {
            return Err(Error::BadStopBit);
        }

        let need_parity = Self::has_even_number_bits(data);

        // Odd parity, so these must not match
        if need_parity != parity_bit {
            return Err(Error::ParityError);
        }

        Ok(data)
    }
}

impl KeyEvent {
    pub fn new(code: KeyCode, state: KeyState) -> KeyEvent {
        KeyEvent { code, state }
    }
}

// ****************************************************************************
//
// Keyboard Layouts
//
// ****************************************************************************

impl Modifiers {
    pub fn is_shifted(&self) -> bool {
        (self.lshift | self.rshift) ^ self.capslock
    }
}

pub mod layouts;

// ****************************************************************************
//
// Tests
//
// ****************************************************************************

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f9() {
        let mut k = Keyboard::new(layouts::Us104Key, ScancodeSet2);
        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        // parity
        assert_eq!(k.add_bit(false), Ok(None));
        // stop
        assert_eq!(
            k.add_bit(true),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_f9_word() {
        let mut k = Keyboard::new(layouts::Us104Key, ScancodeSet2);
        assert_eq!(
            k.add_word(0x0402),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_f9_byte() {
        let mut k = Keyboard::new(layouts::Us104Key, ScancodeSet2);
        assert_eq!(
            k.add_byte(0x01),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_keyup_keydown() {
        let mut k = Keyboard::new(layouts::Us104Key, ScancodeSet2);
        assert_eq!(
            k.add_byte(0x01),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
        assert_eq!(
            k.add_byte(0x01),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
        assert_eq!(
            k.add_byte(0xF0),
            Ok(None)
        );
        assert_eq!(
            k.add_byte(0x01),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Up)))
        );
    }

    #[test]
    fn test_f5() {
        let mut k = Keyboard::new(layouts::Us104Key, ScancodeSet2);
        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        // parity
        assert_eq!(k.add_bit(true), Ok(None));
        // stop
        assert_eq!(
            k.add_bit(true),
            Ok(Some(KeyEvent::new(KeyCode::F5, KeyState::Down)))
        );
    }

    #[test]
    fn test_f5_up() {
        let mut k = Keyboard::new(layouts::Us104Key, ScancodeSet2);
        // Send F0

        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        // parity
        assert_eq!(k.add_bit(true), Ok(None));
        // stop
        assert_eq!(k.add_bit(true), Ok(None));

        // Send 03

        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        // parity
        assert_eq!(k.add_bit(true), Ok(None));
        // stop
        assert_eq!(
            k.add_bit(true),
            Ok(Some(KeyEvent::new(KeyCode::F5, KeyState::Up)))
        );
    }

    #[test]
    fn test_shift() {
        let mut k = Keyboard::new(layouts::Uk105Key, ScancodeSet2);
        // A with left shift held
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::ShiftLeft, KeyState::Down)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Down)), Some(DecodedKey::Unicode('A')));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Up)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::ShiftLeft, KeyState::Up)), None);

        // A with no shift
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Down)), Some(DecodedKey::Unicode('a')));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Up)), None);

        // A with right shift held
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::ShiftRight, KeyState::Down)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Down)), Some(DecodedKey::Unicode('A')));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Up)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::ShiftRight, KeyState::Up)), None);

        // Caps lock ON
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::CapsLock, KeyState::Down)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::CapsLock, KeyState::Up)), None);

        // Letters are now caps
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::X, KeyState::Down)), Some(DecodedKey::Unicode('X')));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::X, KeyState::Up)), None);

        // Unless you press shift
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::ShiftRight, KeyState::Down)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Down)), Some(DecodedKey::Unicode('a')));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Up)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::ShiftRight, KeyState::Up)), None);
    }

    #[test]
    fn test_numlock() {
        let mut k = Keyboard::new(layouts::Uk105Key, ScancodeSet2);

        // Numlock ON by default
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Down)), Some(DecodedKey::Unicode('0')));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Up)), None);

        // Numlock OFF
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::NumpadLock, KeyState::Down)), None);
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::NumpadLock, KeyState::Up)), None);

        // Now KP_0 produces INSERT
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Down)), Some(DecodedKey::RawKey(KeyCode::Insert)));
        assert_eq!(k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Up)), None);
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
