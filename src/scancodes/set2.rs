//! Scan Code Set 2 support

use crate::{
    DecodeState, Error, KeyCode, KeyEvent, KeyState, ScancodeSet, EXTENDED2_KEY_CODE,
    EXTENDED_KEY_CODE, KEY_RELEASE_CODE,
};

/// Contains the implementation of Scancode Set 2.
///
/// See the OS dev wiki: <https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_2>
/// Additional reference: <https://www.win.tue.nl/~aeb/linux/kbd/scancodes-10.html>
pub struct ScancodeSet2 {
    state: DecodeState,
}

impl ScancodeSet2 {
    /// Construct a new [`ScancodeSet2`] decoder.
    pub const fn new() -> ScancodeSet2 {
        ScancodeSet2 {
            state: DecodeState::Start,
        }
    }

    /// Implements the single byte codes for Set 2.
    fn map_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x00 => Ok(KeyCode::TooManyKeys),
            0x01 => Ok(KeyCode::F9),
            // 0x02
            0x03 => Ok(KeyCode::F5),
            0x04 => Ok(KeyCode::F3),
            0x05 => Ok(KeyCode::F1),
            0x06 => Ok(KeyCode::F2),
            0x07 => Ok(KeyCode::F12),
            0x09 => Ok(KeyCode::F10),
            0x0A => Ok(KeyCode::F8),
            0x0B => Ok(KeyCode::F6),
            0x0C => Ok(KeyCode::F4),
            0x0D => Ok(KeyCode::Tab),
            0x0E => Ok(KeyCode::Oem8),
            0x11 => Ok(KeyCode::LAlt),
            0x12 => Ok(KeyCode::LShift),
            0x13 => Ok(KeyCode::Oem11),
            0x14 => Ok(KeyCode::LControl),
            0x15 => Ok(KeyCode::Q),
            0x16 => Ok(KeyCode::Key1),
            0x1A => Ok(KeyCode::Z),
            0x1B => Ok(KeyCode::S),
            0x1C => Ok(KeyCode::A),
            0x1D => Ok(KeyCode::W),
            0x1E => Ok(KeyCode::Key2),
            0x21 => Ok(KeyCode::C),
            0x22 => Ok(KeyCode::X),
            0x23 => Ok(KeyCode::D),
            0x24 => Ok(KeyCode::E),
            0x25 => Ok(KeyCode::Key4),
            0x26 => Ok(KeyCode::Key3),
            0x29 => Ok(KeyCode::Spacebar),
            0x2A => Ok(KeyCode::V),
            0x2B => Ok(KeyCode::F),
            0x2C => Ok(KeyCode::T),
            0x2D => Ok(KeyCode::R),
            0x2E => Ok(KeyCode::Key5),
            0x31 => Ok(KeyCode::N),
            0x32 => Ok(KeyCode::B),
            0x33 => Ok(KeyCode::H),
            0x34 => Ok(KeyCode::G),
            0x35 => Ok(KeyCode::Y),
            0x36 => Ok(KeyCode::Key6),
            0x3A => Ok(KeyCode::M),
            0x3B => Ok(KeyCode::J),
            0x3C => Ok(KeyCode::U),
            0x3D => Ok(KeyCode::Key7),
            0x3E => Ok(KeyCode::Key8),
            0x41 => Ok(KeyCode::OemComma),
            0x42 => Ok(KeyCode::K),
            0x43 => Ok(KeyCode::I),
            0x44 => Ok(KeyCode::O),
            0x45 => Ok(KeyCode::Key0),
            0x46 => Ok(KeyCode::Key9),
            0x49 => Ok(KeyCode::OemPeriod),
            0x4A => Ok(KeyCode::Oem2),
            0x4B => Ok(KeyCode::L),
            0x4C => Ok(KeyCode::Oem1),
            0x4D => Ok(KeyCode::P),
            0x4E => Ok(KeyCode::OemMinus),
            0x51 => Ok(KeyCode::Oem12),
            0x52 => Ok(KeyCode::Oem3),
            0x54 => Ok(KeyCode::Oem4),
            0x55 => Ok(KeyCode::OemPlus),
            0x58 => Ok(KeyCode::CapsLock),
            0x59 => Ok(KeyCode::RShift),
            0x5A => Ok(KeyCode::Return),
            0x5B => Ok(KeyCode::Oem6),
            0x5D => Ok(KeyCode::Oem7),
            0x61 => Ok(KeyCode::Oem5),
            0x64 => Ok(KeyCode::Oem10),
            0x66 => Ok(KeyCode::Backspace),
            0x67 => Ok(KeyCode::Oem9),
            0x69 => Ok(KeyCode::Numpad1),
            0x6A => Ok(KeyCode::Oem13),
            0x6B => Ok(KeyCode::Numpad4),
            0x6C => Ok(KeyCode::Numpad7),
            0x70 => Ok(KeyCode::Numpad0),
            0x71 => Ok(KeyCode::NumpadPeriod),
            0x72 => Ok(KeyCode::Numpad2),
            0x73 => Ok(KeyCode::Numpad5),
            0x74 => Ok(KeyCode::Numpad6),
            0x75 => Ok(KeyCode::Numpad8),
            0x76 => Ok(KeyCode::Escape),
            0x77 => Ok(KeyCode::NumpadLock),
            0x78 => Ok(KeyCode::F11),
            0x79 => Ok(KeyCode::NumpadAdd),
            0x7A => Ok(KeyCode::Numpad3),
            0x7B => Ok(KeyCode::NumpadSubtract),
            0x7C => Ok(KeyCode::NumpadMultiply),
            0x7D => Ok(KeyCode::Numpad9),
            0x7E => Ok(KeyCode::ScrollLock),
            0x7F => Ok(KeyCode::SysRq),
            0x83 => Ok(KeyCode::F7),
            0xAA => Ok(KeyCode::PowerOnTestOk),
            _ => Err(Error::UnknownKeyCode),
        }
    }

    /// Implements the extended byte codes for set 2 (prefixed with E0)
    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x11 => Ok(KeyCode::RAltGr),
            0x12 => Ok(KeyCode::RAlt2),
            0x14 => Ok(KeyCode::RControl),
            0x15 => Ok(KeyCode::PrevTrack),
            0x1F => Ok(KeyCode::LWin),
            0x21 => Ok(KeyCode::VolumeDown),
            0x23 => Ok(KeyCode::Mute),
            0x27 => Ok(KeyCode::RWin),
            0x2B => Ok(KeyCode::Calculator),
            0x2F => Ok(KeyCode::Apps),
            0x32 => Ok(KeyCode::VolumeUp),
            0x34 => Ok(KeyCode::Play),
            0x3A => Ok(KeyCode::WWWHome),
            0x3B => Ok(KeyCode::Stop),
            0x4A => Ok(KeyCode::NumpadDivide),
            0x4D => Ok(KeyCode::NextTrack),
            0x5A => Ok(KeyCode::NumpadEnter),
            0x69 => Ok(KeyCode::End),
            0x6B => Ok(KeyCode::ArrowLeft),
            0x6C => Ok(KeyCode::Home),
            0x70 => Ok(KeyCode::Insert),
            0x71 => Ok(KeyCode::Delete),
            0x72 => Ok(KeyCode::ArrowDown),
            0x74 => Ok(KeyCode::ArrowRight),
            0x75 => Ok(KeyCode::ArrowUp),
            0x7A => Ok(KeyCode::PageDown),
            0x7C => Ok(KeyCode::PrintScreen),
            0x7D => Ok(KeyCode::PageUp),
            _ => Err(Error::UnknownKeyCode),
        }
    }

    /// Implements the alternate extended byte codes for set 2 (prefixed with E1)
    fn map_extended2_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x14 => Ok(KeyCode::RControl2),
            _ => Err(Error::UnknownKeyCode),
        }
    }
}

impl ScancodeSet for ScancodeSet2 {
    /// Implements state logic for scancode set 2
    ///
    /// ## Start:
    /// * F0 => Goto Release
    /// * E0 => Goto Extended
    /// * E1 => Goto Extended2
    /// * xx => Key Down Event
    ///
    /// ## Release:
    /// * xxx => Key Up Event
    ///
    /// ## Extended:
    /// * F0 => Goto Release-Extended
    /// * xx => Extended Key Down Event
    ///
    /// ## Release-Extended:
    /// * xxx => Extended Key Up Event
    ///
    /// ## Extended2:
    /// * F0 => Goto Release-Extended2
    /// * xx => Extended2 Key Down Event
    ///
    /// ## Release-Extended2:
    /// * xxx => Extended2 Key Up Event
    fn advance_state(&mut self, code: u8) -> Result<Option<KeyEvent>, Error> {
        match self.state {
            DecodeState::Start => match code {
                EXTENDED_KEY_CODE => {
                    self.state = DecodeState::Extended;
                    Ok(None)
                }
                EXTENDED2_KEY_CODE => {
                    self.state = DecodeState::Extended2;
                    Ok(None)
                }
                KEY_RELEASE_CODE => {
                    self.state = DecodeState::Release;
                    Ok(None)
                }
                _ => {
                    let keycode = Self::map_scancode(code)?;
                    if keycode == KeyCode::TooManyKeys || keycode == KeyCode::PowerOnTestOk {
                        Ok(Some(KeyEvent::new(keycode, KeyState::SingleShot)))
                    } else {
                        Ok(Some(KeyEvent::new(
                            Self::map_scancode(code)?,
                            KeyState::Down,
                        )))
                    }
                }
            },
            DecodeState::Release => {
                self.state = DecodeState::Start;
                Ok(Some(KeyEvent::new(Self::map_scancode(code)?, KeyState::Up)))
            }
            DecodeState::Extended => match code {
                KEY_RELEASE_CODE => {
                    self.state = DecodeState::ExtendedRelease;
                    Ok(None)
                }
                _ => {
                    self.state = DecodeState::Start;

                    let keycode = Self::map_extended_scancode(code)?;
                    Ok(Some(KeyEvent::new(keycode, KeyState::Down)))
                }
            },
            DecodeState::ExtendedRelease => {
                self.state = DecodeState::Start;
                Ok(Some(KeyEvent::new(
                    Self::map_extended_scancode(code)?,
                    KeyState::Up,
                )))
            }
            DecodeState::Extended2 => match code {
                KEY_RELEASE_CODE => {
                    self.state = DecodeState::Extended2Release;
                    Ok(None)
                }
                _ => {
                    self.state = DecodeState::Start;
                    Ok(Some(KeyEvent::new(
                        Self::map_extended2_scancode(code)?,
                        KeyState::Down,
                    )))
                }
            },
            DecodeState::Extended2Release => {
                self.state = DecodeState::Start;
                Ok(Some(KeyEvent::new(
                    Self::map_extended2_scancode(code)?,
                    KeyState::Up,
                )))
            }
        }
    }
}

impl Default for ScancodeSet2 {
    fn default() -> Self {
        ScancodeSet2::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_scancodes() {
        let mut codes = Vec::new();
        let mut errs = Vec::new();
        for code in 0x00..=0xFF {
            let r = ScancodeSet2::map_scancode(code);
            match r {
                Ok(c) => codes.push(c),
                Err(_) => errs.push(code),
            }
        }
        codes.sort();
        println!("{:?}", codes);
        assert_eq!(codes.len(), 94);
        assert_eq!(errs.len(), 162);
    }
}
