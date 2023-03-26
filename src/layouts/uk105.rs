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
                if modifiers.is_altgr() {
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
                if modifiers.is_altgr() {
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
    use crate::{EventDecoder, HandleControl, Keyboard, ScancodeSet, ScancodeSet1, ScancodeSet2};

    #[test]
    fn layout() {
        // Codes taken from https://kbdlayout.info/kbduk/overview+scancodes?arrangement=ISO105
        let mut s = ScancodeSet1::new();
        let mut dec = EventDecoder::new(Uk105Key, HandleControl::Ignore);
        let data = [
            (0x29, '`'),
            (0x02, '1'),
            (0x03, '2'),
            (0x04, '3'),
            (0x05, '4'),
            (0x06, '5'),
            (0x07, '6'),
            (0x08, '7'),
            (0x09, '8'),
            (0x0a, '9'),
            (0x0b, '0'),
            (0x0c, '-'),
            (0x0d, '='),
            (0x0f, '\t'),
            (0x10, 'q'),
            (0x11, 'w'),
            (0x12, 'e'),
            (0x13, 'r'),
            (0x14, 't'),
            (0x15, 'y'),
            (0x16, 'u'),
            (0x17, 'i'),
            (0x18, 'o'),
            (0x19, 'p'),
            (0x1a, '['),
            (0x1b, ']'),
            (0x1e, 'a'),
            (0x1f, 's'),
            (0x20, 'd'),
            (0x21, 'f'),
            (0x22, 'g'),
            (0x23, 'h'),
            (0x24, 'j'),
            (0x25, 'k'),
            (0x26, 'l'),
            (0x27, ';'),
            (0x28, '\''),
            (0x2B, '#'),
            (0x1c, '\n'),
            (0x56, '\\'),
            (0x2c, 'z'),
            (0x2d, 'x'),
            (0x2e, 'c'),
            (0x2f, 'v'),
            (0x30, 'b'),
            (0x31, 'n'),
            (0x32, 'm'),
            (0x33, ','),
            (0x34, '.'),
            (0x35, '/'),
        ];
        for (code, unicode) in data {
            let ev = s.advance_state(code).unwrap().unwrap();
            assert_eq!(Some(DecodedKey::Unicode(unicode)), dec.process_keyevent(ev));
        }
    }

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
