//! Driver for a PS/2 PC keyboard.
//!
//! Supports PS/2 Scan Code Set 1 and 2, on a variety of keyboard layouts. See
//! [the OSDev Wiki](https://wiki.osdev.org/PS/2_Keyboard).
//!
//! ## Supports:
//!
//! -   Scancode Set 1 (from the i8042 PC keyboard controller)
//! -   Scancode Set 2 (direct from the AT or PS/2 interface keyboard)
//! -   Several keyboard layouts:
//!
//! | Name                                    | No. Keys | Description                                                              | Link                                                                                |
//! | --------------------------------------- | -------- | ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
//! | [`Us104Key`](layouts::Us104Key)         | 101/104  | North American standard English                                          | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#United_States)                     |
//! | [`Uk105Key`](layouts::Uk105Key)         | 102/105  | United Kingdom standard English                                          | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#United_Kingdom)                    |
//! | [`Azerty`](layouts::Azerty)             | 102/105  | Typically used in French locales                                         | [Wikipedia](https://en.wikipedia.org/wiki/AZERTY)                                   |
//! | [`De105Key`](layouts::De105Key)         | 102/105  | German layout                                                            | [Wikipedia](https://en.wikipedia.org/wiki/QWERTZ)                                   |
//! | [`FiSe105Key`](layouts::FiSe105Key)     | 102/105  | Finnish/Swedish layout                                                   | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#Finnish%E2%80%93Swedish)           |
//! | [`No105Key`](layouts::No105Key)         | 102/105  | Norwegian layout                                                         | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#Norwegian)                         |
//! | [`Jis109Key`](layouts::Jis109Key)       | 106/109  | JIS 109-key layout (Latin chars only)                                    | [Wikipedia](https://en.wikipedia.org/wiki/Japanese_input_method#Japanese_keyboards) |
//! | [`Colemak`](layouts::Colemak)           | 101/104  | A keyboard layout designed to make typing more efficient and comfortable | [Wikipedia](https://en.wikipedia.org/wiki/Colemak)                                  |
//! | [`Dvorak104Key`](layouts::Dvorak104Key) | 101/104  | The more 'ergonomic' alternative to QWERTY                               | [Wikipedia](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout)                   |
//! | [`DVP104Key`](layouts::DVP104Key)       | 101/104  | Dvorak for Programmers                                                   | [Wikipedia](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout#Programmer_Dvorak) |
//!
//! 101/104 keys is ANSI layout (wide Enter key) and 102/105 keys is ISO layout
//! (tall Enter key). The difference between 101 and 104 (and between 102 and
//! 105) comes from the two Windows keys and the Menu key that were added when
//! Windows 95 came out. JIS keyboards have extra keys, added by making the
//! space-bar and backspace keys shorter.
//!
//! ## Usage
//!
//! There are three basic steps to handling keyboard input. Your application
//! may bypass some of these.
//!
//! * `Ps2Decoder` - converts 11-bit PS/2 words into bytes, removing the start/stop
//!   bits and checking the parity bits. Only needed if you talk to the PS/2
//!   keyboard over GPIO pins and not required if you talk to the i8042 PC keyboard
//!   controller.
//! * `ScancodeSet` - converts from Scancode Set 1 (i8042 PC keyboard controller) or
//!   Scancode Set 2 (raw PS/2 keyboard output) into a symbolic `KeyCode` and an
//!   up/down `KeyState`.
//! * `EventDecoder` - converts symbolic `KeyCode` and `KeyState` into a
//!   Unicode characters (where possible) according to the currently selected
//!   `KeyboardLayout`.
//!
//! There is also `Keyboard` which combines the above three functions into a
//! single object.
//!
//! See the [`examples`](./examples) folder for more details.
//!
//! ## Keycodes
//!
//! This crate uses symbolic keycodes to abstract over Scancode Set 1 and
//! Scancode Set 2. They represented by the `KeyCode` enum. The scancodes can
//! come from one of three supported physical keyboard layouts: 102/105 key
//! ISO, 101/104 key ANSI and 106/109-key JIS. Note that the symbolic
//! keycodes for letter keys are named after how the keys are used on a US or
//! UK English Keyboard. If you use a French AZERTY layout, the `KeyCode::Q`
//! key will produce the Unicode character `'A'`.
//!
//! ### 102/105 key [ISO](PhysicalKeyboard::Iso)
//!
//! This is the mapping of `KeyCode` to a 102/105-key ISO keyboard:
//!
//! ```text
//! ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
//! │Esc │  │ F1 │ F2 │ F3 │ F4 │  │ F5 │ F6 │ F7 │ F8 │  │ F9 │F10 │F11 │F12 │   │PrSc│Scrl│PBrk│
//! └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
//!
//! ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
//! │Oem8│Key1│Key2│Key3│Key4│Key5│Key6│Key7│Key8│Key9│Key0│Oem─│Oem+│Backspace│  │Inse│Home│PgUp│  │NumL│Num/│Num*│Num─│
//! ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
//! │ Tab │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │Oem4│Oem6│ Enter  │  │Dele│End │PgDo│  │Num7│Num8│Num9│    │
//! ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┐       │  └────┴────┴────┘  ├────┼────┼────┤Num+│
//! │CapsLo│ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │Oem1│Oem3│Oem7│       │                    │Num4│Num5│Num6│    │
//! ├────┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────┴───────┤       ┌────┐       ├────┼────┼────┼────┤
//! │LShf│Oem5│ Z  │ X  │ C  │ V  │ B  │ N  │ M  │OemC│OemP│Oem2│   RShift     │       │ Up │       │Num1│Num2│Num3│    │
//! ├────┴┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤Num │
//! │LCtrl│LWin │ Alt │       Space                  │AltGr│RWin │ Menu │RCtrl │  │Left│Down│Righ│  │Num0     │NumP│Ente│
//! └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
//! ```
//!
//! The 102-key is missing `LWin`, `RWin`, and `Menu`.
//!
//! (Reference: <https://kbdlayout.info/KBDUK/scancodes+virtualkeys?arrangement=ISO105>)
//!
//! ### 101/104 key [ANSI](PhysicalKeyboard::Ansi)
//!
//! This is the mapping of `KeyCode` to a 101/104-key ANSI keyboard:
//!
//! ```text
//! ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
//! │Esc │  │ F1 │ F2 │ F3 │ F4 │  │ F5 │ F6 │ F7 │ F8 │  │ F9 │F10 │F11 │F12 │   │PrSc│Scrl│PBrk│
//! └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
//!
//! ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
//! │Oem8│Key1│Key2│Key3│Key4│Key5│Key6│Key7│Key8│Key9│Key0│Oem─│Oem+│Backspace│  │Inse│Home│PgUp│  │NumL│Num/│Num*│Num─│
//! ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
//! │ Tab │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │Oem4│Oem6│  Oem7  │  │Dele│End │PgDo│  │Num7│Num8│Num9│    │
//! ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤Num+│
//! │CapsLo│ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │Oem1│Oem3│   Enter    │                    │Num4│Num5│Num6│    │
//! ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
//! │ LShift  │ Z  │ X  │ C  │ V  │ B  │ N  │ M  │OemC│OemP│Oem2│   RShift     │       │ Up │       │Num1│Num2│Num3│    │
//! ├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┼────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤Num │
//! │LCtrl│LWin │ Alt │       Space                  │AltGr│RWin │ Menu │RCtrl │  │Left│Down│Righ│  │Num0     │NumP│Ente│
//! └─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
//! ```
//!
//! Note that the `Oem5` key is missing on the 104-key ANSI keyboard.
//!
//! The 101-key is also missing `LWin`, `RWin`, and `Menu`.
//!
//! (Reference: <https://kbdlayout.info/KBDUK/scancodes+virtualkeys?arrangement=ANSI104>)
//!
//! ### 106/109 key [JIS](PhysicalKeyboard::Jis)
//!
//! This is the mapping of `KeyCode` to a 106/109-key JIS keyboard:
//!
//! ```text
//! ┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
//! │Esc │  │ F1 │ F2 │ F3 │ F4 │  │ F5 │ F6 │ F7 │ F8 │  │ F9 │F10 │F11 │F12 │   │PrSc│Scrl│PBrk│
//! └────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘
//!
//! ┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
//! │Oem8│Key1│Key2│Key3│Key4│Key5│Key6│Key7│Key8│Key9│Key0│Oem─│Oem+│Om13│BkSp│  │Inse│Home│PgUp│  │NumL│Num/│Num*│Num─│
//! ├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
//! │ Tab │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │Oem4│Oem6│ Enter  │  │Dele│End │PgDo│  │Num7│Num8│Num9│    │
//! ├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┐       │  └────┴────┴────┘  ├────┼────┼────┤Num+│
//! │CapsLo│ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │Oem1│Oem3│Oem7│       │                    │Num4│Num5│Num6│    │
//! ├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────┴───────┤       ┌────┐       ├────┼────┼────┼────┤
//! │LShift   │ Z  │ X  │ C  │ V  │ B  │ N  │ M  │OemC│OemP│Oem2│Oem12 │RShift │       │ Up │       │Num1│Num2│Num3│    │
//! ├─────┬───┴─┬──┴──┬─┴───┬┴────┴────┴────┴────┴┬───┴─┬──┴──┬─┴──┬───┴┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤Num │
//! │LCtrl│LWin │LAlt │Oem9 │ Space Bar           │Oem10│Oem11│RWin│Menu│RCtrl │  │Left│Down│Righ│  │Num0     │NumP│Ente│
//! └─────┴─────┴─────┴─────┴─────────────────────┴─────┴─────┴────┴────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
//! ```
//!
//! Note that the `Oem5` is missing on the 109-key JIS layout, but `Oem9`
//! (Muhenkan), `Oem10` (Henkan/Zenkouho), `Oem11`
//! (Hiragana/Katakana), `Oem12` (Backslash) and `Oem13` (¥) are added.
//!
//! The 106-key is missing `LWin`, `RWin`, and `Menu`.
//!
//! (Reference: <https://kbdlayout.info/KBDUK/scancodes+virtualkeys?arrangement=OADG109A>)
//!
//! ### Conversion Table
//!
//! Scancode Set 1 and Scancode Set 2 can be losslessly converted. Indeed, this is
//! what the i8042 keyboard controller in your PC does - it takes Scancode Set 2
//! from the keyboard and provides Scancode Set 1 to the Operating System. This
//! allowed them to change the keyboard design without breaking compatibility with
//! any MS-DOS applications that read raw scancodes from the keyboard.
//!
//! This table shows the correspondence between our symbolic KeyCode, Scancode Set 1
//! and Scancode Set 2. We may extend this in the future to also handle USB HID
//! Scancodes. Any codes prefixed `0xE0` or `0xE1` are *extended* multi-byte
//! scancodes. Typically these are keys that were not on the IBM PC and PC/XT
//! keyboards so they they were added in such a way that if you ignored the 0xE0,
//! you got a reasonable result anyway. For example `ArrowLeft` is `0xE04B` in
//! Scancode Set 1 because `Numpad4` is `0x4B` and that was the left-arrow key on an
//! IBM PC or PC/XT.
//!
//! | Symbolic Key   | Scancode Set 1 | Scancode Set 2 |
//! | -------------- | -------------- | -------------- |
//! | Escape         | 0x01           | 0x76           |
//! | F1             | 0x3B           | 0x05           |
//! | F2             | 0x3C           | 0x06           |
//! | F3             | 0x3D           | 0x04           |
//! | F4             | 0x3E           | 0x0C           |
//! | F5             | 0x3F           | 0x03           |
//! | F6             | 0x40           | 0x0B           |
//! | F7             | 0x41           | 0x83           |
//! | F8             | 0x42           | 0x0A           |
//! | F9             | 0x43           | 0x01           |
//! | F10            | 0x44           | 0x09           |
//! | F11            | 0x57           | 0x78           |
//! | F12            | 0x58           | 0x07           |
//! | PrintScreen    | 0xE037         | 0xE07C         |
//! | SysRq          | 0x54           | 0x7F           |
//! | ScrollLock     | 0x46           | 0x7E           |
//! | PauseBreak     | --             | --             |
//! | -              | --             | --             |
//! | Oem8           | 0x29           | 0x0E           |
//! | Key1           | 0x02           | 0x16           |
//! | Key2           | 0x03           | 0x1E           |
//! | Key3           | 0x04           | 0x26           |
//! | Key4           | 0x05           | 0x25           |
//! | Key5           | 0x06           | 0x2E           |
//! | Key6           | 0x07           | 0x36           |
//! | Key7           | 0x08           | 0x3D           |
//! | Key8           | 0x09           | 0x3E           |
//! | Key9           | 0x0A           | 0x46           |
//! | Key0           | 0x0B           | 0x45           |
//! | OemMinus       | 0x0C           | 0x4E           |
//! | OemPlus        | 0x0D           | 0x55           |
//! | Backspace      | 0x0E           | 0x66           |
//! | Insert         | 0xE052         | 0xE070         |
//! | Home           | 0xE047         | 0xE06C         |
//! | PageUp         | 0xE049         | 0xE07D         |
//! | NumpadLock     | 0x45           | 0x77           |
//! | NumpadDivide   | 0xE035         | 0xE04A         |
//! | NumpadMultiply | 0x37           | 0x7C           |
//! | NumpadSubtract | 0x4A           | 0x7B           |
//! | -              | --             | --             |
//! | Tab            | 0x0F           | 0x0D           |
//! | Q              | 0x10           | 0x15           |
//! | W              | 0x11           | 0x1D           |
//! | E              | 0x12           | 0x24           |
//! | R              | 0x13           | 0x2D           |
//! | T              | 0x14           | 0x2C           |
//! | Y              | 0x15           | 0x35           |
//! | U              | 0x16           | 0x3C           |
//! | I              | 0x17           | 0x43           |
//! | O              | 0x18           | 0x44           |
//! | P              | 0x19           | 0x4D           |
//! | Oem4           | 0x1A           | 0x54           |
//! | Oem6           | 0x1B           | 0x5B           |
//! | Oem5           | 0x56           | 0x61           |
//! | Oem7           | 0x2B           | 0x5D           |
//! | Delete         | 0xE053         | 0xE071         |
//! | End            | 0xE04F         | 0xE069         |
//! | PageDown       | 0xE051         | 0xE07A         |
//! | Numpad7        | 0x47           | 0x6C           |
//! | Numpad8        | 0x48           | 0x75           |
//! | Numpad9        | 0x49           | 0x7D           |
//! | NumpadAdd      | 0x4E           | 0x79           |
//! | -              | --             | --             |
//! | CapsLock       | 0x3A           | 0x58           |
//! | A              | 0x1E           | 0x1C           |
//! | S              | 0x1F           | 0x1B           |
//! | D              | 0x20           | 0x23           |
//! | F              | 0x21           | 0x2B           |
//! | G              | 0x22           | 0x34           |
//! | H              | 0x23           | 0x33           |
//! | J              | 0x24           | 0x3B           |
//! | K              | 0x25           | 0x42           |
//! | L              | 0x26           | 0x4B           |
//! | Oem1           | 0x27           | 0x4C           |
//! | Oem3           | 0x28           | 0x52           |
//! | Return         | 0x1C           | 0x5A           |
//! | Numpad4        | 0x4B           | 0x6B           |
//! | Numpad5        | 0x4C           | 0x73           |
//! | Numpad6        | 0x4D           | 0x74           |
//! | -              | --             | --             |
//! | LShift         | 0x2A           | 0x12           |
//! | Z              | 0x2C           | 0x1A           |
//! | X              | 0x2D           | 0x22           |
//! | C              | 0x2E           | 0x21           |
//! | V              | 0x2F           | 0x2A           |
//! | B              | 0x30           | 0x32           |
//! | N              | 0x31           | 0x31           |
//! | M              | 0x32           | 0x3A           |
//! | OemComma       | 0x33           | 0x41           |
//! | OemPeriod      | 0x34           | 0x49           |
//! | Oem2           | 0x35           | 0x4A           |
//! | RShift         | 0x36           | 0x59           |
//! | ArrowUp        | 0xE048         | 0xE075         |
//! | Numpad1        | 0x4F           | 0x69           |
//! | Numpad2        | 0x50           | 0x72           |
//! | Numpad3        | 0x51           | 0x7A           |
//! | NumpadEnter    | 0xE01C         | 0xE075         |
//! | -              | --             | --             |
//! | LControl       | 0x1D           | 0x14           |
//! | LWin           | 0xE05B         | 0xE01F         |
//! | LAlt           | 0x38           | 0x11           |
//! | Spacebar       | 0x39           | 0x29           |
//! | RAltGr         | 0xE038         | 0xE011         |
//! | RWin           | 0xE05C         | 0xE027         |
//! | Apps           | 0xE05C         | 0xE02F         |
//! | RControl       | 0xE01D         | 0xE014         |
//! | ArrowLeft      | 0xE04B         | 0xE06B         |
//! | ArrowDown      | 0xE050         | 0xE072         |
//! | ArrowRight     | 0xE04D         | 0xE074         |
//! | Numpad0        | 0x52           | 0x70           |
//! | NumpadPeriod   | 0x53           | 0x71           |
//! | -              | --             | --             |
//! | Oem9           | 0x7B           | 0x67           |
//! | Oem10          | 0x79           | 0x64           |
//! | Oem11          | 0x70           | 0x13           |
//! | Oem12          | 0x73           | 0x51           |
//! | Oem13          | 0x7D           | 0x6A           |
//! | -              | --             | --             |
//! | PrevTrack      | 0xE010         | 0xE015         |
//! | NextTrack      | 0xE019         | 0xE04D         |
//! | Mute           | 0xE020         | 0xE023         |
//! | Calculator     | 0xE021         | 0xE02B         |
//! | Play           | 0xE022         | 0xE034         |
//! | Stop           | 0xE024         | 0xE03B         |
//! | VolumeDown     | 0xE02E         | 0xE021         |
//! | VolumeUp       | 0xE030         | 0xE032         |
//! | WWWHome        | 0xE032         | 0xE03A         |
//! | TooManyKeys    | --             | 0x00           |
//! | PowerOnTestOk  | --             | 0xAA           |
//! | RControl2      | 0xE11D         | 0xE114         |
//! | RAlt2          | 0xE02A         | 0xE012         |
//!
//! __Note 1:__ `PauseBreak` does not have a scancode because it's something we infer from a
//! sequence of other keypresses (`NumLock` with `RControl2` held).
//!
//! __Note 2:__ `SysReq` doesn't have a key on the diagram, because the scancode is
//! only generated when you do `Alt` + `PrintScreen`.

#![cfg_attr(not(test), no_std)]

// ****************************************************************************
//
// Modules
//
// ****************************************************************************

pub mod layouts;

mod scancodes;
pub use crate::scancodes::{ScancodeSet1, ScancodeSet2};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

/// Encapsulates decode/sampling logic, and handles state transitions and key events.
#[derive(Debug)]
pub struct Keyboard<L, S>
where
    S: ScancodeSet,
    L: KeyboardLayout,
{
    ps2_decoder: Ps2Decoder,
    scancode_set: S,
    event_decoder: EventDecoder<L>,
}

/// Handles decoding of IBM PS/2 Keyboard (and IBM PC/AT Keyboard) bit-streams.
#[derive(Debug)]
pub struct Ps2Decoder {
    register: u16,
    num_bits: u8,
}

/// Converts KeyEvents into Unicode, according to the current Keyboard Layout
#[derive(Debug)]
pub struct EventDecoder<L>
where
    L: KeyboardLayout,
{
    handle_ctrl: HandleControl,
    modifiers: Modifiers,
    layout: L,
}

/// Indicates different error conditions.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum Error {
    BadStartBit,
    BadStopBit,
    ParityError,
    UnknownKeyCode,
}

/// Keycodes that can be generated by a keyboard.
///
/// We use this enum to abstract over Scan Code Set 1 and Scan Code Set 2.
///
/// See <https://kbdlayout.info/kbduk/shiftstates+virtualkeys/base>
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyCode {
    // ========= Row 1 (the F-keys) =========
    /// Top Left of the Keyboard
    Escape,
    /// Function Key F1
    F1,
    /// Function Key F2
    F2,
    /// Function Key F3
    F3,
    /// Function Key F4
    F4,
    /// Function Key F5
    F5,
    /// Function Key F6
    F6,
    /// Function Key F7
    F7,
    /// Function Key F8
    F8,
    /// Function Key F9
    F9,
    /// Function Key F10
    F10,
    /// Function Key F11
    F11,
    /// Function Key F12
    F12,

    /// The Print Screen Key
    PrintScreen,
    /// The Sys Req key (you get this keycode with Alt + PrintScreen)
    SysRq,
    /// The Scroll Lock key
    ScrollLock,
    /// The Pause/Break key
    PauseBreak,

    // ========= Row 2 (the numbers) =========
    /// Symbol key to the left of `Key1`
    Oem8,
    /// Number Line, Digit 1
    Key1,
    /// Number Line, Digit 2
    Key2,
    /// Number Line, Digit 3
    Key3,
    /// Number Line, Digit 4
    Key4,
    /// Number Line, Digit 5
    Key5,
    /// Number Line, Digit 6
    Key6,
    /// Number Line, Digit 7
    Key7,
    /// Number Line, Digit 8
    Key8,
    /// Number Line, Digit 9
    Key9,
    /// Number Line, Digit 0
    Key0,
    /// US Minus/Underscore Key (right of 'Key0')
    OemMinus,
    /// US Equals/Plus Key (right of 'OemMinus')
    OemPlus,
    /// Backspace
    Backspace,

    /// Top Left of the Extended Block
    Insert,
    /// Top Middle of the Extended Block
    Home,
    /// Top Right of the Extended Block
    PageUp,

    /// The Num Lock key
    NumpadLock,
    /// The Numpad Divide (or Slash) key
    NumpadDivide,
    /// The Numpad Multiple (or Star) key
    NumpadMultiply,
    /// The Numpad Subtract (or Minus) key
    NumpadSubtract,

    // ========= Row 3 (QWERTY) =========
    /// The Tab Key
    Tab,
    /// Letters, Top Row #1
    Q,
    /// Letters, Top Row #2
    W,
    /// Letters, Top Row #3
    E,
    /// Letters, Top Row #4
    R,
    /// Letters, Top Row #5
    T,
    /// Letters, Top Row #6
    Y,
    /// Letters, Top Row #7
    U,
    /// Letters, Top Row #8
    I,
    /// Letters, Top Row #9
    O,
    /// Letters, Top Row #10
    P,
    /// US ANSI Left-Square-Bracket key
    Oem4,
    /// US ANSI Right-Square-Bracket key
    Oem6,
    /// US ANSI Backslash Key / UK ISO Backslash Key
    Oem5,
    /// The UK/ISO Hash/Tilde key (ISO layout only)
    Oem7,

    /// The Delete key - bottom Left of the Extended Block
    Delete,
    /// The End key - bottom Middle of the Extended Block
    End,
    /// The Page Down key - -bottom Right of the Extended Block
    PageDown,

    /// The Numpad 7/Home key
    Numpad7,
    /// The Numpad 8/Up Arrow key
    Numpad8,
    /// The Numpad 9/Page Up key
    Numpad9,
    /// The Numpad Add/Plus key
    NumpadAdd,

    // ========= Row 4 (ASDF) =========
    /// Caps Lock
    CapsLock,
    /// Letters, Middle Row #1
    A,
    /// Letters, Middle Row #2
    S,
    /// Letters, Middle Row #3
    D,
    /// Letters, Middle Row #4
    F,
    /// Letters, Middle Row #5
    G,
    /// Letters, Middle Row #6
    H,
    /// Letters, Middle Row #7
    J,
    /// Letters, Middle Row #8
    K,
    /// Letters, Middle Row #9
    L,
    /// The US ANSI Semicolon/Colon key
    Oem1,
    /// The US ANSI Single-Quote/At key
    Oem3,

    /// The Return Key
    Return,

    /// The Numpad 4/Left Arrow key
    Numpad4,
    /// The Numpad 5 Key
    Numpad5,
    /// The Numpad 6/Right Arrow key
    Numpad6,

    // ========= Row 5 (ZXCV) =========
    /// Left Shift
    LShift,
    /// Letters, Bottom Row #1
    Z,
    /// Letters, Bottom Row #2
    X,
    /// Letters, Bottom Row #3
    C,
    /// Letters, Bottom Row #4
    V,
    /// Letters, Bottom Row #5
    B,
    /// Letters, Bottom Row #6
    N,
    /// Letters, Bottom Row #7
    M,
    /// US ANSI `,<` key
    OemComma,
    /// US ANSI `.>` Key
    OemPeriod,
    /// US ANSI `/?` Key
    Oem2,
    /// Right Shift
    RShift,

    /// The up-arrow in the inverted-T
    ArrowUp,

    /// Numpad 1/End Key
    Numpad1,
    /// Numpad 2/Arrow Down Key
    Numpad2,
    /// Numpad 3/Page Down Key
    Numpad3,
    /// Numpad Enter
    NumpadEnter,

    // ========= Row 6 (modifers and space bar) =========
    /// The left-hand Control key
    LControl,
    /// The left-hand 'Windows' key
    LWin,
    /// The left-hand Alt key
    LAlt,
    /// The Space Bar
    Spacebar,
    /// The right-hand AltGr key
    RAltGr,
    /// The right-hand Win key
    RWin,
    /// The 'Apps' key (aka 'Menu' or 'Right-Click')
    Apps,
    /// The right-hand Control key
    RControl,

    /// The left-arrow in the inverted-T
    ArrowLeft,
    /// The down-arrow in the inverted-T
    ArrowDown,
    /// The right-arrow in the inverted-T
    ArrowRight,

    /// The Numpad 0/Insert Key
    Numpad0,
    /// The Numppad Period/Delete Key
    NumpadPeriod,

    // ========= JIS 109-key extra keys =========
    /// Extra JIS key (0x7B)
    Oem9,
    /// Extra JIS key (0x79)
    Oem10,
    /// Extra JIS key (0x70)
    Oem11,
    /// Extra JIS symbol key (0x73)
    Oem12,
    /// Extra JIS symbol key (0x7D)
    Oem13,

    // ========= Extra Keys =========
    /// Multi-media keys - Previous Track
    PrevTrack,
    /// Multi-media keys - Next Track
    NextTrack,
    /// Multi-media keys - Volume Mute Toggle
    Mute,
    /// Multi-media keys - Open Calculator
    Calculator,
    /// Multi-media keys - Play
    Play,
    /// Multi-media keys - Stop
    Stop,
    /// Multi-media keys - Increase Volume
    VolumeDown,
    /// Multi-media keys - Decrease Volume
    VolumeUp,
    /// Multi-media keys - Open Browser
    WWWHome,
    /// Sent when the keyboard boots
    PowerOnTestOk,
    /// Sent by the keyboard when too many keys are pressed
    TooManyKeys,
    /// Used as a 'hidden' Right Control Key (Pause = RControl2 + Num Lock)
    RControl2,
    /// Used as a 'hidden' Right Alt Key (Print Screen = RAlt2 + PrntScr)
    RAlt2,
}

/// The new state for a key, as part of a key event.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    /// Key has just been released
    Up,
    /// Key has just been pressed
    Down,
    /// Key was pressed and then released as an atomic action. Or it's like a
    /// PowerOnSelfTest event which doesn't have an 'Up' or a 'Down'.
    SingleShot,
}

/// Options for how we can handle what happens when the Ctrl key is held down
/// and a letter is pressed.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HandleControl {
    /// If either Ctrl key is held down, convert the letters A through Z into
    /// Unicode chars U+0001 through U+001A. If the Ctrl keys are not held
    /// down, letters go through normally.
    MapLettersToUnicode,
    /// Don't do anything special - send through the Ctrl key up/down events,
    /// and leave the letters as letters.
    Ignore,
}

/// A event describing something happen to a key on your keyboard.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    /// Which key this event is for
    pub code: KeyCode,
    /// The new state for the key
    pub state: KeyState,
}

/// Describes a physical keyboard
pub enum PhysicalKeyboard {
    /// 102 or 105 key ISO, as used by UK English keyboards (and others)
    Iso,
    /// 101 or 104 key ANSI, as used by US English keyboards (and others)
    Ansi,
    /// 106 or 109 key JIS, as used by Japanese keyboards (and others)
    Jis,
}

/// Describes a Keyboard Layout.
///
/// Layouts might include "en_US", or "en_GB", or "de_GR".
pub trait KeyboardLayout {
    /// Convert a `KeyCode` enum to a Unicode character, if possible.
    /// `KeyCode::A` maps to `DecodedKey::Unicode('a')` (or
    /// `DecodedKey::Unicode('A')` if shifted), while `KeyCode::LAlt` becomes
    /// `DecodedKey::RawKey(KeyCode::LAlt)` because there's no Unicode equivalent.
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey;

    /// Which physical keyboard does this layout work on?
    fn get_physical(&self) -> PhysicalKeyboard;
}

/// A mechanism to convert bytes from a Keyboard into [`KeyCode`] values.
///
/// This conversion is stateful.
pub trait ScancodeSet {
    /// Handles the state logic for the decoding of scan codes into key events.
    fn advance_state(&mut self, code: u8) -> Result<Option<KeyEvent>, Error>;
}

/// The set of modifier keys you have on a keyboard.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Modifiers {
    /// The left shift key is down
    pub lshift: bool,
    /// The right shift key is down
    pub rshift: bool,
    /// The left control key is down
    pub lctrl: bool,
    /// The right control key is down
    pub rctrl: bool,
    /// The Num Lock toggle is on
    pub numlock: bool,
    /// The caps lock toggle is on
    pub capslock: bool,
    /// The left alt key is down
    pub lalt: bool,
    /// The right alt key is down
    pub ralt: bool,
    /// Special 'hidden' control key is down (used when you press Pause)
    pub rctrl2: bool,
}

/// Contains either a Unicode character, or a raw key code.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodedKey {
    RawKey(KeyCode),
    Unicode(char),
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

/// Tracls
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DecodeState {
    Start,
    Extended,
    Release,
    ExtendedRelease,
    Extended2,
    Extended2Release,
}

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

const KEYCODE_BITS: u8 = 11;
const EXTENDED_KEY_CODE: u8 = 0xE0;
const EXTENDED2_KEY_CODE: u8 = 0xE1;
const KEY_RELEASE_CODE: u8 = 0xF0;

const QUO: char = '\'';
const SLS: char = '\\';

// ****************************************************************************
//
// Public Functions and Implementation
//
// ****************************************************************************

impl<L, S> Keyboard<L, S>
where
    L: KeyboardLayout,
    S: ScancodeSet,
{
    /// Make a new Keyboard object with the given layout.
    pub const fn new(scancode_set: S, layout: L, handle_ctrl: HandleControl) -> Keyboard<L, S> {
        Keyboard {
            ps2_decoder: Ps2Decoder::new(),
            scancode_set,
            event_decoder: EventDecoder::new(layout, handle_ctrl),
        }
    }

    /// Get the current key modifier states.
    pub const fn get_modifiers(&self) -> &Modifiers {
        &self.event_decoder.modifiers
    }

    /// Change the Ctrl key mapping.
    pub fn set_ctrl_handling(&mut self, new_value: HandleControl) {
        self.event_decoder.set_ctrl_handling(new_value);
    }

    /// Get the current Ctrl key mapping.
    pub const fn get_ctrl_handling(&self) -> HandleControl {
        self.event_decoder.get_ctrl_handling()
    }

    /// Clears the bit register.
    ///
    /// Call this when there is a timeout reading data from the keyboard.
    pub fn clear(&mut self) {
        self.ps2_decoder.clear();
    }

    /// Processes a 16-bit word from the keyboard.
    ///
    /// * The start bit (0) must be in bit 0.
    /// * The data octet must be in bits 1..8, with the LSB in bit 1 and the
    ///   MSB in bit 8.
    /// * The parity bit must be in bit 9.
    /// * The stop bit (1) must be in bit 10.
    pub fn add_word(&mut self, word: u16) -> Result<Option<KeyEvent>, Error> {
        let byte = self.ps2_decoder.add_word(word)?;
        self.add_byte(byte)
    }

    /// Processes an 8-bit byte from the keyboard.
    ///
    /// We assume the start, stop and parity bits have been processed and
    /// verified.
    pub fn add_byte(&mut self, byte: u8) -> Result<Option<KeyEvent>, Error> {
        self.scancode_set.advance_state(byte)
    }

    /// Shift a bit into the register.
    ///
    /// Call this /or/ call `add_word` - don't call both.
    /// Until the last bit is added you get Ok(None) returned.
    pub fn add_bit(&mut self, bit: bool) -> Result<Option<KeyEvent>, Error> {
        if let Some(byte) = self.ps2_decoder.add_bit(bit)? {
            self.scancode_set.advance_state(byte)
        } else {
            Ok(None)
        }
    }

    /// Processes a `KeyEvent` returned from `add_bit`, `add_byte` or `add_word`
    /// and produces a decoded key.
    ///
    /// For example, the KeyEvent for pressing the '5' key on your keyboard
    /// gives a DecodedKey of unicode character '5', unless the shift key is
    /// held in which case you get the unicode character '%'.
    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        self.event_decoder.process_keyevent(ev)
    }
}

impl Ps2Decoder {
    /// Build a new PS/2 protocol decoder.
    pub const fn new() -> Ps2Decoder {
        Ps2Decoder {
            register: 0,
            num_bits: 0,
        }
    }

    /// Clears the bit register.
    ///
    /// Call this when there is a timeout reading data from the keyboard.
    pub fn clear(&mut self) {
        self.register = 0;
        self.num_bits = 0;
    }

    /// Shift a bit into the register.
    ///
    /// Until the last bit is added you get Ok(None) returned.
    pub fn add_bit(&mut self, bit: bool) -> Result<Option<u8>, Error> {
        self.register |= (bit as u16) << self.num_bits;
        self.num_bits += 1;
        if self.num_bits == KEYCODE_BITS {
            let word = self.register;
            self.register = 0;
            self.num_bits = 0;
            let byte = Self::check_word(word)?;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    /// Process an entire 11-bit word.
    ///
    /// Must be packed into the bottom 11-bits of the 16-bit value.
    pub fn add_word(&self, word: u16) -> Result<u8, Error> {
        Self::check_word(word)
    }

    /// Check 11-bit word has 1 start bit, 1 stop bit and an odd parity bit.
    const fn check_word(word: u16) -> Result<u8, Error> {
        let start_bit = Self::get_bit(word, 0);
        let parity_bit = Self::get_bit(word, 9);
        let stop_bit = Self::get_bit(word, 10);
        let data = ((word >> 1) & 0xFF) as u8;

        if start_bit {
            return Err(Error::BadStartBit);
        }

        if !stop_bit {
            return Err(Error::BadStopBit);
        }

        // We have odd parity, so if there are an even number of 1 bits, we need
        // the parity bit set to make it odd.
        let need_parity = Self::has_even_number_bits(data);

        if need_parity != parity_bit {
            return Err(Error::ParityError);
        }

        Ok(data)
    }

    const fn get_bit(word: u16, offset: usize) -> bool {
        ((word >> offset) & 0x0001) != 0
    }

    const fn has_even_number_bits(data: u8) -> bool {
        (data.count_ones() % 2) == 0
    }
}

impl Default for Ps2Decoder {
    fn default() -> Self {
        Ps2Decoder::new()
    }
}

impl<L> EventDecoder<L>
where
    L: KeyboardLayout,
{
    /// Construct a new event decoder.
    pub const fn new(layout: L, handle_ctrl: HandleControl) -> EventDecoder<L> {
        EventDecoder {
            handle_ctrl,
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                numlock: true,
                capslock: false,
                lalt: false,
                ralt: false,
                rctrl2: false,
            },
            layout,
        }
    }

    /// Change the Ctrl key mapping.
    pub fn set_ctrl_handling(&mut self, new_value: HandleControl) {
        self.handle_ctrl = new_value;
    }

    /// Get the current Ctrl key mapping.
    pub const fn get_ctrl_handling(&self) -> HandleControl {
        self.handle_ctrl
    }

    /// Processes a `KeyEvent` returned from `add_bit`, `add_byte` or `add_word`
    /// and produces a decoded key.
    ///
    /// For example, the KeyEvent for pressing the '5' key on your keyboard
    /// gives a DecodedKey of unicode character '5', unless the shift key is
    /// held in which case you get the unicode character '%'.
    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev {
            KeyEvent {
                code: KeyCode::LShift,
                state: KeyState::Down,
            } => {
                self.modifiers.lshift = true;
                Some(DecodedKey::RawKey(KeyCode::LShift))
            }
            KeyEvent {
                code: KeyCode::RShift,
                state: KeyState::Down,
            } => {
                self.modifiers.rshift = true;
                Some(DecodedKey::RawKey(KeyCode::RShift))
            }
            KeyEvent {
                code: KeyCode::LShift,
                state: KeyState::Up,
            } => {
                self.modifiers.lshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::RShift,
                state: KeyState::Up,
            } => {
                self.modifiers.rshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::CapsLock,
                state: KeyState::Down,
            } => {
                self.modifiers.capslock = !self.modifiers.capslock;
                Some(DecodedKey::RawKey(KeyCode::CapsLock))
            }
            KeyEvent {
                code: KeyCode::NumpadLock,
                state: KeyState::Down,
            } => {
                if self.modifiers.rctrl2 {
                    // It's a Pause key because we got the 'hidden' rctrl2
                    // sequence first.
                    Some(DecodedKey::RawKey(KeyCode::PauseBreak))
                } else {
                    // It's a numlock toggle
                    self.modifiers.numlock = !self.modifiers.numlock;
                    Some(DecodedKey::RawKey(KeyCode::NumpadLock))
                }
            }
            KeyEvent {
                code: KeyCode::LControl,
                state: KeyState::Down,
            } => {
                self.modifiers.lctrl = true;
                Some(DecodedKey::RawKey(KeyCode::LControl))
            }
            KeyEvent {
                code: KeyCode::LControl,
                state: KeyState::Up,
            } => {
                self.modifiers.lctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::RControl,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl = true;
                Some(DecodedKey::RawKey(KeyCode::RControl))
            }
            KeyEvent {
                code: KeyCode::RControl,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::LAlt,
                state: KeyState::Down,
            } => {
                self.modifiers.lalt = true;
                Some(DecodedKey::RawKey(KeyCode::LAlt))
            }
            KeyEvent {
                code: KeyCode::LAlt,
                state: KeyState::Up,
            } => {
                self.modifiers.lalt = false;
                None
            }
            KeyEvent {
                code: KeyCode::RAltGr,
                state: KeyState::Down,
            } => {
                self.modifiers.ralt = true;
                Some(DecodedKey::RawKey(KeyCode::RAltGr))
            }
            KeyEvent {
                code: KeyCode::RAltGr,
                state: KeyState::Up,
            } => {
                self.modifiers.ralt = false;
                None
            }
            KeyEvent {
                code: KeyCode::RControl2,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl2 = true;
                Some(DecodedKey::RawKey(KeyCode::RControl2))
            }
            KeyEvent {
                code: KeyCode::RControl2,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl2 = false;
                None
            }
            KeyEvent {
                code: c,
                state: KeyState::Down,
            } => Some(
                self.layout
                    .map_keycode(c, &self.modifiers, self.handle_ctrl),
            ),
            _ => None,
        }
    }

    /// Change the keyboard layout.
    ///
    /// Only useful with [`layouts::AnyLayout`], otherwise you can only change a
    /// layout for exactly the same layout.
    pub fn change_layout(&mut self, new_layout: L) {
        self.layout = new_layout;
    }
}

impl KeyEvent {
    pub const fn new(code: KeyCode, state: KeyState) -> KeyEvent {
        KeyEvent { code, state }
    }
}

impl Modifiers {
    pub const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    pub const fn is_ctrl(&self) -> bool {
        self.lctrl | self.rctrl
    }

    pub const fn is_alt(&self) -> bool {
        self.lalt | self.ralt
    }

    pub const fn is_altgr(&self) -> bool {
        self.ralt | (self.lalt & self.is_ctrl())
    }

    pub const fn is_caps(&self) -> bool {
        self.is_shifted() ^ self.capslock
    }

    /// Handle letter keys with standard ASCII 'A'..'Z' keycaps.
    ///
    /// ONLY pass 'A'..='Z' - nothing else.
    ///
    /// You will get a `DecodedKey::Unicode` value with the appropriate lower
    /// or upper case letter, according to state of the the Caps Lock and
    /// Shift modifiers.
    pub(crate) fn handle_ascii_2(&self, letter: char, handle_ctrl: HandleControl) -> DecodedKey {
        debug_assert!(letter.is_ascii_uppercase());
        if handle_ctrl == HandleControl::MapLettersToUnicode && self.is_ctrl() {
            // Get a Control code, like Ctrl+C => U+0003
            const ASCII_UPPERCASE_START_OFFSET: u8 = 64;
            DecodedKey::Unicode((letter as u8 - ASCII_UPPERCASE_START_OFFSET) as char)
        } else if self.is_caps() {
            // Capital letter
            DecodedKey::Unicode(letter)
        } else {
            // Lowercase letter
            const ASCII_UPPER_TO_LOWER_OFFSET: u8 = 32;
            DecodedKey::Unicode((letter as u8 + ASCII_UPPER_TO_LOWER_OFFSET) as char)
        }
    }

    /// Handle letter keys with just two variants (lower and upper case).
    ///
    /// Designed for non-ASCII keys, this does not produce control codes.
    ///
    /// You will get a `DecodedKey::Unicode` value with the appropriate lower
    /// or upper case letter, according to state of the the Caps Lock and
    /// Shift modifiers.
    ///
    /// We make you pass both upper and lower case variants to avoid having to
    /// use the `char::to_lowercase` function.
    pub(crate) fn handle_letter2(&self, letter_lower: char, letter_upper: char) -> DecodedKey {
        if self.is_caps() {
            DecodedKey::Unicode(letter_upper)
        } else {
            DecodedKey::Unicode(letter_lower)
        }
    }

    /// Handle letter keys with standard ASCII 'A'..'Z' keycaps with two extra symbols.
    ///
    /// ONLY pass 'A'..='Z' - nothing else
    ///
    /// You will get a `DecodedKey::Unicode` value with the appropriate lower
    /// or upper case letter, according to state of the the Caps Lock and
    /// Shift modifiers. Or, if AltGr is held, you get either the alternate
    /// character. Useful if your alternate character is e.g. `€`.
    ///
    /// We make you pass both upper and lower case variants to avoid having to
    /// use the `char::to_lowercase` function.
    pub(crate) fn handle_ascii_3(
        &self,
        letter_upper: char,
        alt: char,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        debug_assert!(letter_upper.is_ascii_uppercase());
        if handle_ctrl == HandleControl::MapLettersToUnicode && self.is_ctrl() {
            // Get a Control code, like Ctrl+C => U+0003
            const ASCII_UPPERCASE_START_OFFSET: u8 = 64;
            DecodedKey::Unicode((letter_upper as u8 - ASCII_UPPERCASE_START_OFFSET) as char)
        } else if self.ralt {
            // Alternate character
            DecodedKey::Unicode(alt)
        } else if self.is_caps() {
            // Capital letter
            DecodedKey::Unicode(letter_upper)
        } else {
            // Lowercase letter
            const ASCII_UPPER_TO_LOWER_OFFSET: u8 = 32;
            DecodedKey::Unicode((letter_upper as u8 + ASCII_UPPER_TO_LOWER_OFFSET) as char)
        }
    }

    /// Handle letter keys with standard ASCII 'A'..'Z' keycaps with two extra symbols.
    ///
    /// ONLY pass 'A'..='Z' - nothing else
    ///
    /// You will get a `DecodedKey::Unicode` value with the appropriate lower
    /// or upper case letter, according to state of the the Caps Lock and
    /// Shift modifiers. Or, if AltGr is held, you get either the upper or
    /// lower case alternate character. Useful if your alternate character is
    /// e.g. `é` (or `É` if Shift or Caps Lock is enabled).
    ///
    /// We make you pass both upper and lower case variants to avoid having to
    /// use the `char::to_lowercase` function.
    pub(crate) fn handle_ascii_4(
        &self,
        letter_upper: char,
        alt_letter_lower: char,
        alt_letter_upper: char,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        debug_assert!(letter_upper.is_ascii_uppercase());
        if handle_ctrl == HandleControl::MapLettersToUnicode && self.is_ctrl() {
            // Get a Control code, like Ctrl+C => U+0003
            const ASCII_UPPERCASE_START_OFFSET: u8 = 64;
            DecodedKey::Unicode((letter_upper as u8 - ASCII_UPPERCASE_START_OFFSET) as char)
        } else if self.ralt && self.is_caps() {
            // Capital letter
            DecodedKey::Unicode(alt_letter_upper)
        } else if self.ralt {
            // Lowercase letter
            DecodedKey::Unicode(alt_letter_lower)
        } else if self.is_caps() {
            // Capital letter
            DecodedKey::Unicode(letter_upper)
        } else {
            // Lowercase letter
            const ASCII_UPPER_TO_LOWER_OFFSET: u8 = 32;
            DecodedKey::Unicode((letter_upper as u8 + ASCII_UPPER_TO_LOWER_OFFSET) as char)
        }
    }

    /// Handle numpad keys which are either a character or a raw key
    pub(crate) fn handle_num_pad(&self, letter: char, key: KeyCode) -> DecodedKey {
        if self.numlock {
            DecodedKey::Unicode(letter)
        } else {
            DecodedKey::RawKey(key)
        }
    }

    /// Handle numpad keys which produce a pair of characters
    ///
    /// This is usually just for Numpad Delete.
    pub(crate) fn handle_num_del(&self, letter: char, other: char) -> DecodedKey {
        if self.numlock {
            DecodedKey::Unicode(letter)
        } else {
            DecodedKey::Unicode(other)
        }
    }

    /// Handle standard two-glyph shifted keys
    ///
    /// Caps Lock is ignored here - only shift matters.
    pub(crate) fn handle_symbol2(&self, plain: char, shifted: char) -> DecodedKey {
        if self.is_shifted() {
            DecodedKey::Unicode(shifted)
        } else {
            DecodedKey::Unicode(plain)
        }
    }

    /// Handle standard three-glyph shifted keys
    ///
    /// Caps Lock is ignored here - only shift matters. AltGr gets you the
    /// alternate letter, regardless of Shift status.
    pub(crate) fn handle_symbol3(&self, plain: char, shifted: char, alt: char) -> DecodedKey {
        if self.is_altgr() {
            DecodedKey::Unicode(alt)
        } else if self.is_shifted() {
            DecodedKey::Unicode(shifted)
        } else {
            DecodedKey::Unicode(plain)
        }
    }
}

// ****************************************************************************
//
// Tests
//
// ****************************************************************************

#[cfg(test)]
mod test {
    use super::*;

    fn add_bytes<L, S>(keyboard: &mut Keyboard<L, S>, test_sequence: &[(u8, Option<KeyEvent>)])
    where
        L: KeyboardLayout,
        S: ScancodeSet,
    {
        for (byte, expected_key) in test_sequence.iter().cloned() {
            let result = keyboard.add_byte(byte);
            assert_eq!(
                result,
                Ok(expected_key.clone()),
                "0x{:02x} should have given {:?} not {:?}",
                byte,
                expected_key,
                result
            );
        }
    }

    fn process_keyevents<L, S>(
        keyboard: &mut Keyboard<L, S>,
        test_sequence: &[(KeyEvent, Option<DecodedKey>)],
    ) where
        L: KeyboardLayout,
        S: ScancodeSet,
    {
        for (idx, (event, expected_decode)) in test_sequence.iter().cloned().enumerate() {
            let result = keyboard.process_keyevent(event.clone());
            assert_eq!(
                result,
                expected_decode.clone(),
                "Entry {} {:?} should have given {:?} not {:?}",
                idx,
                event,
                expected_decode,
                result
            );
        }
    }

    #[test]
    fn test_f9() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        // parity
        assert_eq!(k.add_bit(false), Ok(None));
        // stop
        assert_eq!(
            k.add_bit(true),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_f9_word() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        assert_eq!(
            k.add_word(0x0402),
            Ok(Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))
        );
    }

    #[test]
    fn test_f9_byte() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );

        let test_sequence = [(0x01, Some(KeyEvent::new(KeyCode::F9, KeyState::Down)))];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_keyup_keydown() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            (0x01, Some(KeyEvent::new(KeyCode::F9, KeyState::Down))),
            (0x01, Some(KeyEvent::new(KeyCode::F9, KeyState::Down))),
            (0xF0, None),
            (0x01, Some(KeyEvent::new(KeyCode::F9, KeyState::Up))),
        ];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_f5() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        // parity
        assert_eq!(k.add_bit(true), Ok(None));
        // stop
        assert_eq!(
            k.add_bit(true),
            Ok(Some(KeyEvent::new(KeyCode::F5, KeyState::Down)))
        );
    }

    #[test]
    fn test_f5_up() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        // Send F0

        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        // parity
        assert_eq!(k.add_bit(true), Ok(None));
        // stop
        assert_eq!(k.add_bit(true), Ok(None));

        // Send 03

        // start
        assert_eq!(k.add_bit(false), Ok(None));
        // 8 data bits (LSB first)
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(true), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        assert_eq!(k.add_bit(false), Ok(None));
        // parity
        assert_eq!(k.add_bit(true), Ok(None));
        // stop
        assert_eq!(
            k.add_bit(true),
            Ok(Some(KeyEvent::new(KeyCode::F5, KeyState::Up)))
        );
    }

    #[test]
    fn test_shift() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            // A with left shift held
            (
                KeyEvent::new(KeyCode::LShift, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::LShift)),
            ),
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('A')),
            ),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            (KeyEvent::new(KeyCode::LShift, KeyState::Up), None),
            // A with no shift
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('a')),
            ),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            // A with right shift held
            (
                KeyEvent::new(KeyCode::RShift, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::RShift)),
            ),
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('A')),
            ),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            (KeyEvent::new(KeyCode::RShift, KeyState::Up), None),
            // Caps lock ON
            (
                KeyEvent::new(KeyCode::CapsLock, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::CapsLock)),
            ),
            (KeyEvent::new(KeyCode::CapsLock, KeyState::Up), None),
            // Letters are now caps
            (
                KeyEvent::new(KeyCode::X, KeyState::Down),
                Some(DecodedKey::Unicode('X')),
            ),
            (KeyEvent::new(KeyCode::X, KeyState::Up), None),
            // Unless you press shift
            (
                KeyEvent::new(KeyCode::RShift, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::RShift)),
            ),
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('a')),
            ),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            (KeyEvent::new(KeyCode::RShift, KeyState::Up), None),
            // Numbers are not shifted
            (
                KeyEvent::new(KeyCode::Key1, KeyState::Down),
                Some(DecodedKey::Unicode('1')),
            ),
            (KeyEvent::new(KeyCode::Key1, KeyState::Up), None),
        ];

        process_keyevents(&mut k, &test_sequence);
    }

    #[test]
    fn test_ctrl() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            // Normal
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('a')),
            ),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            // Left Control
            (
                KeyEvent::new(KeyCode::LControl, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::LControl)),
            ),
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('\u{0001}')),
            ),
            (KeyEvent::new(KeyCode::LControl, KeyState::Up), None),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            // Normal
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('a')),
            ),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
            // Right Control
            (
                KeyEvent::new(KeyCode::RControl, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::RControl)),
            ),
            (
                KeyEvent::new(KeyCode::A, KeyState::Down),
                Some(DecodedKey::Unicode('\u{0001}')),
            ),
            (KeyEvent::new(KeyCode::RControl, KeyState::Up), None),
            (KeyEvent::new(KeyCode::A, KeyState::Up), None),
        ];
        process_keyevents(&mut k, &test_sequence);
    }

    #[test]
    fn test_numlock() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        let test_sequence = [
            // Numlock ON by default so we get digits
            (
                KeyEvent::new(KeyCode::Numpad0, KeyState::Down),
                Some(DecodedKey::Unicode('0')),
            ),
            (KeyEvent::new(KeyCode::Numpad0, KeyState::Up), None),
            // Numlock OFF
            (
                KeyEvent::new(KeyCode::NumpadLock, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::NumpadLock)),
            ),
            (KeyEvent::new(KeyCode::NumpadLock, KeyState::Up), None),
            // Now KP_0 produces INSERT
            (
                KeyEvent::new(KeyCode::Numpad0, KeyState::Down),
                Some(DecodedKey::RawKey(KeyCode::Insert)),
            ),
            (KeyEvent::new(KeyCode::Numpad0, KeyState::Up), None),
        ];
        process_keyevents(&mut k, &test_sequence);
    }

    #[test]
    fn test_set_1_down_up_down() {
        let mut k = Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            (0x1e, Some(KeyEvent::new(KeyCode::A, KeyState::Down))),
            (0x9e, Some(KeyEvent::new(KeyCode::A, KeyState::Up))),
            (0x1f, Some(KeyEvent::new(KeyCode::S, KeyState::Down))),
        ];

        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_set_1_ext_down_up_down() {
        let mut k = Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            (0xe0, None),
            (
                0x1c,
                Some(KeyEvent::new(KeyCode::NumpadEnter, KeyState::Down)),
            ),
            (0xe0, None),
            (
                0x9c,
                Some(KeyEvent::new(KeyCode::NumpadEnter, KeyState::Up)),
            ),
        ];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_set_2_poweron() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [(
            0xAA,
            Some(KeyEvent::new(KeyCode::PowerOnTestOk, KeyState::SingleShot)),
        )];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_set_2_toomanykeys() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [(
            0x00,
            Some(KeyEvent::new(KeyCode::TooManyKeys, KeyState::SingleShot)),
        )];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_set_2_down_up() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            (0x29, Some(KeyEvent::new(KeyCode::Spacebar, KeyState::Down))),
            (0xF0, None),
            (0x29, Some(KeyEvent::new(KeyCode::Spacebar, KeyState::Up))),
            (0x29, Some(KeyEvent::new(KeyCode::Spacebar, KeyState::Down))),
            (0xF0, None),
            (0x29, Some(KeyEvent::new(KeyCode::Spacebar, KeyState::Up))),
            (0x29, Some(KeyEvent::new(KeyCode::Spacebar, KeyState::Down))),
            (0xF0, None),
            (0x29, Some(KeyEvent::new(KeyCode::Spacebar, KeyState::Up))),
        ];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_set_2_ext_down_up() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Us104Key,
            HandleControl::MapLettersToUnicode,
        );
        let test_sequence = [
            (0xE0, None),
            (0x6C, Some(KeyEvent::new(KeyCode::Home, KeyState::Down))),
            (0xE0, None),
            (0xF0, None),
            (0x6C, Some(KeyEvent::new(KeyCode::Home, KeyState::Up))),
        ];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_pause_set1() {
        let mut k = Keyboard::new(
            ScancodeSet1::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        // A Pause keypress generates this sequence all in one go. There is no
        // 'Break' code for this key.
        let test_sequence = [
            // rctrl2
            (0xE1, None),
            (
                0x1D,
                Some(KeyEvent {
                    code: KeyCode::RControl2,
                    state: KeyState::Down,
                }),
            ),
            // Numlock
            (
                0x45,
                Some(KeyEvent {
                    code: KeyCode::NumpadLock,
                    state: KeyState::Down,
                }),
            ),
            // Release rctrl2
            (0xE1, None),
            (
                0x9D,
                Some(KeyEvent {
                    code: KeyCode::RControl2,
                    state: KeyState::Up,
                }),
            ),
            // Release Numlock
            (
                0xC5,
                Some(KeyEvent {
                    code: KeyCode::NumpadLock,
                    state: KeyState::Up,
                }),
            ),
        ];

        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_pause_set2() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        // A Pause keypress generates this sequence all in one go. There is no
        // 'Break' code for this key.
        let test_sequence = [
            // rctrl2
            (0xE1, None),
            (
                0x14,
                Some(KeyEvent {
                    code: KeyCode::RControl2,
                    state: KeyState::Down,
                }),
            ),
            // Numlock
            (
                0x77,
                Some(KeyEvent {
                    code: KeyCode::NumpadLock,
                    state: KeyState::Down,
                }),
            ),
            // Release rctrl2
            (0xE1, None),
            (0xF0, None),
            (
                0x14,
                Some(KeyEvent {
                    code: KeyCode::RControl2,
                    state: KeyState::Up,
                }),
            ),
            // Release Numlock
            (0xF0, None),
            (
                0x77,
                Some(KeyEvent {
                    code: KeyCode::NumpadLock,
                    state: KeyState::Up,
                }),
            ),
        ];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_pause_events() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        // A Pause keypress generates this sequence all in one go. There is no
        // 'Break' code for this key.
        let test_sequence = [
            // rctrl2
            (
                KeyEvent {
                    code: KeyCode::RControl2,
                    state: KeyState::Down,
                },
                Some(DecodedKey::RawKey(KeyCode::RControl2)),
            ),
            // Numlock
            (
                KeyEvent {
                    code: KeyCode::NumpadLock,
                    state: KeyState::Down,
                },
                Some(DecodedKey::RawKey(KeyCode::PauseBreak)),
            ),
            // Release rctrl2
            (
                KeyEvent {
                    code: KeyCode::RControl2,
                    state: KeyState::Up,
                },
                None,
            ),
            // Release Numlock
            (
                KeyEvent {
                    code: KeyCode::NumpadLock,
                    state: KeyState::Up,
                },
                None,
            ),
        ];
        process_keyevents(&mut k, &test_sequence);
    }

    #[test]
    fn test_print_screen_set1() {
        let mut k = Keyboard::new(
            ScancodeSet1::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        // A Print Screen keypress generates this sequence on make and break.
        let test_sequence = [
            // ralt2
            (0xE0, None),
            (
                0x2A,
                Some(KeyEvent {
                    code: KeyCode::RAlt2,
                    state: KeyState::Down,
                }),
            ),
            // Print Screen
            (0xE0, None),
            (
                0x37,
                Some(KeyEvent {
                    code: KeyCode::PrintScreen,
                    state: KeyState::Down,
                }),
            ),
            // Release Print Screen
            (0xE0, None),
            (
                0xB7,
                Some(KeyEvent {
                    code: KeyCode::PrintScreen,
                    state: KeyState::Up,
                }),
            ),
            // Release ralt2
            (0xE0, None),
            (
                0xAA,
                Some(KeyEvent {
                    code: KeyCode::RAlt2,
                    state: KeyState::Up,
                }),
            ),
        ];
        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_print_screen_set2() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        // A Print Screen keypress generates this sequence on make and break.
        let test_sequence = [
            // ralt2
            (0xE0, None),
            (
                0x12,
                Some(KeyEvent {
                    code: KeyCode::RAlt2,
                    state: KeyState::Down,
                }),
            ),
            // Print Screen
            (0xE0, None),
            (
                0x7C,
                Some(KeyEvent {
                    code: KeyCode::PrintScreen,
                    state: KeyState::Down,
                }),
            ),
            // Release Print Screen
            (0xE0, None),
            (0xF0, None),
            (
                0x7C,
                Some(KeyEvent {
                    code: KeyCode::PrintScreen,
                    state: KeyState::Up,
                }),
            ),
            // Release ralt2
            (0xE0, None),
            (0xF0, None),
            (
                0x12,
                Some(KeyEvent {
                    code: KeyCode::RAlt2,
                    state: KeyState::Up,
                }),
            ),
        ];

        add_bytes(&mut k, &test_sequence);
    }

    #[test]
    fn test_print_screen_events() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );

        // A Print Screen keypress generates this sequence on make and break.
        let test_sequence = [
            // ralt2
            (
                KeyEvent {
                    code: KeyCode::RAlt2,
                    state: KeyState::Down,
                },
                Some(DecodedKey::RawKey(KeyCode::RAlt2)),
            ),
            // Print Screen
            (
                KeyEvent {
                    code: KeyCode::PrintScreen,
                    state: KeyState::Down,
                },
                Some(DecodedKey::RawKey(KeyCode::PrintScreen)),
            ),
            // Release Print Screen
            (
                KeyEvent {
                    code: KeyCode::PrintScreen,
                    state: KeyState::Up,
                },
                None,
            ),
            // Release ralt2
            (
                KeyEvent {
                    code: KeyCode::RAlt2,
                    state: KeyState::Up,
                },
                None,
            ),
        ];

        process_keyevents(&mut k, &test_sequence);
    }

    #[test]
    fn test_modifier_state_shift() {
        let mut k = Keyboard::new(
            ScancodeSet2::new(),
            layouts::Uk105Key,
            HandleControl::MapLettersToUnicode,
        );
        assert!(!k.get_modifiers().lshift);

        k.process_keyevent(KeyEvent {
            code: KeyCode::LShift,
            state: KeyState::Down,
        });
        assert!(k.get_modifiers().lshift);

        k.process_keyevent(KeyEvent {
            code: KeyCode::LShift,
            state: KeyState::Up,
        });
        assert!(!k.get_modifiers().lshift);
    }
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
