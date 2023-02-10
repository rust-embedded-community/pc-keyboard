//! Dvorak keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A Dvorak 101-key (or 104-key including Windows keys) keyboard.
///
/// Has a 1-row high Enter key, with Oem7 above (ANSI layout).
pub struct Dvorak104Key;

impl KeyboardLayout for Dvorak104Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
        match keycode {
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('[')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode(']')
                }
            }
            KeyCode::Q => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::W => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('<')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::E => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::R => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0010}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
                }
            }
            KeyCode::T => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0019}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::Y => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0006}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                }
            }
            KeyCode::U => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0007}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                }
            }
            KeyCode::I => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0003}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                }
            }
            KeyCode::O => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0012}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                }
            }
            KeyCode::P => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000C}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                }
            }
            KeyCode::Oem4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('/')
                }
            }
            KeyCode::Oem6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::S => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000F}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                }
            }
            KeyCode::D => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0005}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::F => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0015}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                }
            }
            KeyCode::G => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0009}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                }
            }
            KeyCode::H => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0004}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                }
            }
            KeyCode::J => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0008}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                }
            }
            KeyCode::K => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0014}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                }
            }
            KeyCode::L => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000E}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                }
            }
            KeyCode::Oem1 => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0013}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Z => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::X => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::C => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                }
            }
            KeyCode::V => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000B}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                }
            }
            KeyCode::B => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0018}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                }
            }
            KeyCode::N => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0002}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                }
            }
            KeyCode::OemComma => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0017}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::OemPeriod => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0016}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                }
            }
            KeyCode::Oem2 => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            e => {
                let us = super::Us104Key;
                us.map_keycode(e, modifiers, handle_ctrl)
            }
        }
    }
}
