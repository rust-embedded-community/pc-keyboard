//! German keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard German 102-key (or 105-key including Windows keys) keyboard.
///
/// The top row spells `QWERTZ`.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
pub struct De105Key;

impl KeyboardLayout for De105Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
        match keycode {
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
            KeyCode::Oem8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('°')
                } else {
                    DecodedKey::Unicode('^')
                }
            }
            KeyCode::Key1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('!')
                } else {
                    DecodedKey::Unicode('1')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('§')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('$')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Key5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('5')
                }
            }
            KeyCode::Key6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('6')
                }
            }
            KeyCode::Key7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('/')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('=')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('ß')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('`')
                } else {
                    DecodedKey::Unicode('´')
                }
            }
            KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
            KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
            KeyCode::Q => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('@')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::E => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0005}')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('€')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::Y => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0014}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::Oem4 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Ü')
                } else {
                    DecodedKey::Unicode('ü')
                }
            }
            KeyCode::Oem6 => {
                if modifiers.is_altgr() {
                    DecodedKey::Unicode('~')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('+')
                }
            }
            KeyCode::Return => DecodedKey::Unicode(10.into()),
            KeyCode::Oem7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('\'')
                } else {
                    DecodedKey::Unicode('#')
                }
            }
            KeyCode::Oem1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('Ö')
                } else {
                    DecodedKey::Unicode('ö')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('Ä')
                } else {
                    DecodedKey::Unicode('ä')
                }
            }
            KeyCode::Z => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::OemComma => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(';')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::OemPeriod => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::Oem2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Oem5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('<')
                }
            }
            e => {
                let us = super::Us104Key;
                us.map_keycode(e, modifiers, handle_ctrl)
            }
        }
    }
}
