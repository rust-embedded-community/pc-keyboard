//! A standard United Kingdom 102-key (or 105-key including Windows keys) keyboard.
//! Has a 2-row high Enter key, with Backslash next to the left shift.

use crate::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

pub use super::us104::Us104Key;

pub struct Uk105Key;

impl KeyboardLayout for Uk105Key {
    fn map_keycode(
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
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
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Quote => {
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
            KeyCode::HashTilde => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('#')
                }
            }
            e => <super::Us104Key as KeyboardLayout>::map_keycode(e, modifiers, handle_ctrl),
        }
    }
}