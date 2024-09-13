//! United States keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard United States 101-key (or 104-key including Windows keys) keyboard.
///
/// Has a 1-row high Enter key, with Oem5 above (ANSI layout).
pub struct Us104Key;

impl KeyboardLayout for Us104Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
        match keycode {
            KeyCode::Oem8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('`')
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
                    DecodedKey::Unicode('@')
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
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('6')
                }
            }
            KeyCode::Key7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
            KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
            KeyCode::Q => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::W => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0017}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::E => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0005}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::R => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0012}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                }
            }
            KeyCode::T => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0014}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                }
            }
            KeyCode::Y => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0019}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::U => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0015}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                }
            }
            KeyCode::I => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0009}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                }
            }
            KeyCode::O => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000F}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                }
            }
            KeyCode::P => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0010}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
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
            KeyCode::A => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0001}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::S => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0013}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                }
            }
            KeyCode::D => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0004}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                }
            }
            KeyCode::F => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0006}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                }
            }
            KeyCode::G => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0007}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                }
            }
            KeyCode::H => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0008}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                }
            }
            KeyCode::J => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                }
            }
            KeyCode::K => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000B}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                }
            }
            KeyCode::L => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000C}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                }
            }
            KeyCode::Oem1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Return => DecodedKey::Unicode(10.into()),
            KeyCode::Z => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::X => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0018}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                }
            }
            KeyCode::C => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0003}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                }
            }
            KeyCode::V => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0016}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                }
            }
            KeyCode::B => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0002}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                }
            }
            KeyCode::N => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000E}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                }
            }
            KeyCode::M => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000D}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::OemComma => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('<')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::OemPeriod => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::Oem2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('/')
                }
            }
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            KeyCode::Delete => DecodedKey::Unicode(127.into()),
            KeyCode::NumpadDivide => DecodedKey::Unicode('/'),
            KeyCode::NumpadMultiply => DecodedKey::Unicode('*'),
            KeyCode::NumpadSubtract => DecodedKey::Unicode('-'),
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
                    DecodedKey::RawKey(KeyCode::ArrowUp)
                }
            }
            KeyCode::Numpad9 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('9')
                } else {
                    DecodedKey::RawKey(KeyCode::PageUp)
                }
            }
            KeyCode::NumpadAdd => DecodedKey::Unicode('+'),
            KeyCode::Numpad4 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('4')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowLeft)
                }
            }
            KeyCode::Numpad5 => DecodedKey::Unicode('5'),
            KeyCode::Numpad6 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('6')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowRight)
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
                    DecodedKey::RawKey(KeyCode::ArrowDown)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{EventDecoder, ScancodeSet, ScancodeSet1};

    #[test]
    fn layout() {
        // Codes taken from https://kbdlayout.info/kbdus/overview+scancodes?arrangement=ANSI104
        let mut s = ScancodeSet1::new();
        let mut dec = EventDecoder::new(Us104Key, HandleControl::Ignore);
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
            (0x2b, '\\'),
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
            (0x1c, '\n'),
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
}
