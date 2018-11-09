use super::{ConsumeState, DecodeState, Error, EXTENDED_KEY_CODE, KeyCode, KEY_RELEASE_CODE, ScancodeSet};

/// Contains the implementation of Scancode Set 1. 
/// See the OS dev wiki: https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1
pub struct ScancodeSet1;

impl ScancodeSet for ScancodeSet1 {
    /// Implements state logic for scancode set 1
    fn advance_state(state: DecodeState, code: u8) -> Result<ConsumeState, Error> {
        match state {
            DecodeState::Start => {
                match code {
                    EXTENDED_KEY_CODE => {
                        Ok(ConsumeState::Consume(DecodeState::Extended))
                    }, 
                    0x81..=0xD8 => {
                        Ok(ConsumeState::Proceed(DecodeState::Release))
                    },
                    _ => Ok(ConsumeState::Proceed(DecodeState::Start))
                }
            }, 
            DecodeState::Extended => {
                match code {
                    0x90..=0xED => {
                        Ok(ConsumeState::Proceed(DecodeState::ExtendedRelease))
                    }, 
                    _ => Ok(ConsumeState::Proceed(DecodeState::Extended))
                }
            },
            DecodeState::Release => Ok(ConsumeState::Proceed(DecodeState::Release)), 
            DecodeState::ExtendedRelease => Ok(ConsumeState::Proceed(DecodeState::ExtendedRelease))
        }
    }

    /// Implements the single byte codes for Set 1.
    fn map_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x01 => Ok(KeyCode::Escape),             // 01
            0x02 => Ok(KeyCode::Key1),               // 02
            0x03 => Ok(KeyCode::Key2),               // 03
            0x04 => Ok(KeyCode::Key3),               // 04
            0x05 => Ok(KeyCode::Key4),               // 05
            0x06 => Ok(KeyCode::Key5),               // 06
            0x07 => Ok(KeyCode::Key6),               // 07
            0x08 => Ok(KeyCode::Key7),               // 08
            0x09 => Ok(KeyCode::Key8),               // 09
            0x0A => Ok(KeyCode::Key9),               // 0A
            0x0B => Ok(KeyCode::Key0),               // 0B
            0x0C => Ok(KeyCode::Minus),              // 0C
            0x0D => Ok(KeyCode::Equals),             // 0D
            0x0E => Ok(KeyCode::Backspace),          // 0E
            0x10 => Ok(KeyCode::Q),                  // 10
            0x11 => Ok(KeyCode::W),                  // 11
            0x12 => Ok(KeyCode::E),                  // 12
            0x13 => Ok(KeyCode::R),                  // 13
            0x14 => Ok(KeyCode::T),                  // 14
            0x15 => Ok(KeyCode::Y),                  // 15
            0x16 => Ok(KeyCode::U),                  // 16
            0x17 => Ok(KeyCode::I),                  // 17
            0x18 => Ok(KeyCode::O),                  // 18
            0x19 => Ok(KeyCode::P),                  // 19
            0x1A => Ok(KeyCode::BracketSquareLeft),  // 1A  
            0x1B => Ok(KeyCode::BracketSquareRight), // 1B
            0x1C => Ok(KeyCode::Enter),              // 1C
            0x1D => Ok(KeyCode::ControlLeft),        // 1D
            0x1E => Ok(KeyCode::A),                  // 1E
            0x1F => Ok(KeyCode::S),                  // 1F
            0x20 => Ok(KeyCode::D),                  // 20
            0x21 => Ok(KeyCode::F),                  // 21
            0x22 => Ok(KeyCode::G),                  // 22
            0x23 => Ok(KeyCode::H),                  // 23
            0x24 => Ok(KeyCode::J),                  // 24
            0x25 => Ok(KeyCode::K),                  // 25
            0x26 => Ok(KeyCode::L),                  // 26
            0x27 => Ok(KeyCode::SemiColon),          // 27
            0x28 => Ok(KeyCode::Quote),              // 28
            0x29 => Ok(KeyCode::BackTick),           // 29
            0x2A => Ok(KeyCode::ShiftLeft),          // 2A
            0x2B => Ok(KeyCode::BackSlash),          // 2B 
            0x2C => Ok(KeyCode::Z),                  // 2C
            0x2D => Ok(KeyCode::X),                  // 2D
            0x2E => Ok(KeyCode::C),                  // 2E
            0x2F => Ok(KeyCode::V),                  // 2F
            0x30 => Ok(KeyCode::B),                  // 30
            0x31 => Ok(KeyCode::N),                  // 31
            0x32 => Ok(KeyCode::M),                  // 32
            0x33 => Ok(KeyCode::Comma),              // 33
            0x34 => Ok(KeyCode::Fullstop),           // 34
            0x35 => Ok(KeyCode::Slash),              // 35 
            0x36 => Ok(KeyCode::ShiftRight),         // 36
            0x37 => Ok(KeyCode::NumpadStar),         // 37
            0x38 => Ok(KeyCode::AltLeft),            // 38
            0x39 => Ok(KeyCode::Spacebar),           // 39
            0x3A => Ok(KeyCode::CapsLock),           // 3A
            0x3B => Ok(KeyCode::F1),                 // 3B
            0x3C => Ok(KeyCode::F2),                 // 3C
            0x3D => Ok(KeyCode::F3),                 // 3D
            0x3E => Ok(KeyCode::F4),                 // 3E
            0x3F => Ok(KeyCode::F5),                 // 3F
            0x40 => Ok(KeyCode::F6),                 // 40
            0x41 => Ok(KeyCode::F7),                 // 41
            0x42 => Ok(KeyCode::F8),                 // 42
            0x43 => Ok(KeyCode::F9),                 // 43
            0x44 => Ok(KeyCode::F10),                // 44
            0x45 => Ok(KeyCode::NumpadLock),         // 45
            0x46 => Ok(KeyCode::ScrollLock),         // 46
            0x47 => Ok(KeyCode::Numpad7),            // 47
            0x48 => Ok(KeyCode::Numpad8),            // 48
            0x49 => Ok(KeyCode::Numpad9),            // 49
            0x4A => Ok(KeyCode::NumpadMinus),        // 4A
            0x4B => Ok(KeyCode::Numpad4),            // 4B
            0x4C => Ok(KeyCode::Numpad5),            // 4C
            0x4D => Ok(KeyCode::Numpad6),            // 4D
            0x4E => Ok(KeyCode::NumpadPlus),         // 4E
            0x4F => Ok(KeyCode::Numpad1),            // 4F
            0x50 => Ok(KeyCode::Numpad2),            // 50
            0x51 => Ok(KeyCode::Numpad3),            // 51
            0x52 => Ok(KeyCode::Numpad0),            // 52
            0x53 => Ok(KeyCode::NumpadPeriod),       // 53 
            //0x54
            //0x55
            //0x56
            0x57 => Ok(KeyCode::F11),                // 57
            0x58 => Ok(KeyCode::F12),                // 58
            0x81..=0xD8 => Ok(Self::map_scancode(code-80)?),
            _ => Err(Error::UnknownKeyCode),
        }
    }

    /// Implements the extended byte codes for set 1 (prefixed with E0)
    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x10 => Ok(KeyCode::PrevTrack),    // E010
            //0x11
            //0x12
            //0x13
            //0x14
            //0x15
            //0x16
            //0x17
            //0x18
            0x19 => Ok(KeyCode::NextTrack),    // E019
            //0x1A
            //0x1B
            0x1C => Ok(KeyCode::NumpadEnter),  // E01C
            0x1D => Ok(KeyCode::ControlRight), // E01D 
            //0x1E
            //0x1F
            0x20 => Ok(KeyCode::Mute),         // E020
            0x21 => Ok(KeyCode::Calculator),   // E021
            0x22 => Ok(KeyCode::Play),         // E022
            //0x23
            0x24 => Ok(KeyCode::Stop),         // E024
            //0x25
            //0x26
            //0x27
            //0x28
            //0x29
            //0x2A
            //0x2B
            //0x2C
            //0x2D
            0x2E => Ok(KeyCode::VolumeDown),   // E02E
            //0x2F
            0x30 => Ok(KeyCode::VolumeUp),     // E030
            //0x31
            0x32 => Ok(KeyCode::WWWHome),      // E032
            //0x33
            //0x34
            0x35 => Ok(KeyCode::NumpadSlash),  // E035
            //0x36
            //0x37
            0x38 => Ok(KeyCode::AltRight),     // E038
            //0x39
            //0x3A
            //0x3B
            //0x3C
            //0x3D
            //0x3E
            //0x3F
            //0x40
            //0x41
            //0x42
            //0x43
            //0x44
            //0x45
            //0x46
            0x47 => Ok(KeyCode::Home),       // E047
            0x48 => Ok(KeyCode::ArrowUp),    // E048
            0x49 => Ok(KeyCode::PageUp),     // E049
            //0x4A
            0x4B => Ok(KeyCode::ArrowLeft),  // E04B
            //0x4C         
            0x4D => Ok(KeyCode::ArrowRight), // E04D
            //0x4E
            0x4F => Ok(KeyCode::End),        // E04F
            0x50 => Ok(KeyCode::ArrowDown),  // E050
            0x51 => Ok(KeyCode::PageDown),   // E051
            0x52 => Ok(KeyCode::Insert),     // E052
            0x53 => Ok(KeyCode::Delete),     // E053
            0x90..=0xED => Ok(Self::map_extended_scancode(code-80)?),
            _ => Err(Error::UnknownKeyCode),
        }
    }
}


/// Contains the implementation of Scancode Set 2. 
/// See the OS dev wiki: https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_2
pub struct ScancodeSet2;

impl ScancodeSet for ScancodeSet2 {
    /// Implements state logic for scancode set 2
    fn advance_state(state: DecodeState, code: u8) -> Result<ConsumeState, Error> {
        match state {
            DecodeState::Start => {
                match code {
                    EXTENDED_KEY_CODE => {
                        Ok(ConsumeState::Consume(DecodeState::Extended))
                    }, 
                    KEY_RELEASE_CODE => {
                        Ok(ConsumeState::Consume(DecodeState::Release))
                    },
                    _ => Ok(ConsumeState::Proceed(DecodeState::Start))
                }
            }, 
            DecodeState::Extended => {
                match code {
                    KEY_RELEASE_CODE => {
                        Ok(ConsumeState::Consume(DecodeState::ExtendedRelease))
                    }, 
                    _ => Ok(ConsumeState::Proceed(DecodeState::Extended))
                }
            },
            DecodeState::Release => Ok(ConsumeState::Proceed(DecodeState::Release)), 
            DecodeState::ExtendedRelease => Ok(ConsumeState::Proceed(DecodeState::ExtendedRelease))
        }
    }
    /// Implements the single byte codes for Set 2.
    fn map_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x01 => Ok(KeyCode::F9),                 // 01
            0x03 => Ok(KeyCode::F5),                 // 03
            0x04 => Ok(KeyCode::F3),                 // 04
            0x05 => Ok(KeyCode::F1),                 // 05
            0x06 => Ok(KeyCode::F2),                 // 06
            0x07 => Ok(KeyCode::F12),                // 07
            0x09 => Ok(KeyCode::F10),                // 09
            0x0A => Ok(KeyCode::F8),                 // 0A
            0x0B => Ok(KeyCode::F6),                 // 0B
            0x0C => Ok(KeyCode::F4),                 // 0C
            0x0D => Ok(KeyCode::Tab),                // 0D
            0x0E => Ok(KeyCode::BackTick),           // 0E
            0x11 => Ok(KeyCode::AltLeft),            // 11
            0x12 => Ok(KeyCode::ShiftLeft),          // 12
            0x14 => Ok(KeyCode::ControlLeft),        // 14
            0x15 => Ok(KeyCode::Q),                  // 15
            0x16 => Ok(KeyCode::Key1),               // 16
            0x1A => Ok(KeyCode::Z),                  // 1A
            0x1B => Ok(KeyCode::S),                  // 1B
            0x1C => Ok(KeyCode::A),                  // 1C
            0x1D => Ok(KeyCode::W),                  // 1D
            0x1e => Ok(KeyCode::Key2),               // 1e
            0x21 => Ok(KeyCode::C),                  // 21
            0x22 => Ok(KeyCode::X),                  // 22
            0x23 => Ok(KeyCode::D),                  // 23
            0x24 => Ok(KeyCode::E),                  // 24
            0x25 => Ok(KeyCode::Key4),               // 25
            0x26 => Ok(KeyCode::Key3),               // 26
            0x29 => Ok(KeyCode::Spacebar),           // 29
            0x2A => Ok(KeyCode::V),                  // 2A
            0x2B => Ok(KeyCode::F),                  // 2B
            0x2C => Ok(KeyCode::T),                  // 2C
            0x2D => Ok(KeyCode::R),                  // 2D
            0x2E => Ok(KeyCode::Key5),               // 2E
            0x31 => Ok(KeyCode::N),                  // 31
            0x32 => Ok(KeyCode::B),                  // 32
            0x33 => Ok(KeyCode::H),                  // 33
            0x34 => Ok(KeyCode::G),                  // 34
            0x35 => Ok(KeyCode::Y),                  // 35
            0x36 => Ok(KeyCode::Key6),               // 36
            0x3A => Ok(KeyCode::M),                  // 3A
            0x3B => Ok(KeyCode::J),                  // 3B
            0x3C => Ok(KeyCode::U),                  // 3C
            0x3D => Ok(KeyCode::Key7),               // 3D
            0x3E => Ok(KeyCode::Key8),               // 3E
            0x41 => Ok(KeyCode::Comma),              // 41
            0x42 => Ok(KeyCode::K),                  // 42
            0x43 => Ok(KeyCode::I),                  // 43
            0x44 => Ok(KeyCode::O),                  // 44
            0x45 => Ok(KeyCode::Key0),               // 45
            0x46 => Ok(KeyCode::Key9),               // 46
            0x49 => Ok(KeyCode::Fullstop),           // 49
            0x4A => Ok(KeyCode::Slash),              // 4A
            0x4B => Ok(KeyCode::L),                  // 4B
            0x4C => Ok(KeyCode::SemiColon),          // 4C
            0x4D => Ok(KeyCode::P),                  // 4D
            0x4E => Ok(KeyCode::Minus),              // 4E
            0x52 => Ok(KeyCode::Quote),              // 52
            0x54 => Ok(KeyCode::BracketSquareLeft),  // 54
            0x55 => Ok(KeyCode::Equals),             // 55
            0x58 => Ok(KeyCode::CapsLock),           // 58
            0x59 => Ok(KeyCode::ShiftRight),         // 59
            0x5A => Ok(KeyCode::Enter),              // 5A
            0x5B => Ok(KeyCode::BracketSquareRight), // 5B
            0x5D => Ok(KeyCode::BackSlash),          // 5D
            0x66 => Ok(KeyCode::Backspace),          // 66
            0x69 => Ok(KeyCode::Numpad1),            // 69
            0x6B => Ok(KeyCode::Numpad4),            // 6B
            0x6C => Ok(KeyCode::Numpad7),            // 6C
            0x70 => Ok(KeyCode::Numpad0),            // 70
            0x71 => Ok(KeyCode::NumpadPeriod),       // 71
            0x72 => Ok(KeyCode::Numpad2),            // 72
            0x73 => Ok(KeyCode::Numpad5),            // 73
            0x74 => Ok(KeyCode::Numpad6),            // 74
            0x75 => Ok(KeyCode::Numpad8),            // 75
            0x76 => Ok(KeyCode::Escape),             // 76
            0x77 => Ok(KeyCode::NumpadLock),         // 77
            0x78 => Ok(KeyCode::F11),                // 78
            0x79 => Ok(KeyCode::NumpadPlus),         // 79
            0x7A => Ok(KeyCode::Numpad3),            // 7A
            0x7B => Ok(KeyCode::NumpadMinus),        // 7B
            0x7C => Ok(KeyCode::NumpadStar),         // 7C
            0x7D => Ok(KeyCode::Numpad9),            // 7D
            0x7E => Ok(KeyCode::ScrollLock),         // 7E
            0x83 => Ok(KeyCode::F7),                 // 83
            _ => Err(Error::UnknownKeyCode),
        }
    }

    /// Implements the extended byte codes for set 1 (prefixed with E0)
    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error> {
        match code {
            0x11 => Ok(KeyCode::AltRight),     // E011
            0x14 => Ok(KeyCode::ControlRight), // E014
            0x1F => Ok(KeyCode::WindowsLeft),  // E01F
            0x27 => Ok(KeyCode::WindowsRight), // E027
            0x2F => Ok(KeyCode::Menus),        // E02F
            0x4A => Ok(KeyCode::NumpadSlash),  // E04A
            0x5A => Ok(KeyCode::NumpadEnter),  // E05A
            0x69 => Ok(KeyCode::End),          // E069
            0x6B => Ok(KeyCode::ArrowLeft),    // E06B
            0x6C => Ok(KeyCode::Home),         // E06C
            0x70 => Ok(KeyCode::Insert),       // E070
            0x71 => Ok(KeyCode::Delete),       // E071
            0x72 => Ok(KeyCode::ArrowDown),    // E072
            0x74 => Ok(KeyCode::ArrowRight),   // E074
            0x75 => Ok(KeyCode::ArrowUp),      // E075
            0x7A => Ok(KeyCode::PageDown),     // E07A
            0x7D => Ok(KeyCode::PageUp),       // E07D
            _ => Err(Error::UnknownKeyCode),
        }
    }
}