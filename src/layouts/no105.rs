//! Norwegian keyboard support

use crate::{
    DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard, QUO, SLS,
};

/// A standard Norwegian 102-key (or 105-key including Windows keys) keyboard.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
pub struct No105Key;

impl KeyboardLayout for No105Key {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8      => modifiers.handle_shift('|', '§'),
            KeyCode::Key2      => modifiers.handle_altsh('2', '"', '@'),
            KeyCode::Key3      => modifiers.handle_altsh('3', '#', '£'),
            KeyCode::Key4      => modifiers.handle_altsh('4', '¤', '$'),
            KeyCode::Key5      => modifiers.handle_shift('5', '%'),
            KeyCode::Key6      => modifiers.handle_shift('6', '&'),
            KeyCode::Key7      => modifiers.handle_altsh('7', '/', '{'),
            KeyCode::Key8      => modifiers.handle_altsh('8', '(', '['),
            KeyCode::Key9      => modifiers.handle_altsh('9', ')', ']'),
            KeyCode::Key0      => modifiers.handle_altsh('0', '=', '}'),
            KeyCode::OemMinus  => modifiers.handle_shift('+', '?'),
            KeyCode::OemPlus   => modifiers.handle_altsh(SLS, '`', '´'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::E         => modifiers.handle_alalt('E', '€', '€', handle_ctrl),
            KeyCode::Oem4      => modifiers.handle_accen('å', 'Å'),
            KeyCode::Oem6      => modifiers.handle_altsh('¨', '^', '~'),
            // ========= Row 4 (ASDF) =========
            KeyCode::Oem7      => modifiers.handle_shift(QUO, '*'),
            KeyCode::Oem1      => modifiers.handle_accen('ø', 'Ø'),
            KeyCode::Oem3      => modifiers.handle_accen('æ', 'Æ'),
            // ========= Row 5 (ZXCV) =========
            KeyCode::Oem5      => modifiers.handle_shift('<', '>'),
            KeyCode::M         => modifiers.handle_alalt('M', 'µ', 'µ', handle_ctrl),
            KeyCode::OemComma  => modifiers.handle_shift(',', ';'),
            KeyCode::OemPeriod => modifiers.handle_shift('.', ':'),
            KeyCode::Oem2      => modifiers.handle_shift('-', '_'),
            KeyCode::NumpadPeriod if modifiers.numlock => DecodedKey::Unicode(','),
            // ========= Row 6 (modifers and space bar) =========
            e => super::Us104Key.map_keycode(e, modifiers, handle_ctrl),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Iso
    }
}
