//! French keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard French 102-key (or 105-key including Windows keys) keyboard.
///
/// The top row spells `AZERTY`.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
pub struct Azerty;

impl KeyboardLayout for Azerty {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        let map_to_unicode = handle_ctrl == HandleControl::MapLettersToUnicode;
        match keycode {
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
            KeyCode::Oem8 => DecodedKey::Unicode('²'),
            KeyCode::Oem5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('<')
                }
            }
            KeyCode::Key1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('1')
                } else {
                    DecodedKey::Unicode('&')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('2')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('é')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('3')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('"')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('4')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::Key5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('5')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('[')
                } else {
                    DecodedKey::Unicode('(')
                }
            }
            KeyCode::Key6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('6')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Key7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('7')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('`')
                } else {
                    DecodedKey::Unicode('è')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('8')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('\\')
                } else {
                    DecodedKey::Unicode('_')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('9')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('ç')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('0')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('à')
                }
            }
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('°')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode(']')
                } else {
                    DecodedKey::Unicode(')')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
            KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
            KeyCode::Q => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0001}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::W => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
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
                    DecodedKey::Unicode('¨')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('ˇ')
                } else {
                    DecodedKey::Unicode('^')
                }
            }
            KeyCode::Oem6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('£')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('¤')
                } else {
                    DecodedKey::Unicode('$')
                }
            }
            KeyCode::Oem7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('µ')
                } else {
                    DecodedKey::Unicode('*')
                }
            }
            KeyCode::A => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
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
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000D}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('ù')
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Return => DecodedKey::Unicode(10.into()),
            KeyCode::Z => {
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0017}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
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
                if modifiers.is_caps() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::OemComma => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('.')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::OemPeriod => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('/')
                } else {
                    DecodedKey::Unicode(':')
                }
            }
            KeyCode::Oem2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('§')
                } else {
                    DecodedKey::Unicode('!')
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
            KeyCode::LShift => DecodedKey::Unicode('<'),
            k => DecodedKey::RawKey(k),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{KeyCode, KeyEvent, KeyState, Keyboard, ScancodeSet2};

    #[test]
    fn test_frazert() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            Azerty,
            HandleControl::MapLettersToUnicode,
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::NumpadDivide, KeyState::Down)),
            Some(DecodedKey::Unicode('/'))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::NumpadMultiply, KeyState::Down)),
            Some(DecodedKey::Unicode('*'))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Oem5, KeyState::Down)),
            Some(DecodedKey::Unicode('*'))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Down)),
            Some(DecodedKey::Unicode('q'))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Key4, KeyState::Down)),
            Some(DecodedKey::Unicode('\''))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Oem7, KeyState::Down)),
            Some(DecodedKey::Unicode('*'))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Up)),
            None
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::NumpadLock, KeyState::Down)),
            Some(DecodedKey::RawKey(KeyCode::NumpadLock))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::NumpadLock, KeyState::Up)),
            None
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Down)),
            Some(DecodedKey::RawKey(KeyCode::Insert))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Numpad0, KeyState::Up)),
            None
        );
    }
}
