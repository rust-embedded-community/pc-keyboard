//! Implements the various keyboard layouts.
//!
//! We have one layout per file, but where two layouts are similar, you can
//! handle all the 'different' keys first, and then jump to another handler -
//! see [`Uk105Key`] and [`Us104Key`] as an example of that.

mod dvorak_programmer104;
pub use self::dvorak_programmer104::DVP104Key;

mod dvorak104;
pub use self::dvorak104::Dvorak104Key;

mod us104;
pub use self::us104::Us104Key;

mod uk105;
pub use self::uk105::Uk105Key;

mod jis109;
pub use self::jis109::Jis109Key;

mod azerty;
pub use self::azerty::Azerty;

mod colemak;
pub use self::colemak::Colemak;

mod de105;
pub use self::de105::De105Key;

mod no105;
pub use self::no105::No105Key;

mod fi_se105;
pub use self::fi_se105::FiSe105Key;

/// A enum of all the supported keyboard layouts.
pub enum AnyLayout {
    DVP104Key(DVP104Key),
    Dvorak104Key(Dvorak104Key),
    Us104Key(Us104Key),
    Uk105Key(Uk105Key),
    Jis109Key(Jis109Key),
    Azerty(Azerty),
    Colemak(Colemak),
    De105Key(De105Key),
    No105Key(No105Key),
    FiSe105Key(FiSe105Key),
}

impl super::KeyboardLayout for AnyLayout {
    fn map_keycode(
        &self,
        keycode: super::KeyCode,
        modifiers: &super::Modifiers,
        handle_ctrl: super::HandleControl,
    ) -> super::DecodedKey {
        match self {
            AnyLayout::DVP104Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Dvorak104Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Us104Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Uk105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Jis109Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Azerty(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Colemak(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::De105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::No105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::FiSe105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
        }
    }
}

impl super::KeyboardLayout for &AnyLayout {
    fn map_keycode(
        &self,
        keycode: super::KeyCode,
        modifiers: &super::Modifiers,
        handle_ctrl: super::HandleControl,
    ) -> super::DecodedKey {
        match self {
            AnyLayout::DVP104Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Dvorak104Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Us104Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Uk105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Jis109Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Azerty(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::Colemak(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::De105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::No105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
            AnyLayout::FiSe105Key(inner) => inner.map_keycode(keycode, modifiers, handle_ctrl),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn test_any() {
        let mut decoder = EventDecoder::new(AnyLayout::Uk105Key(Uk105Key), HandleControl::Ignore);
        // Q gets you a 'q'
        let decoded = decoder.process_keyevent(KeyEvent {
            code: KeyCode::Q,
            state: KeyState::Down,
        });
        assert_eq!(decoded, Some(DecodedKey::Unicode('q')));
        // Swap the layout
        decoder.change_layout(AnyLayout::Azerty(Azerty));
        // Q gets you a 'a'
        let decoded = decoder.process_keyevent(KeyEvent {
            code: KeyCode::Q,
            state: KeyState::Down,
        });
        assert_eq!(decoded, Some(DecodedKey::Unicode('a')));
    }
}
