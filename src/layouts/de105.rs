//! German keyboard support

use crate::{
    DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard, QUO, SLS,
};

/// A standard German 102-key (or 105-key including Windows keys) keyboard.
///
/// The top row spells `QWERTZ`.
///
/// Has a 2-row high Enter key, with Oem5 next to the left shift (ISO format).
pub struct De105Key;

impl KeyboardLayout for De105Key {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8      => modifiers.handle_shift('^', '°'),
            KeyCode::Key2      => modifiers.handle_altsh('2', '"', '²'),
            KeyCode::Key3      => modifiers.handle_altsh('3', '§', '³'),
            KeyCode::Key6      => modifiers.handle_shift('6', '&'),
            KeyCode::Key7      => modifiers.handle_altsh('7', '/', '{'),
            KeyCode::Key8      => modifiers.handle_altsh('8', '(', '['),
            KeyCode::Key9      => modifiers.handle_altsh('9', ')', ']'),
            KeyCode::Key0      => modifiers.handle_altsh('0', '=', '}'),
            KeyCode::OemMinus  => modifiers.handle_altsh('ß', '?', SLS),
            KeyCode::OemPlus   => modifiers.handle_shift('´', '`'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::Q         => modifiers.handle_alalt('Q', '@', '@', handle_ctrl),
            KeyCode::E         => modifiers.handle_alalt('E', '€', '€', handle_ctrl),
            KeyCode::Y         => modifiers.handle_alpha('Z', handle_ctrl),
            KeyCode::Oem4      => modifiers.handle_accen('ü', 'Ü'),
            KeyCode::Oem6      => modifiers.handle_altsh('+', '*', '~'),
            // ========= Row 4 (ASDFG) =========
            KeyCode::Oem1      => modifiers.handle_accen('ö', 'Ö'),
            KeyCode::Oem3      => modifiers.handle_accen('ä', 'Ä'),
            KeyCode::Oem7      => modifiers.handle_shift('#', QUO),
            // ========= Row 5 (ZXCVB) =========
            KeyCode::Oem5      => modifiers.handle_altsh('<', '>', '|'),
            KeyCode::Z         => modifiers.handle_alpha('Y', handle_ctrl),
            KeyCode::M         => modifiers.handle_alalt('M', 'µ', 'µ', handle_ctrl),
            KeyCode::OemComma  => modifiers.handle_shift(',', ';'),
            KeyCode::OemPeriod => modifiers.handle_shift('.', ':'),
            KeyCode::Oem2      => modifiers.handle_shift('-', '_'),
            // ========= Fallback =========
            e => super::Us104Key.map_keycode(e, modifiers, handle_ctrl),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Iso
    }
}
