//! United Kingdom keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard United Kingdom 102-key (or 105-key including Windows keys) keyboard.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
pub struct Uk105Key;

impl KeyboardLayout for Uk105Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            KeyCode::Oem8 => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('|')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('¬')
                } else {
                    DecodedKey::Unicode('`')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('£')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('€')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('$')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Oem7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('#')
                }
            }
            KeyCode::Oem5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('\\')
                }
            }
            e => {
                let us = super::Us104Key;
                us.map_keycode(e, modifiers, handle_ctrl)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{HandleControl, Keyboard, ScancodeSet2};

    #[test]
    fn test_hash() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            Uk105Key,
            HandleControl::MapLettersToUnicode,
        );
        // As seen on a UK 105 key Dell PS/2 keyboard when pressing `~#`
        let ev = k.add_byte(0x5D).unwrap().unwrap();
        let decoded_key = k.process_keyevent(ev);
        assert_eq!(decoded_key, Some(DecodedKey::Unicode('#')));
    }

    #[test]
    fn test_backslash() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            Uk105Key,
            HandleControl::MapLettersToUnicode,
        );
        // As seen on a UK 105 key Dell PS/2 keyboard when pressing `|\`
        let ev = k.add_byte(0x61).unwrap().unwrap();
        let decoded_key = k.process_keyevent(ev);
        assert_eq!(decoded_key, Some(DecodedKey::Unicode('\\')));
    }

    #[test]
    fn test_tilde() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            Uk105Key,
            HandleControl::MapLettersToUnicode,
        );
        // As seen on a UK 105 key Dell PS/2 keyboard when pressing Shift and `~#`
        let ev = k.add_byte(0x12).unwrap().unwrap();
        let _ = k.process_keyevent(ev);
        let ev = k.add_byte(0x5D).unwrap().unwrap();
        let decoded_key = k.process_keyevent(ev);
        assert_eq!(decoded_key, Some(DecodedKey::Unicode('~')));
    }

    #[test]
    fn test_pipe() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            Uk105Key,
            HandleControl::MapLettersToUnicode,
        );
        // As seen on a UK 105 key Dell PS/2 keyboard when pressing Shift and `|\`
        let ev = k.add_byte(0x12).unwrap().unwrap();
        let _ = k.process_keyevent(ev);
        let ev = k.add_byte(0x61).unwrap().unwrap();
        let decoded_key = k.process_keyevent(ev);
        assert_eq!(decoded_key, Some(DecodedKey::Unicode('|')));
    }
}
