//! Finnish/Swedish keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard Finnish/Swedish 102-key (or 105-key including Windows keys) keyboard.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
pub struct FiSe105Key;

impl KeyboardLayout for FiSe105Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
        let fallback = super::Us104Key;
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('½')
                } else {
                    DecodedKey::Unicode('§')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('#')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('£')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('¤')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('$')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Key5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('€')
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
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('[')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode(']')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('=')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('\\')
                } else {
                    DecodedKey::Unicode('+')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('`')
                } else {
                    DecodedKey::Unicode('´')
                }
            }
            // ========= Row 3 (QWERTY) =========
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
            KeyCode::Oem4 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Å')
                } else {
                    DecodedKey::Unicode('å')
                }
            }
            KeyCode::Oem6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('^')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('¨')
                }
            }
            // ========= Row 4 (ASDF) =========
            KeyCode::Oem1 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Ö')
                } else {
                    DecodedKey::Unicode('ö')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Ä')
                } else {
                    DecodedKey::Unicode('ä')
                }
            }
            KeyCode::Oem7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            // ========= Row 5 (ZXCV) =========
            KeyCode::Oem5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('<')
                }
            }
            KeyCode::M => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000D}')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('µ')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
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
            // ========= Row 6 (modifers and space bar) =========
            KeyCode::NumpadPeriod => {
                if modifiers.numlock {
                    DecodedKey::Unicode(',')
                } else {
                    fallback.map_keycode(keycode, modifiers, handle_ctrl)
                }
            }
            e => {
                let us = super::Us104Key;
                us.map_keycode(e, modifiers, handle_ctrl)
            }
        }
    }
}
