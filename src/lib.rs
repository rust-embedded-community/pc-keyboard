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
    modifiers: Modifiers,
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

impl<T> Keyboard<T>
where
    T: KeyboardLayout,
{
    /// Make a new Keyboard object with the given layout.
    pub fn new(_layout: T) -> Keyboard<T> {
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

pub mod layouts {
    use super::*;

    /// A standard United States 101-key (or 104-key including Windows keys) keyboard.
    /// Has a 1-row high Enter key, with Backslash above.
    pub struct Us104Key;

    /// A standard United Kingdom 102-key (or 105-key including Windows keys) keyboard.
    /// Has a 2-row high Enter key, with Backslash next to the left shift.
    pub struct Uk105Key;

    impl KeyboardLayout for Us104Key {
        fn map_scancode(code: u8) -> Result<KeyCode, Error> {
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
                0x12 => Ok(KeyCode::ShiftLeft),          // 12
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

        fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
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

        fn map_keycode(keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey {
            match keycode {
                KeyCode::BackTick => {
                    if modifiers.is_shifted() {
                        DecodedKey::Unicode('~')
                    } else {
                        DecodedKey::Unicode('`')
                    }
                }
                KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
                KeyCode::Key1 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('!')
                } else {
                    DecodedKey::Unicode('1')
                },
                KeyCode::Key2 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('2')
                },
                KeyCode::Key3 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('3')
                },
                KeyCode::Key4 => {
                    if modifiers.is_shifted() {
                        DecodedKey::Unicode('$')
                    } else {
                        DecodedKey::Unicode('4')
                    }
                }
                KeyCode::Key5 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('5')
                },
                KeyCode::Key6 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('6')
                },
                KeyCode::Key7 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('7')
                },
                KeyCode::Key8 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('8')
                },
                KeyCode::Key9 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('9')
                },
                KeyCode::Key0 => if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('0')
                },
                KeyCode::Minus => if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                },
                KeyCode::Equals => if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('=')
                },
                KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
                KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
                KeyCode::Q => if modifiers.is_shifted() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                },
                KeyCode::W => if modifiers.is_shifted() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                },
                KeyCode::E => if modifiers.is_shifted() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                },
                KeyCode::R => if modifiers.is_shifted() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                },
                KeyCode::T => if modifiers.is_shifted() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                },
                KeyCode::Y => if modifiers.is_shifted() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                },
                KeyCode::U => if modifiers.is_shifted() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                },
                KeyCode::I => if modifiers.is_shifted() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                },
                KeyCode::O => if modifiers.is_shifted() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                },
                KeyCode::P => if modifiers.is_shifted() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
                },
                KeyCode::LeftSquareBracket => if modifiers.is_shifted() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('[')
                },
                KeyCode::RightSquareBracket => if modifiers.is_shifted() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode(']')
                },
                KeyCode::Backslash => if modifiers.is_shifted() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('\\')
                },
                KeyCode::A => if modifiers.is_shifted() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                },
                KeyCode::S => if modifiers.is_shifted() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                },
                KeyCode::D => if modifiers.is_shifted() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                },
                KeyCode::F => if modifiers.is_shifted() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                },
                KeyCode::G => if modifiers.is_shifted() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                },
                KeyCode::H => if modifiers.is_shifted() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                },
                KeyCode::J => if modifiers.is_shifted() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                },
                KeyCode::K => if modifiers.is_shifted() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                },
                KeyCode::L => if modifiers.is_shifted() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                },
                KeyCode::SemiColon => if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode(';')
                },
                KeyCode::Quote => if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('\'')
                },
                // Enter gives LF, not CRLF or CR
                KeyCode::Enter => DecodedKey::Unicode(10.into()),
                KeyCode::Z => if modifiers.is_shifted() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                },
                KeyCode::X => if modifiers.is_shifted() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                },
                KeyCode::C => if modifiers.is_shifted() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                },
                KeyCode::V => if modifiers.is_shifted() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                },
                KeyCode::B => if modifiers.is_shifted() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                },
                KeyCode::N => if modifiers.is_shifted() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                },
                KeyCode::M => if modifiers.is_shifted() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                },
                KeyCode::Comma => if modifiers.is_shifted() {
                    DecodedKey::Unicode('<')
                } else {
                    DecodedKey::Unicode(',')
                },
                KeyCode::Fullstop => if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('.')
                },
                KeyCode::Slash => if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('/')
                },
                KeyCode::Spacebar => DecodedKey::Unicode(' '),
                KeyCode::Delete => DecodedKey::Unicode(127.into()),
                KeyCode::NumpadSlash => DecodedKey::Unicode('/'),
                KeyCode::NumpadStar => DecodedKey::Unicode('*'),
                KeyCode::NumpadMinus => DecodedKey::Unicode('-'),
                KeyCode::Numpad7 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('7')
                    } else {
                        DecodedKey::RawKey(KeyCode::Home)
                    }
                }
                KeyCode::Numpad8 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('8')
                    } else {
                        DecodedKey::RawKey(KeyCode::UpArrow)
                    }
                }
                KeyCode::Numpad9 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('9')
                    } else {
                        DecodedKey::RawKey(KeyCode::PageUp)
                    }
                }
                KeyCode::NumpadPlus => DecodedKey::Unicode('+'),
                KeyCode::Numpad4 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('4')
                    } else {
                        DecodedKey::RawKey(KeyCode::LeftArrow)
                    }
                }
                KeyCode::Numpad5 => DecodedKey::Unicode('5'),
                KeyCode::Numpad6 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('6')
                    } else {
                        DecodedKey::RawKey(KeyCode::RightArrow)
                    }
                }
                KeyCode::Numpad1 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('1')
                    } else {
                        DecodedKey::RawKey(KeyCode::End)
                    }
                }
                KeyCode::Numpad2 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('2')
                    } else {
                        DecodedKey::RawKey(KeyCode::DownArrow)
                    }
                }
                KeyCode::Numpad3 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('3')
                    } else {
                        DecodedKey::RawKey(KeyCode::PageDown)
                    }
                }
                KeyCode::Numpad0 => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('0')
                    } else {
                        DecodedKey::RawKey(KeyCode::Insert)
                    }
                }
                KeyCode::NumpadPeriod => {
                    if modifiers.numlock {
                        DecodedKey::Unicode('.')
                    } else {
                        DecodedKey::Unicode(127.into())
                    }
                }
                KeyCode::NumpadEnter => DecodedKey::Unicode(10.into()),
                k => DecodedKey::RawKey(k),
            }
        }
    }

    impl KeyboardLayout for Uk105Key {
        fn map_scancode(code: u8) -> Result<KeyCode, Error> {
            match code {
                0x61 => Ok(KeyCode::Backslash),
                _ => Us104Key::map_scancode(code),
            }
        }

        fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
            match code {
                _ => Us104Key::map_extended_scancode(code),
            }
        }

        fn map_keycode(keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey {
            match keycode {
                KeyCode::BackTick => {
                    if modifiers.alt_gr {
                        DecodedKey::Unicode('|')
                    } else if modifiers.is_shifted() {
                        DecodedKey::Unicode('¬')
                    } else {
                        DecodedKey::Unicode('`')
                    }
                }
                KeyCode::Key2 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('2')
                },
                KeyCode::Quote => if modifiers.is_shifted() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('\'')
                },
                KeyCode::Key3 => if modifiers.is_shifted() {
                    DecodedKey::Unicode('£')
                } else {
                    DecodedKey::Unicode('3')
                },
                KeyCode::Key4 => {
                    if modifiers.alt_gr {
                        DecodedKey::Unicode('€')
                    } else if modifiers.is_shifted() {
                        DecodedKey::Unicode('$')
                    } else {
                        DecodedKey::Unicode('4')
                    }
                }
                KeyCode::HashTilde => if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('#')
                },
                e => Us104Key::map_keycode(e, modifiers),
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
        let mut k = Keyboard::new(layouts::Us104Key);
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
        let mut k = Keyboard::new(layouts::Us104Key);
        assert_eq!(
            k.add_word(0x0402),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_f9_byte() {
        let mut k = Keyboard::new(layouts::Us104Key);
        assert_eq!(
            k.add_byte(0x01),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_keyup_keydown() {
        let mut k = Keyboard::new(layouts::Us104Key);
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
        let mut k = Keyboard::new(layouts::Us104Key);
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
        let mut k = Keyboard::new(layouts::Us104Key);
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
        let mut k = Keyboard::new(layouts::Uk105Key);
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
        let mut k = Keyboard::new(layouts::Uk105Key);

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
