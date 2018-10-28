use super::{DecodedKey, Error, KeyboardLayout, KeyCode, Modifiers, ScancodeSet};
use super::scancodes::ScancodeSet2;

/// A standard United States 101-key (or 104-key including Windows keys) keyboard.
/// Has a 1-row high Enter key, with Backslash above.
pub struct Us104Key;

/// A standard United Kingdom 102-key (or 105-key including Windows keys) keyboard.
/// Has a 2-row high Enter key, with Backslash next to the left shift.
pub struct Uk105Key;

impl<S> KeyboardLayout<S> for Us104Key 
where 
    S: ScancodeSet
{
    fn map_scancode(code: u8) -> Result<KeyCode, Error> {
        S::map_scancode(code)
    }

    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
        S::map_extended_scancode(code)
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
            KeyCode::BracketSquareLeft => if modifiers.is_shifted() {
                DecodedKey::Unicode('{')
            } else {
                DecodedKey::Unicode('[')
            },
            KeyCode::BracketSquareRight => if modifiers.is_shifted() {
                DecodedKey::Unicode('}')
            } else {
                DecodedKey::Unicode(']')
            },
            KeyCode::BackSlash => if modifiers.is_shifted() {
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
            KeyCode::NumpadPlus => DecodedKey::Unicode('+'),
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

impl<S> KeyboardLayout<S> for Uk105Key 
where 
    S: ScancodeSet
{
    fn map_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x61 => Ok(KeyCode::BackSlash),
            _ => S::map_scancode(code),
        }
    }

    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            _ => S::map_extended_scancode(code),
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
            e => <Us104Key as KeyboardLayout<ScancodeSet2>>::map_keycode(e, modifiers),
        }
    }
}