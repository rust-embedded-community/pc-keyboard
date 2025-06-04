//! # United States keyboard support

use crate::{
    DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers, PhysicalKeyboard, QUO, SLS,
};

/// A standard United States 101-key (or 104-key including Windows keys) keyboard.
///
/// Has a 1-row high Enter key, with Oem5 above (ANSI layout).
///
/// These diagrams illustrate the conversion from [`KeyCode`] to Unicode. We
/// show either a Unicode glyph, or a hex number if the glyph isn't a
/// printable character. Blank spaces are passed through as
/// [`DecodedKey::RawKey`]. We also show Raw outputs on keys that are
/// sometimes Unicode and sometimes not, depending on modifiers.
///
/// Run the `print_keyboard` example to re-generate these images.
///
/// ## Unmodified
///
/// ```text
/// ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
/// │001b│  │    │    │    │    │  │    │    │    │    │  │    │    │    │    │   │    │    │    │
/// └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
///
/// ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
/// │ `  │ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │ 9  │ 0  │ -  │ =  │0008     │  │    │    │    │  │    │ /  │ *  │ -  │
/// ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
/// │0009 │ q  │ w  │ e  │ r  │ t  │ y  │ u  │ i  │ o  │ p  │ [  │ ]  │   \    │  │007f│    │    │  │ 7  │ 8  │ 9  │    │
/// ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤ +  │
/// │      │ a  │ s  │ d  │ f  │ g  │ h  │ j  │ k  │ l  │ ;  │ '  │   000a     │                    │ 4  │ 5  │ 6  │    │
/// ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
/// │         │ z  │ x  │ c  │ v  │ b  │ n  │ m  │ ,  │ .  │ /  │              │       │    │       │ 1  │ 2  │ 3  │    │
/// ├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤000a│
/// │     │     │     │             0020             │     │     │      │      │  │    │    │    │  │ 0       │ .  │    │
/// └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
/// ```
///
/// ## Shifted
///
/// ```text
/// ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
/// │001b│  │    │    │    │    │  │    │    │    │    │  │    │    │    │    │   │    │    │    │
/// └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
///
/// ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
/// │ ~  │ !  │ @  │ #  │ $  │ %  │ ^  │ &  │ *  │ (  │ )  │ _  │ +  │0008     │  │    │    │    │  │    │ /  │ *  │ -  │
/// ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
/// │0009 │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │ {  │ }  │   |    │  │007f│    │    │  │ 7  │ 8  │ 9  │    │
/// ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤ +  │
/// │      │ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │ :  │ "  │   000a     │                    │ 4  │ 5  │ 6  │    │
/// ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
/// │         │ Z  │ X  │ C  │ V  │ B  │ N  │ M  │ <  │ >  │ ?  │              │       │    │       │ 1  │ 2  │ 3  │    │
/// ├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤000a│
/// │     │     │     │             0020             │     │     │      │      │  │    │    │    │  │ 0       │ .  │    │
/// └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
/// ```
///
/// ## Caps Lock
///
/// ```text
/// ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
/// │001b│  │    │    │    │    │  │    │    │    │    │  │    │    │    │    │   │    │    │    │
/// └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
///
/// ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
/// │ `  │ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │ 9  │ 0  │ -  │ =  │0008     │  │    │    │    │  │    │ /  │ *  │ -  │
/// ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
/// │0009 │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │ [  │ ]  │   \    │  │007f│    │    │  │ 7  │ 8  │ 9  │    │
/// ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤ +  │
/// │      │ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │ ;  │ '  │   000a     │                    │ 4  │ 5  │ 6  │    │
/// ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
/// │         │ Z  │ X  │ C  │ V  │ B  │ N  │ M  │ ,  │ .  │ /  │              │       │    │       │ 1  │ 2  │ 3  │    │
/// ├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤000a│
/// │     │     │     │             0020             │     │     │      │      │  │    │    │    │  │ 0       │ .  │    │
/// └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
/// ```
///
/// ## Control
///
/// ```text
/// ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
/// │001b│  │    │    │    │    │  │    │    │    │    │  │    │    │    │    │   │    │    │    │
/// └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
///
/// ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
/// │ `  │ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │ 9  │ 0  │ -  │ =  │0008     │  │    │    │    │  │    │ /  │ *  │ -  │
/// ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
/// │0009 │0011│0017│0005│0012│0014│0019│0015│0009│000f│0010│ [  │ ]  │   \    │  │007f│    │    │  │ 7  │ 8  │ 9  │    │
/// ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤ +  │
/// │      │0001│0013│0004│0006│0007│0008│000a│000b│000c│ ;  │ '  │   000a     │                    │ 4  │ 5  │ 6  │    │
/// ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
/// │         │001a│0018│0003│0016│0002│000e│000d│ ,  │ .  │ /  │              │       │    │       │ 1  │ 2  │ 3  │    │
/// ├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤000a│
/// │     │     │     │             0020             │     │     │      │      │  │    │    │    │  │ 0       │ .  │    │
/// └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
/// ```
///
pub struct Us104Key;

impl KeyboardLayout for Us104Key {
    #[rustfmt::skip]
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            // ========= Row 2 (the numbers) =========
            KeyCode::Oem8            => modifiers.handle_shift('`', '~'),
            KeyCode::Escape          => DecodedKey::Unicode('\u{001B}'),
            KeyCode::Key1            => modifiers.handle_shift('1', '!'),
            KeyCode::Key2            => modifiers.handle_shift('2', '@'),
            KeyCode::Key3            => modifiers.handle_shift('3', '#'),
            KeyCode::Key4            => modifiers.handle_shift('4', '$'),
            KeyCode::Key5            => modifiers.handle_shift('5', '%'),
            KeyCode::Key6            => modifiers.handle_shift('6', '^'),
            KeyCode::Key7            => modifiers.handle_shift('7', '&'),
            KeyCode::Key8            => modifiers.handle_shift('8', '*'),
            KeyCode::Key9            => modifiers.handle_shift('9', '('),
            KeyCode::Key0            => modifiers.handle_shift('0', ')'),
            KeyCode::OemMinus        => modifiers.handle_shift('-', '_'),
            KeyCode::OemPlus         => modifiers.handle_shift('=', '+'),
            KeyCode::Backspace       => DecodedKey::Unicode('\u{0008}'),
            // ========= Row 3 (QWERTY) =========
            KeyCode::Tab             => DecodedKey::Unicode('\u{0009}'),
            KeyCode::Q               => modifiers.handle_alpha('Q', handle_ctrl),
            KeyCode::W               => modifiers.handle_alpha('W', handle_ctrl),
            KeyCode::E               => modifiers.handle_alpha('E', handle_ctrl),
            KeyCode::R               => modifiers.handle_alpha('R', handle_ctrl),
            KeyCode::T               => modifiers.handle_alpha('T', handle_ctrl),
            KeyCode::Y               => modifiers.handle_alpha('Y', handle_ctrl),
            KeyCode::U               => modifiers.handle_alpha('U', handle_ctrl),
            KeyCode::I               => modifiers.handle_alpha('I', handle_ctrl),
            KeyCode::O               => modifiers.handle_alpha('O', handle_ctrl),
            KeyCode::P               => modifiers.handle_alpha('P', handle_ctrl),
            KeyCode::Oem4            => modifiers.handle_shift('[', '{'),
            KeyCode::Oem6            => modifiers.handle_shift(']', '}'),
            KeyCode::Oem7            => modifiers.handle_shift(SLS, '|'),
            // ========= Row 4 (ASDFG) =========
            KeyCode::A               => modifiers.handle_alpha('A', handle_ctrl),
            KeyCode::S               => modifiers.handle_alpha('S', handle_ctrl),
            KeyCode::D               => modifiers.handle_alpha('D', handle_ctrl),
            KeyCode::F               => modifiers.handle_alpha('F', handle_ctrl),
            KeyCode::G               => modifiers.handle_alpha('G', handle_ctrl),
            KeyCode::H               => modifiers.handle_alpha('H', handle_ctrl),
            KeyCode::J               => modifiers.handle_alpha('J', handle_ctrl),
            KeyCode::K               => modifiers.handle_alpha('K', handle_ctrl),
            KeyCode::L               => modifiers.handle_alpha('L', handle_ctrl),
            KeyCode::Oem1            => modifiers.handle_shift(';', ':'),
            KeyCode::Oem3            => modifiers.handle_shift(QUO, '"'),
            KeyCode::Return          => DecodedKey::Unicode('\u{000A}'),
            // ========= Row 5 (ZXCVB) =========
            KeyCode::Z               => modifiers.handle_alpha('Z', handle_ctrl),
            KeyCode::X               => modifiers.handle_alpha('X', handle_ctrl),
            KeyCode::C               => modifiers.handle_alpha('C', handle_ctrl),
            KeyCode::V               => modifiers.handle_alpha('V', handle_ctrl),
            KeyCode::B               => modifiers.handle_alpha('B', handle_ctrl),
            KeyCode::N               => modifiers.handle_alpha('N', handle_ctrl),
            KeyCode::M               => modifiers.handle_alpha('M', handle_ctrl),
            KeyCode::OemComma        => modifiers.handle_shift(',', '<'),
            KeyCode::OemPeriod       => modifiers.handle_shift('.', '>'),
            KeyCode::Oem2            => modifiers.handle_shift('/', '?'),
            // ========= Unicode Specials =========
            KeyCode::Spacebar        => DecodedKey::Unicode(' '),
            KeyCode::Delete          => DecodedKey::Unicode('\u{007f}'),
            // ========= Numpad =========
            KeyCode::NumpadDivide    => DecodedKey::Unicode('/'),
            KeyCode::NumpadMultiply  => DecodedKey::Unicode('*'),
            KeyCode::NumpadSubtract  => DecodedKey::Unicode('-'),
            KeyCode::Numpad7         => modifiers.handle_numpad('7', KeyCode::Home),
            KeyCode::Numpad8         => modifiers.handle_numpad('8', KeyCode::ArrowUp),
            KeyCode::Numpad9         => modifiers.handle_numpad('9', KeyCode::PageUp),
            KeyCode::NumpadAdd       => DecodedKey::Unicode('+'),
            KeyCode::Numpad4         => modifiers.handle_numpad('4', KeyCode::ArrowLeft),
            KeyCode::Numpad5         => DecodedKey::Unicode('5'),
            KeyCode::Numpad6         => modifiers.handle_numpad('6', KeyCode::ArrowRight),
            KeyCode::Numpad1         => modifiers.handle_numpad('1', KeyCode::End),
            KeyCode::Numpad2         => modifiers.handle_numpad('2', KeyCode::ArrowDown),
            KeyCode::Numpad3         => modifiers.handle_numpad('3', KeyCode::PageDown),
            KeyCode::Numpad0         => modifiers.handle_numpad('0', KeyCode::Insert),
            KeyCode::NumpadPeriod    => modifiers.handle_numpad_pair('.', '\u{007f}'),
            KeyCode::NumpadEnter     => DecodedKey::Unicode('\u{000A}'),
            // ========= Fallback =========
            k                        => DecodedKey::RawKey(k),
        }
    }

    fn get_physical(&self) -> PhysicalKeyboard {
        PhysicalKeyboard::Ansi
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

    #[test]
    fn lowercase() {
        let modifiers = Modifiers {
            capslock: false,
            lalt: false,
            lctrl: false,
            lshift: false,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_alpha('A', HandleControl::MapLettersToUnicode),
            DecodedKey::Unicode('a')
        );
    }

    #[test]
    fn uppercase() {
        let modifiers = Modifiers {
            capslock: true,
            lalt: false,
            lctrl: false,
            lshift: false,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_alpha('A', HandleControl::MapLettersToUnicode),
            DecodedKey::Unicode('A')
        );
    }

    #[test]
    fn shifted() {
        let modifiers = Modifiers {
            capslock: false,
            lalt: false,
            lctrl: false,
            lshift: true,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_alpha('A', HandleControl::MapLettersToUnicode),
            DecodedKey::Unicode('A')
        );
    }

    #[test]
    fn shift_caps() {
        let modifiers = Modifiers {
            capslock: true,
            lalt: false,
            lctrl: false,
            lshift: true,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_alpha('A', HandleControl::MapLettersToUnicode),
            DecodedKey::Unicode('a')
        );
    }

    #[test]
    fn ctrl() {
        let modifiers = Modifiers {
            capslock: true,
            lalt: false,
            lctrl: true,
            lshift: true,
            numlock: false,
            ralt: false,
            rctrl: false,
            rctrl2: false,
            rshift: false,
        };
        assert_eq!(
            modifiers.handle_alpha('A', HandleControl::MapLettersToUnicode),
            DecodedKey::Unicode('\u{0001}')
        );
    }
}
