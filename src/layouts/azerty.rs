//! French keyboard support

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

/// A standard French 102-key (or 105-key including Windows keys) keyboard.
///
/// The top row spells `AZERTY`.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
///
/// NB: no "dead key" support for now
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
            // Works with Unicode & 850 code page, not 437 that has neither ¹ or ³
            KeyCode::Oem8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('³')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('¹')
                } else {
                    DecodedKey::Unicode('²')
                }
            }
            // Works with Unicode, 437 & 850 code pages
            KeyCode::Oem5 => {
                if modifiers.is_shifted() {
                    if modifiers.is_altgr() {
                        DecodedKey::Unicode('≥')
                    } else {
                        DecodedKey::Unicode('>')
                    }
                } else {
                    if modifiers.is_altgr() {
                        DecodedKey::Unicode('≤')
                    } else {
                        DecodedKey::Unicode('<')
                    }
                }
            }
            KeyCode::Key1 => {
                // NB: ˇ & ˛ dead keys with AltGr (+ Shift)
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('1')
                } else {
                    DecodedKey::Unicode('&')
                }
            }
            KeyCode::Key2 => {
                // NB: É can be done with AltGr + Shift
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('2')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('é')
                }
            }
            KeyCode::Key3 => {
                // NB: ˘ dead key with AltGr + Shift
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('3')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('"')
                }
            }
            KeyCode::Key4 => {
                // NB: — (not -) can be done with AltGr + Shift, but is Unicode only
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('4')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::Key5 => {
                // NB: – (not -) can be done with AltGr + Shift, but is Unicode only
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('5')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('[')
                } else {
                    DecodedKey::Unicode('(')
                }
            }
            KeyCode::Key6 => {
                // NB: ‑ (not -) can be done with AltGr + Shift, but is Unicode only
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('6')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::Key7 => {
                // NB: È can be done with AltGr + Shift
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('7')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('`')
                } else {
                    DecodedKey::Unicode('è')
                }
            }
            KeyCode::Key8 => {
                // NB: ™ can be done with AltGr + Shift
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('8')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('\\')
                } else {
                    DecodedKey::Unicode('_')
                }
            }
            KeyCode::Key9 => {
                // NB: Ç can be done with AltGr + Shift
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('9')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('ç')
                }
            }
            KeyCode::Key0 => {
                // NB: À can be done with AltGr + Shift
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('0')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('à')
                }
            }
            KeyCode::OemMinus => {
                // NB: ≠ can be done with AltGr + Shift, but is Unicode only
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('°')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode(']')
                } else {
                    DecodedKey::Unicode(')')
                }
            }
            KeyCode::OemPlus => {
                // NB: ± can be done with AltGr + Shift
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
                // NB: æ & Æ can be done with AltGr (+ Shift)
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0001}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::W => {
                // NB: â & Â can be done with AltGr (+ Shift), but no Â in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{001A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::E => {
                // NB: € & ¢ can be done with AltGr (+ Shift), but not with code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0005}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::R => {
                // NB: ê & Ê can be done with AltGr (+ Shift), but no Ê in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0012}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                }
            }
            KeyCode::T => {
                // NB: þ & Þ can be done with AltGr (+ Shift), but not with code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0014}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                }
            }
            KeyCode::Y => {
                // NB: ÿ & Ÿ can be done with AltGr (+ Shift), but no Ÿ in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0019}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::U => {
                // NB: û & Û can be done with AltGr (+ Shift), but no Û in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0015}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                }
            }
            KeyCode::I => {
                // NB: î & Î can be done with AltGr (+ Shift), but no Î in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0009}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                }
            }
            KeyCode::O => {
                // NB: œ & Œ can be done with AltGr (+ Shift), but not with code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000F}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                }
            }
            KeyCode::P => {
                // NB: ô & Ô can be done with AltGr (+ Shift), but no Ô in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0010}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
                }
            }
            KeyCode::Oem4 => {
                // NB: these should be dead keys
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('¨')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('ˇ')
                } else {
                    DecodedKey::Unicode('^')
                }
            }
            KeyCode::Oem6 => {
                // NB: ø & Ø can be done with AltGr (+ Shift), but not with code page 437
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('£')
                } else if modifiers.is_altgr() {
                    DecodedKey::Unicode('¤')
                } else {
                    DecodedKey::Unicode('$')
                }
            }
            KeyCode::Oem7 => {
                // NB: ´ & ¯ dead keys can be done with AltGr (+ Shift)
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('µ')
                } else {
                    DecodedKey::Unicode('*')
                }
            }
            KeyCode::A => {
                // NB: ä & Ä can be done with AltGr (+ Shift)
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0011}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::S => {
                // NB: ß & „ can be done with AltGr (+ Shift), but „ is Unicode only
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0013}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                }
            }
            KeyCode::D => {
                // NB: ë & Ë can be done with AltGr (+ Shift), but no Ë in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0004}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                }
            }
            KeyCode::F => {
                // NB: ‘ & ‚ can be done with AltGr (+ Shift), but are Unicode only
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0006}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                }
            }
            KeyCode::G => {
                // NB: ’ & ¥ can be done with AltGr (+ Shift), but no ’ in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0007}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                }
            }
            KeyCode::H => {
                // NB: ð & Ð can be done with AltGr (+ Shift), but not with code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0008}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                }
            }
            KeyCode::J => {
                // NB: ü & Ü can be done with AltGr (+ Shift)
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000A}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                }
            }
            KeyCode::K => {
                // NB: ï & Ï can be done with AltGr (+ Shift), but no Ï in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000B}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                }
            }
            KeyCode::L => {
                // NB: ŀ & Ŀ can be done with AltGr (+ Shift), but are Unicode only
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000C}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                }
            }
            KeyCode::Oem1 => {
                // NB: ö & Ö can be done with AltGr (+ Shift)
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000D}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::Oem3 => {
                // NB: ´ dead key & Ù can be done with AltGr (+ Shift), but no Ù in code page 437
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('ù')
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Return => DecodedKey::Unicode(10.into()),
            KeyCode::Z => {
                // NB: « & “ can be done with AltGr (+ Shift), but no “ in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0017}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::X => {
                // NB: » & ” can be done with AltGr (+ Shift), but no ” in code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0018}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                }
            }
            KeyCode::C => {
                // NB: © & ® can be done with AltGr (+ Shift), but not with code page 437
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0003}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                }
            }
            KeyCode::V => {
                // NB: ' ' & ← can be done with AltGr (+ Shift), but '' is Unicode NARROW UNBREAKABLE SPACE
                //                                               and ← is 0x1B with 437 and 850 code pages
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0016}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                }
            }
            KeyCode::B => {
                // NB: ↓ & ↑ can be done with AltGr (+ Shift), but are 0x19 & 0x18 with 437 and 850 code pages
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{0002}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                }
            }
            KeyCode::N => {
                // NB: ¬ & → can be done with AltGr (+ Shift), but → is 0x1A with 437 and 850 code pages
                if map_to_unicode && modifiers.is_ctrl() {
                    DecodedKey::Unicode('\u{000E}')
                } else if modifiers.is_caps() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                }
            }
            KeyCode::M => {
                // NB: ¿ & … can be done with AltGr (+ Shift), but no … in code page 437
                if modifiers.is_caps() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::OemComma => {
                // NB: × & ⋅ can be done with AltGr (+ Shift), but not with code page 437
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('.')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::OemPeriod => {
                // NB: ÷ & ∕ can be done with AltGr (+ Shift), but ∕ is Unicode only
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('/')
                } else {
                    DecodedKey::Unicode(':')
                }
            }
            KeyCode::Oem2 => {
                // NB: ¡ & − (not -) can be done with AltGr (+ Shift), but no − in code page 437
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('§')
                } else {
                    DecodedKey::Unicode('!')
                }
            }
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            KeyCode::Delete => DecodedKey::Unicode(127.into()),
            // NB: these ones give respectively ÷, × & − with AltGr
            KeyCode::NumpadDivide => DecodedKey::Unicode('/'),
            KeyCode::NumpadMultiply => DecodedKey::Unicode('*'),
            KeyCode::NumpadSubtract => DecodedKey::Unicode('-'),
            // NB: this is interesting with AltGr or AltGr+Shift, but Unicode only
            // 7: ↖⇖ 8: ↑⇑ 9:↗⇗
            // 4: ←⇐ 5: ↔⇔ 6:→⇒
            // 1: ↙⇙ 2: ↓⇓ 3:↘⇘
            // 0: ↕⇕ .: , (space: e2 80 af in UTF-8)
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
            // NB: this one gives nothing different with AltGr
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
            k.process_keyevent(KeyEvent::new(KeyCode::A, KeyState::Down)),
            Some(DecodedKey::Unicode('q'))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Key4, KeyState::Down)),
            Some(DecodedKey::Unicode('\''))
        );
        assert_eq!(
            k.process_keyevent(KeyEvent::new(KeyCode::Oem5, KeyState::Down)),
            Some(DecodedKey::Unicode('<'))
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
