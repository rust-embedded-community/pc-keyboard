use super::{Error, KeyCode, ScancodeSet};

pub struct ScancodeSet1;

impl ScancodeSet for ScancodeSet1 {
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
                0x1A => Ok(KeyCode::LeftSquareBracket),  // 1A  #inconsistent naming order
                0x1B => Ok(KeyCode::RightSquareBracket), // 1B
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
                0x2B => Ok(KeyCode::Backslash),          // 2B #inconsistent capitals
                0x2C => Ok(KeyCode::Z),                  // 2C
                0x2D => Ok(KeyCode::X),                  // 2D
                0x2E => Ok(KeyCode::C),                  // 2E
                0x2F => Ok(KeyCode::V),                  // 2F
                0x30 => Ok(KeyCode::B),                  // 30
                0x31 => Ok(KeyCode::N),                  // 31
                0x32 => Ok(KeyCode::M),                  // 32
                0x33 => Ok(KeyCode::Comma),              // 33
                0x34 => Ok(KeyCode::Fullstop),           // 34
                0x35 => Ok(KeyCode::Slash),              // 35 #inconsistent naming
                0x36 => Ok(KeyCode::ShiftRight),         // 36
                0x37 => Ok(KeyCode::NumpadStar),         // 37
                0x38 => Ok(KeyCode::AltLeft),            // 38
                0x39 => Ok(KeyCode::Spacebar),           // 39
                0x3A => Ok(KeyCode::CapsLock),           // 3A
                0x3B => Ok(KeyCode::F1),                  // 3B
                0x3C => Ok(KeyCode::F2),                  // 3C
                0x3D => Ok(KeyCode::F3),               // 3D
                0x3E => Ok(KeyCode::F4),               // 3E
                0x3F => Ok(KeyCode::F5),             // 3F
                0x40 => Ok(KeyCode::F6),              // 40
                0x41 => Ok(KeyCode::F7),              // 41
                0x42 => Ok(KeyCode::F8),                  // 42
                0x43 => Ok(KeyCode::F9),                  // 43
                0x44 => Ok(KeyCode::F10),                  // 44
                0x45 => Ok(KeyCode::NumpadLock),               // 45
                0x46 => Ok(KeyCode::ScrollLock),               // 46
                0x47 => Ok(KeyCode::Numpad7),           // 47
                0x48 => Ok(KeyCode::Numpad8), //48
                0x49 => Ok(KeyCode::Numpad9),           // 49
                0x4A => Ok(KeyCode::NumpadMinus),              // 4A
                0x4B => Ok(KeyCode::Numpad4),                  // 4B
                0x4C => Ok(KeyCode::Numpad5),          // 4C
                0x4D => Ok(KeyCode::Numpad6),                  // 4D
                0x4E => Ok(KeyCode::NumpadPlus),              // 4E
                0x4F => Ok(KeyCode::Numpad1), //4F
                0x50 => Ok(KeyCode::Numpad2), //50
                0x51 => Ok(KeyCode::Numpad3), //51
                0x52 => Ok(KeyCode::Numpad0),              // 52
                0x53 => Ok(KeyCode::NumpadPeriod), // 53 #inconsistent naming
                //0x54
                //0x55
                //0x56
                0x57 => Ok(KeyCode::F11),  //57
                0x58 => Ok(KeyCode::F12),           // 58
                _ => Err(Error::UnknownKeyCode),
            }
        }

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
                0x1D => Ok(KeyCode::RightControl), // E01D #inconsistent naming
                //0x1E
                //0x1F
                0x20 => Ok(KeyCode::Mute),         // E020
                0x21 => Ok(KeyCode::Calculator), 
                0x22 => Ok(KeyCode::Play),
                //0x23
                0x24 => Ok(KeyCode::Stop), 
                //0x25
                //0x26
                //0x27
                //0x28
                //0x29
                //0x2A
                //0x2B
                //0x2C
                //0x2D
                0x2E => Ok(KeyCode::VolumeDown), 
                //0x2F
                0x30 => Ok(KeyCode::VolumeUp), 
                //0x31
                0x32 => Ok(KeyCode::WWWHome), 
                //0x33
                //0x34
                0x35 => Ok(KeyCode::NumpadSlash), 
                //0x36
                //0x37
                0x38 => Ok(KeyCode::AltRight),
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
                0x47 => Ok(KeyCode::Home), 
                0x48 => Ok(KeyCode::UpArrow),
                0x49 => Ok(KeyCode::PageUp),
                //0x4A
                0x4B => Ok(KeyCode::LeftArrow),
                //0x4C
                0x4D => Ok(KeyCode::RightArrow),
                //0x4E
                0x4F => Ok(KeyCode::End),
                0x50 => Ok(KeyCode::DownArrow),
                0x51 => Ok(KeyCode::PageDown),
                0x52 => Ok(KeyCode::Insert), 
                0x53 => Ok(KeyCode::Delete), 
                _ => Err(Error::UnknownKeyCode),
            }
        }
}