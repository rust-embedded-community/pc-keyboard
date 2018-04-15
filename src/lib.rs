//! Driver for a PS/2 keyboard.
//!
//! Only supports PS/2 Scan Code Set 2, on a UK English keyboard. See [the
//! OSDev Wiki](https://wiki.osdev.org/PS/2_Keyboard).
//!
//! Requires that you sample a pin in an interrupt routine and shift in the
//! bit. We don't sample the pin in this library, as that makes testing
//! difficult, and it means you have to make this object a global static mut
//! that the interrupt can access, which is unsafe.

// #![cfg_attr(not(test), no_std)]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

// #[cfg(not(test))]
// use core::marker::PhantomData;

// #[cfg(test)]
use std::marker::PhantomData;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(Debug)]
pub struct Keyboard<T>
where
    T: KeyboardLayout,
{
    register: u16,
    num_bits: u8,
    decode_state: DecodeState,
    _layout: PhantomData<T>,
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
    Escape,             // 76
    F1,                 // 05
    F2,                 // 06
    F3,                 // 04
    F4,                 // 0C
    F5,                 // 03
    F6,                 // 0B
    F7,                 // 83
    F8,                 // 0A
    F9,                 // 01
    F10,                // 09
    F11,                // 78
    F12,                // 07
    PrintScreen,        // E012 E07C / E0F07C E0F012
    ScrollLock,         // 7E
    PauseBreak,         // E11477 E1F014 E077
    BackTick,           // 0E
    Key1,               // 16
    Key2,               // 1e
    Key3,               // 26
    Key4,               // 25
    Key5,               // 2E
    Key6,               // 36
    Key7,               // 3D
    Key8,               // 3E
    Key9,               // 46
    Key0,               // 45
    Minus,              // 4E
    Equals,             // 55
    Backspace,          // 66
    Tab,                // 0D
    Q,                  // 15
    W,                  // 1D
    E,                  // 24
    R,                  // 2D
    T,                  // 2C
    Y,                  // 35
    U,                  // 3C
    I,                  // 43
    O,                  // 44
    P,                  // 4D
    LeftSquareBracket,  // 54
    RightSquareBracket, // 5B
    Backslash,          // 5D
    CapsLock,           // 58
    A,                  // 1C
    S,                  // 1B
    D,                  // 23
    F,                  // 2B
    G,                  // 34
    H,                  // 33
    J,                  // 3B
    K,                  // 42
    L,                  // 4B
    SemiColon,          // 4C
    Quote,              // 52
    Enter,              // 5A
    LeftShift,          // 12
    Z,                  // 1A
    X,                  // 22
    C,                  // 21
    V,                  // 2A
    B,                  // 32
    N,                  // 31
    M,                  // 3A
    Comma,              // 41
    Fullstop,           // 49
    Slash,              // 4A
    ShiftRight,         // 59
    ControlLeft,        // 14
    WindowsLeft,        // E01F
    AltLeft,            // 11
    Spacebar,           // 29
    AltRight,           // E011
    WindowsRight,       // E027
    Menus,              // E02F
    RightControl,       // E014
    Insert,             // E070
    Home,               // E06C
    PageUp,             // E07D
    Delete,             // E071
    End,                // E069
    PageDown,           // E07A
    UpArrow,            // E075
    LeftArrow,          // E06B
    DownArrow,          // E072
    RightArrow,         // E074
    NumpadLock,         // 77
    NumpadSlash,        // E04A
    NumpadStar,         // 7C
    NumpadMinus,        // 7B
    Numpad7,            // 6C
    Numpad8,            // 75
    Numpad9,            // 7D
    NumpadPlus,         // 79
    Numpad4,            // 6B
    Numpad5,            // 73
    Numpad6,            // 74
    Numpad1,            // 69
    Numpad2,            // 72
    Numpad3,            // 7A
    Numpad0,            // 70
    NumpadPeriod,       // 71
    NumpadEnter,        // E05A
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

pub trait KeyboardLayout {
    fn map_key(code: u8) -> Result<KeyCode, Error>;
    fn map_extended_key(code: u8) -> Result<KeyCode, Error>;
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

impl<T> Keyboard<T>
where
    T: KeyboardLayout,
{
    pub fn new(_layout: T) -> Keyboard<T> {
        Keyboard {
            register: 0,
            num_bits: 0,
            decode_state: DecodeState::Start,
            _layout: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        self.register = 0;
        self.num_bits = 0;
        self.decode_state = DecodeState::Start;
    }

    pub fn add_bit(&mut self, bit: bool) -> Result<Option<KeyEvent>, Error> {
        self.register |= (bit as u16) << self.num_bits;
        self.num_bits += 1;
        if self.num_bits == KEYCODE_BITS {
            let byte = Self::check_parity(self.register)?;
            let st = self.decode_state;
            self.clear();
            match st {
                DecodeState::Start => {
                    // All keys start here
                    let code = match byte {
                        KEY_RELEASE_CODE => {
                            println!("Key release!\n");
                            self.decode_state = DecodeState::Release;
                            return Ok(None);
                        }
                        EXTENDED_KEY_CODE => {
                            self.decode_state = DecodeState::Extended;
                            return Ok(None);
                        }
                        e => T::map_key(e)?,
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
                        e => T::map_extended_key(e)?,
                    };
                    Ok(Some(KeyEvent::new(code, KeyState::Down)))
                }
                DecodeState::Release => {
                    // These are 'normal' keys being released
                    let code = T::map_key(byte)?;
                    Ok(Some(KeyEvent::new(code, KeyState::Up)))
                }
                DecodeState::ExtendedRelease => {
                    // These are extended keys being release
                    let code = T::map_extended_key(byte)?;
                    Ok(Some(KeyEvent::new(code, KeyState::Up)))
                }
            }
        } else {
            Ok(None)
        }
    }

    fn get_bit(word: u16, offset: usize) -> bool {
        ((word >> offset) & 0x0001) != 0
    }

    fn has_even_number_bits(data: u8) -> bool {
        (data.count_ones() % 2) == 0
    }

    /// Check 11-bit word has 1 start bit, 1 stop bit and an odd parity bit.
    fn check_parity(word: u16) -> Result<u8, Error> {
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

pub mod layouts {
    use super::*;

    pub struct EnglishUk;

    impl KeyboardLayout for EnglishUk {
        fn map_key(code: u8) -> Result<KeyCode, Error> {
            match code {
                0x01 => Ok(KeyCode::F9),                 // 01
                0x03 => Ok(KeyCode::F5),                 // 03
                0x04 => Ok(KeyCode::F3),                 // 04
                0x05 => Ok(KeyCode::F1),                 // 05
                0x06 => Ok(KeyCode::F2),                 // 06
                0x07 => Ok(KeyCode::F12),                // 07
                0x09 => Ok(KeyCode::F10),                // 09
                0x0A => Ok(KeyCode::F8),                 // 0A
                0x0B => Ok(KeyCode::F6),                 // 0B
                0x0C => Ok(KeyCode::F4),                 // 0C
                0x0D => Ok(KeyCode::Tab),                // 0D
                0x0E => Ok(KeyCode::BackTick),           // 0E
                0x11 => Ok(KeyCode::AltLeft),            // 11
                0x12 => Ok(KeyCode::LeftShift),          // 12
                0x14 => Ok(KeyCode::ControlLeft),        // 14
                0x15 => Ok(KeyCode::Q),                  // 15
                0x16 => Ok(KeyCode::Key1),               // 16
                0x1A => Ok(KeyCode::Z),                  // 1A
                0x1B => Ok(KeyCode::S),                  // 1B
                0x1C => Ok(KeyCode::A),                  // 1C
                0x1D => Ok(KeyCode::W),                  // 1D
                0x1e => Ok(KeyCode::Key2),               // 1e
                0x21 => Ok(KeyCode::C),                  // 21
                0x22 => Ok(KeyCode::X),                  // 22
                0x23 => Ok(KeyCode::D),                  // 23
                0x24 => Ok(KeyCode::E),                  // 24
                0x25 => Ok(KeyCode::Key4),               // 25
                0x26 => Ok(KeyCode::Key3),               // 26
                0x29 => Ok(KeyCode::Spacebar),           // 29
                0x2A => Ok(KeyCode::V),                  // 2A
                0x2B => Ok(KeyCode::F),                  // 2B
                0x2C => Ok(KeyCode::T),                  // 2C
                0x2D => Ok(KeyCode::R),                  // 2D
                0x2E => Ok(KeyCode::Key5),               // 2E
                0x31 => Ok(KeyCode::N),                  // 31
                0x32 => Ok(KeyCode::B),                  // 32
                0x33 => Ok(KeyCode::H),                  // 33
                0x34 => Ok(KeyCode::G),                  // 34
                0x35 => Ok(KeyCode::Y),                  // 35
                0x36 => Ok(KeyCode::Key6),               // 36
                0x3A => Ok(KeyCode::M),                  // 3A
                0x3B => Ok(KeyCode::J),                  // 3B
                0x3C => Ok(KeyCode::U),                  // 3C
                0x3D => Ok(KeyCode::Key7),               // 3D
                0x3E => Ok(KeyCode::Key8),               // 3E
                0x41 => Ok(KeyCode::Comma),              // 41
                0x42 => Ok(KeyCode::K),                  // 42
                0x43 => Ok(KeyCode::I),                  // 43
                0x44 => Ok(KeyCode::O),                  // 44
                0x45 => Ok(KeyCode::Key0),               // 45
                0x46 => Ok(KeyCode::Key9),               // 46
                0x49 => Ok(KeyCode::Fullstop),           // 49
                0x4A => Ok(KeyCode::Slash),              // 4A
                0x4B => Ok(KeyCode::L),                  // 4B
                0x4C => Ok(KeyCode::SemiColon),          // 4C
                0x4D => Ok(KeyCode::P),                  // 4D
                0x4E => Ok(KeyCode::Minus),              // 4E
                0x52 => Ok(KeyCode::Quote),              // 52
                0x54 => Ok(KeyCode::LeftSquareBracket),  // 54
                0x55 => Ok(KeyCode::Equals),             // 55
                0x58 => Ok(KeyCode::CapsLock),           // 58
                0x59 => Ok(KeyCode::ShiftRight),         // 59
                0x5A => Ok(KeyCode::Enter),              // 5A
                0x5B => Ok(KeyCode::RightSquareBracket), // 5B
                0x5D => Ok(KeyCode::Backslash),          // 5D
                0x66 => Ok(KeyCode::Backspace),          // 66
                0x69 => Ok(KeyCode::Numpad1),            // 69
                0x6B => Ok(KeyCode::Numpad4),            // 6B
                0x6C => Ok(KeyCode::Numpad7),            // 6C
                0x70 => Ok(KeyCode::Numpad0),            // 70
                0x71 => Ok(KeyCode::NumpadPeriod),       // 71
                0x72 => Ok(KeyCode::Numpad2),            // 72
                0x73 => Ok(KeyCode::Numpad5),            // 73
                0x74 => Ok(KeyCode::Numpad6),            // 74
                0x75 => Ok(KeyCode::Numpad8),            // 75
                0x76 => Ok(KeyCode::Escape),             // 76
                0x77 => Ok(KeyCode::NumpadLock),         // 77
                0x78 => Ok(KeyCode::F11),                // 78
                0x79 => Ok(KeyCode::NumpadPlus),         // 79
                0x7A => Ok(KeyCode::Numpad3),            // 7A
                0x7B => Ok(KeyCode::NumpadMinus),        // 7B
                0x7C => Ok(KeyCode::NumpadStar),         // 7C
                0x7D => Ok(KeyCode::Numpad9),            // 7D
                0x7E => Ok(KeyCode::ScrollLock),         // 7E
                0x83 => Ok(KeyCode::F7),                 // 83
                _ => Err(Error::UnknownKeyCode),
            }
        }

        fn map_extended_key(code: u8) -> Result<KeyCode, Error> {
            match code {
                0x11 => Ok(KeyCode::AltRight),     // E011
                0x14 => Ok(KeyCode::RightControl), // E014
                0x1F => Ok(KeyCode::WindowsLeft),  // E01F
                0x27 => Ok(KeyCode::WindowsRight), // E027
                0x2F => Ok(KeyCode::Menus),        // E02F
                0x4A => Ok(KeyCode::NumpadSlash),  // E04A
                0x5A => Ok(KeyCode::NumpadEnter),  // E05A
                0x69 => Ok(KeyCode::End),          // E069
                0x6B => Ok(KeyCode::LeftArrow),    // E06B
                0x6C => Ok(KeyCode::Home),         // E06C
                0x70 => Ok(KeyCode::Insert),       // E070
                0x71 => Ok(KeyCode::Delete),       // E071
                0x72 => Ok(KeyCode::DownArrow),    // E072
                0x74 => Ok(KeyCode::RightArrow),   // E074
                0x75 => Ok(KeyCode::UpArrow),      // E075
                0x7A => Ok(KeyCode::PageDown),     // E07A
                0x7D => Ok(KeyCode::PageUp),       // E07D
                _ => Err(Error::UnknownKeyCode),
            }
        }
    }
}

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
        let mut k = Keyboard::new(layouts::EnglishUk);
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
    fn test_f5() {
        let mut k = Keyboard::new(layouts::EnglishUk);
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
        let mut k = Keyboard::new(layouts::EnglishUk);
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
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
