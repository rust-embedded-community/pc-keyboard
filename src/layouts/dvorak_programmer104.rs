//! Dvorak Programmer keyboard support

use crate::{
    DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard, QUO, SLS,
};

/// A Dvorak Programmer 101-key (or 104-key including Windows keys) keyboard.
///
/// Has a 1-row high Enter key, with Oem5 above (ANSI layout).
pub struct DVP104Key;

impl KeyboardLayout for DVP104Key {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8      => modifiers.handle_shift('$', '~'),
            KeyCode::Key1      => modifiers.handle_shift('&', '%'),
            KeyCode::Key2      => modifiers.handle_shift('[', '7'),
            KeyCode::Key3      => modifiers.handle_shift('{', '5'),
            KeyCode::Key4      => modifiers.handle_shift('}', '3'),
            KeyCode::Key5      => modifiers.handle_shift('(', '1'),
            KeyCode::Key6      => modifiers.handle_shift('=', '9'),
            KeyCode::Key7      => modifiers.handle_shift('*', '0'),
            KeyCode::Key8      => modifiers.handle_shift(')', '2'),
            KeyCode::Key9      => modifiers.handle_shift('+', '4'),
            KeyCode::Key0      => modifiers.handle_shift(']', '6'),
            KeyCode::OemMinus  => modifiers.handle_shift('!', '8'),
            KeyCode::OemPlus   => modifiers.handle_shift('=', '`'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::Q         => modifiers.handle_shift(';', ':'),
            KeyCode::W         => modifiers.handle_shift(',', '<'),
            KeyCode::E         => modifiers.handle_shift('.', '>'),
            KeyCode::R         => modifiers.handle_alpha('P', handle_ctrl),
            KeyCode::T         => modifiers.handle_alpha('Y', handle_ctrl),
            KeyCode::Y         => modifiers.handle_alpha('F', handle_ctrl),
            KeyCode::U         => modifiers.handle_alpha('G', handle_ctrl),
            KeyCode::I         => modifiers.handle_alpha('C', handle_ctrl),
            KeyCode::O         => modifiers.handle_alpha('R', handle_ctrl),
            KeyCode::P         => modifiers.handle_alpha('L', handle_ctrl),
            KeyCode::Oem4      => modifiers.handle_shift('/', '?'),
            KeyCode::Oem6      => modifiers.handle_shift('@', '^'),
            KeyCode::Oem7      => modifiers.handle_shift(SLS, '|'),
            // ========= Row 4 (ASDFG) =========
            KeyCode::A         => modifiers.handle_alpha('A', handle_ctrl),
            KeyCode::S         => modifiers.handle_alpha('O', handle_ctrl),
            KeyCode::D         => modifiers.handle_alpha('E', handle_ctrl),
            KeyCode::F         => modifiers.handle_alpha('U', handle_ctrl),
            KeyCode::G         => modifiers.handle_alpha('I', handle_ctrl),
            KeyCode::H         => modifiers.handle_alpha('D', handle_ctrl),
            KeyCode::J         => modifiers.handle_alpha('H', handle_ctrl),
            KeyCode::K         => modifiers.handle_alpha('T', handle_ctrl),
            KeyCode::L         => modifiers.handle_alpha('N', handle_ctrl),
            KeyCode::Oem1      => modifiers.handle_alpha('S', handle_ctrl),
            KeyCode::Oem3      => modifiers.handle_shift('-', '_'),
            // ========= Row 5 (ZXCVB) =========
            KeyCode::Z         => modifiers.handle_shift(QUO, '"'),
            KeyCode::X         => modifiers.handle_alpha('Q', handle_ctrl),
            KeyCode::C         => modifiers.handle_alpha('J', handle_ctrl),
            KeyCode::V         => modifiers.handle_alpha('K', handle_ctrl),
            KeyCode::B         => modifiers.handle_alpha('X', handle_ctrl),
            KeyCode::N         => modifiers.handle_alpha('B', handle_ctrl),
            KeyCode::M         => modifiers.handle_alpha('M', handle_ctrl),
            KeyCode::OemComma  => modifiers.handle_alpha('W', handle_ctrl),
            KeyCode::OemPeriod => modifiers.handle_alpha('V', handle_ctrl),
            KeyCode::Oem2      => modifiers.handle_alpha('Z', handle_ctrl),
            // ========= Fallback =========
            e => super::Us104Key.map_keycode(e, modifiers, handle_ctrl),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Ansi
    }
}
