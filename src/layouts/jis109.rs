//! JIS keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard Japan 106-key (or 109-key including Windows keys) keyboard.
///
/// Has a small space bar, to fit in extra buttons.
pub struct Jis109Key;

impl KeyboardLayout for Jis109Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            KeyCode::Oem8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('`')
                } else {
                    DecodedKey::Unicode('@')
                }
            }
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
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
                    DecodedKey::Unicode('#')
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
                    DecodedKey::Unicode('\'')
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
                    DecodedKey::Unicode(' ')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('=')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::Oem4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('[')
                }
            }
            KeyCode::Oem6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode(']')
                }
            }
            KeyCode::Oem7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('\\')
                }
            }
            KeyCode::Oem1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode(':')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('^')
                }
            }
            e => {
                let us = super::Us104Key;
                us.map_keycode(e, modifiers, handle_ctrl)
            }
        }
    }
}
