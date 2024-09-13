# pc-keyboard

A simple driver for handling PC keyboards, with both Scancode Set 1 (when
running on a PC) and Scancode Set 2 support (when reading a PS/2 keyboard
output directly).

## Supports:

-   Scancode Set 1 (from the i8042 PC keyboard controller)
-   Scancode Set 2 (direct from the AT or PS/2 interface keyboard)
-   Several keyboard layouts:

| Name                                                 | No. Keys | Description                                                              | Link                                                                                |
| ---------------------------------------------------- | -------- | ------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| [`Us104Key`](./src/layouts/us104.rs)                 | 101/104  | North American standard English                                          | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#United_States)                     |
| [`Uk105Key`](./src/layouts/uk105.rs)                 | 102/105  | United Kingdom standard English                                          | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#United_Kingdom)                    |
| [`Azerty`](./src/layouts/azerty.rs)                  | 102/105  | Typically used in French locales                                         | [Wikipedia](https://en.wikipedia.org/wiki/AZERTY)                                   |
| [`De105Key`](./src/layouts/de105.rs)                 | 102/105  | German layout                                                            | [Wikipedia](https://en.wikipedia.org/wiki/QWERTZ)                                   |
| [`FiSe105Key`](./src/layouts/fi_se105.rs)            | 102/105  | Finnish/Swedish layout                                                   | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#Finnish%E2%80%93Swedish)           |
| [`No105Key`](./src/layouts/no105.rs)                 | 102/105  | Norwegian layout                                                         | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#Norwegian)                         |
| [`Jis109Key`](./src/layouts/jis109.rs)               | 106/109  | JIS 109-key layout (Latin chars only)                                    | [Wikipedia](https://en.wikipedia.org/wiki/Japanese_input_method#Japanese_keyboards) |
| [`Colemak`](./src/layouts/colemak.rs)                | 101/104  | A keyboard layout designed to make typing more efficient and comfortable | [Wikipedia](https://en.wikipedia.org/wiki/Colemak)                                  |
| [`Dvorak104Key`](./src/layouts/dvorak104.rs)         | 101/104  | The more 'ergonomic' alternative to QWERTY                               | [Wikipedia](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout)                   |
| [`DVP104Key`](./src/layouts/dvorak_programmer104.rs) | 101/104  | Dvorak for Programmers                                                   | [Wikipedia](https://en.wikipedia.org/wiki/Dvorak_keyboard_layout#Programmer_Dvorak) |

101/104 keys is ANSI layout (wide Enter key) and 102/105 keys is ISO layout
(tall Enter key). The difference between 101 and 104 (and between 102 and
105) comes from the two Windows keys and the Menu key that were added when
Windows 95 came out. JIS keyboards have extra keys, added by making the
space-bar and backspace keys shorter.


## Usage

There are three basic steps to handling keyboard input. Your application may bypass some of these.

* `Ps2Decoder` - converts 11-bit PS/2 words into bytes, removing the start/stop
  bits and checking the parity bits. Only needed if you talk to the PS/2
  keyboard over GPIO pins and not required if you talk to the i8042 PC keyboard
  controller.
* `ScancodeSet` - converts from Scancode Set 1 (i8042 PC keyboard controller) or
  Scancode Set 2 (raw PS/2 keyboard output) into a symbolic `KeyCode` and an
  up/down `KeyState`.
* `EventDecoder` - converts symbolic `KeyCode` and `KeyState` into a Unicode
  characters (where possible) according to the currently selected `KeyboardLayout`.

There is also `Keyboard` which combines the above three functions into a single object.

See the [`examples`](./examples) folder for more details.

## [Documentation](https://docs.rs/crate/pc-keyboard)

## Keycodes

This crate uses symbolic keycodes to abstract over Scancode Set 1 and Scancode
Set 2. They represented by the `KeyCode` enum. The scancodes can come from one of three supported keyboards: 102/105 key ISO, 101/104 key ANSI and 106/109-key JIS.

### 102/105 key ISO

This is the mapping of `KeyCode` to a 102/105-key ISO keyboard:

```text
┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
│Esc │  │ F1 │ F2 │ F3 │ F4 │  │ F5 │ F6 │ F7 │ F8 │  │ F9 │F10 │F11 │F12 │   │PrSc│Scrl│PBrk│
└────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘

┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
│Oem8│Key1│Key2│Key3│Key4│Key5│Key6│Key7│Key8│Key9│Key0│Oem─│Oem+│Backspace│  │Inse│Home│PgUp│  │NumL│Num/│Num*│Num─│
├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
│ Tab │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │Oem4│Oem6│ Enter  │  │Dele│End │PgDo│  │Num7│Num8│Num9│    │
├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┐       │  └────┴────┴────┘  ├────┼────┼────┤Num+│
│CapsLo│ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │Oem1│Oem3│Oem7│       │                    │Num4│Num5│Num6│    │
├────┬─┴───┬┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────┴───────┤       ┌────┐       ├────┼────┼────┼────┤
│LShf│Oem5 │ Z │ X  │ C  │ V  │ B  │ N  │ M  │OemC│OemP│Oem2│   RShift     │       │ Up │       │Num1│Num2│Num3│    │
├────┴┬────┴┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┴────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤Num │
│LCtrl│LWin │ Alt │       Space                  │AltGr│RWin │ Menu │RCtrl │  │Left│Down│Righ│  │Num0     │NumP│Ente│
└─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
```

The 102-key is missing `LWin`, `RWin`, and `Menu`.

(Reference: <https://kbdlayout.info/KBDUK/scancodes+virtualkeys?arrangement=ISO105>)

### 101/104 key ANSI

This is the mapping of `KeyCode` to a 101/104-key ANSI keyboard:

```text
┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
│Esc │  │ F1 │ F2 │ F3 │ F4 │  │ F5 │ F6 │ F7 │ F8 │  │ F9 │F10 │F11 │F12 │   │PrSc│Scrl│PBrk│
└────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘

┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬─────────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
│Oem8│Key1│Key2│Key3│Key4│Key5│Key6│Key7│Key8│Key9│Key0│Oem─│Oem+│Backspace│  │Inse│Home│PgUp│  │NumL│Num/│Num*│Num─│
├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────+────+────┤  ├────┼────┼────┼────┤
│ Tab │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │Oem4│Oem6│  Oem7  │  │Dele│End │PgDo│  │Num7│Num8│Num9│    │
├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴────────┤  └────┴────┴────┘  ├────┼────┼────┤Num+│
│CapsLo│ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │Oem1│Oem3│   Enter    │                    │Num4│Num5│Num6│    │
├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────────────┤       ┌────┐       ├────┼────┼────┼────┤
│ LShift  │ Z  │ X  │ C  │ V  │ B  │ N  │ M  │OemC│OemP│Oem2│   RShift     │       │ Up │       │Num1│Num2│Num3│    │
├─────┬───┴─┬──┴──┬─┴────┴────┴────┴────┴────┴───┬┴────┴────┴┬──────┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤Num │
│LCtrl│LWin │ Alt │       Space                  │AltGr│RWin │ Menu │RCtrl │  │Left│Down│Righ│  │Num0     │NumP│Ente│
└─────┴─────┴─────┴──────────────────────────────┴─────┴─────┴──────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
```

Note that the `Oem5` key is missing on the 104-key ANSI keyboard.

The 101-key is also missing `LWin`, `RWin`, and `Menu`.

(Reference: <https://kbdlayout.info/KBDUK/scancodes+virtualkeys?arrangement=ANSI104>)

### 106/109 key JIS

This is the mapping of `KeyCode` to a 106/109-key JIS keyboard:

```text
┌────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐  ┌────┬────┬────┬────┐   ┌────┬────┬────┐
│Esc │  │ F1 │ F2 │ F3 │ F4 │  │ F5 │ F6 │ F7 │ F8 │  │ F9 │F10 │F11 │F12 │   │PrSc│Scrl│PBrk│
└────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘  └────┴────┴────┴────┘   └────┴────┴────┘

┌────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┬────┐  ┌────┬────┬────┐  ┌────┬────┬────┬────┐
│Oem8│Key1│Key2│Key3│Key4│Key5│Key6│Key7│Key8│Key9│Key0│Oem─│Oem+│Om13│BkSp│  │Inse│Home│PgUp│  │NumL│Num/│Num*│Num─│
├────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬────────┤  ├────┼────┼────┤  ├────┼────┼────┼────┤
│ Tab │ Q  │ W  │ E  │ R  │ T  │ Y  │ U  │ I  │ O  │ P  │Oem4│Oem6│ Enter  │  │Dele│End │PgDo│  │Num7│Num8│Num9│    │
├─────┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┬───┴┐       │  └────┴────┴────┘  ├────┼────┼────┤Num+│
│CapsLo│ A  │ S  │ D  │ F  │ G  │ H  │ J  │ K  │ L  │Oem1│Oem3│Oem7│       │                    │Num4│Num5│Num6│    │
├──────┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴──┬─┴────┴───────┤       ┌────┐       ├────┼────┼────┼────┤
│LShift   │ Z  │ X  │ C  │ V  │ B  │ N  │ M  │OemC│OemP│Oem2│Oem12 │RShift │       │ Up │       │Num1│Num2│Num3│    │
├─────┬───┴─┬──┴──┬─┴───┬┴────┴────┴────┴────┴┬───┴─┬──┴──┬─┴──┬───┴┬──────┤  ┌────┼────┼────┐  ├────┴────┼────┤Num │
│LCtrl│LWin │LAlt │Oem9 │ Space Bar           │Oem10│Oem11│RWin│Menu│RCtrl │  │Left│Down│Righ│  │Num0     │NumP│Ente│
└─────┴─────┴─────┴─────┴─────────────────────┴─────┴─────┴────┴────┴──────┘  └────┴────┴────┘  └─────────┴────┴────┘
```

Note that the `Oem5` is missing on the 109-key JIS layout, but `Oem9` (Muhenkan), `Oem10` (Henkan/Zenkouho), `Oem11` (Hiragana/Katakana), `Oem12` (Backslash) and `Oem13` (¥) are added.

The 106-key is missing `LWin`, `RWin`, and `Menu`.

(Reference: <https://kbdlayout.info/KBDUK/scancodes+virtualkeys?arrangement=OADG109A>)

### Conversion Table

Scancode Set 1 and Scancode Set 2 can be losslessly converted. Indeed, this is
what the i8042 keyboard controller in your PC does - it takes Scancode Set 2
from the keyboard and provides Scancode Set 1 to the Operating System. This
allowed them to change the keyboard design without breaking compatibility with
any MS-DOS applications that read raw scancodes from the keyboard.

This table shows the correspondence between our symbolic KeyCode, Scancode Set 1
and Scancode Set 2. We may extend this in the future to also handle USB HID
Scancodes. Any codes prefixed `0xE0` or `0xE1` are *extended* multi-byte
scancodes. Typically these are keys that were not on the IBM PC and PC/XT
keyboards so they they were added in such a way that if you ignored the 0xE0,
you got a reasonable result anyway. For example `ArrowLeft` is `0xE04B` in
Scancode Set 1 because `Numpad4` is `0x4B` and that was the left-arrow key on an
IBM PC or PC/XT.

| Symbolic Key   | Scancode Set 1 | Scancode Set 2 |
| -------------- | -------------- | -------------- |
| Escape         | 0x01           | 0x76           |
| F1             | 0x3B           | 0x05           |
| F2             | 0x3C           | 0x06           |
| F3             | 0x3D           | 0x04           |
| F4             | 0x3E           | 0x0C           |
| F5             | 0x3F           | 0x03           |
| F6             | 0x40           | 0x0B           |
| F7             | 0x41           | 0x83           |
| F8             | 0x42           | 0x0A           |
| F9             | 0x43           | 0x01           |
| F10            | 0x44           | 0x09           |
| F11            | 0x57           | 0x78           |
| F12            | 0x58           | 0x07           |
| PrintScreen    | 0xE037         | 0xE07C         |
| SysRq          | 0x54           | 0x7F           |
| ScrollLock     | 0x46           | 0x7E           |
| PauseBreak     | --             | --             |
| -              | --             | --             |
| Oem8           | 0x29           | 0x0E           |
| Key1           | 0x02           | 0x16           |
| Key2           | 0x03           | 0x1E           |
| Key3           | 0x04           | 0x26           |
| Key4           | 0x05           | 0x25           |
| Key5           | 0x06           | 0x2E           |
| Key6           | 0x07           | 0x36           |
| Key7           | 0x08           | 0x3D           |
| Key8           | 0x09           | 0x3E           |
| Key9           | 0x0A           | 0x46           |
| Key0           | 0x0B           | 0x45           |
| OemMinus       | 0x0C           | 0x4E           |
| OemPlus        | 0x0D           | 0x55           |
| Backspace      | 0x0E           | 0x66           |
| Insert         | 0xE052         | 0xE070         |
| Home           | 0xE047         | 0xE06C         |
| PageUp         | 0xE049         | 0xE07D         |
| NumpadLock     | 0x45           | 0x77           |
| NumpadDivide   | 0xE035         | 0xE04A         |
| NumpadMultiply | 0x37           | 0x7C           |
| NumpadSubtract | 0x4A           | 0x7B           |
| -              | --             | --             |
| Tab            | 0x0F           | 0x0D           |
| Q              | 0x10           | 0x15           |
| W              | 0x11           | 0x1D           |
| E              | 0x12           | 0x24           |
| R              | 0x13           | 0x2D           |
| T              | 0x14           | 0x2C           |
| Y              | 0x15           | 0x35           |
| U              | 0x16           | 0x3C           |
| I              | 0x17           | 0x43           |
| O              | 0x18           | 0x44           |
| P              | 0x19           | 0x4D           |
| Oem4           | 0x1A           | 0x54           |
| Oem6           | 0x1B           | 0x5B           |
| Oem5           | 0x56           | 0x61           |
| Oem7           | 0x2B           | 0x5D           |
| Delete         | 0xE053         | 0xE071         |
| End            | 0xE04F         | 0xE069         |
| PageDown       | 0xE051         | 0xE07A         |
| Numpad7        | 0x47           | 0x6C           |
| Numpad8        | 0x48           | 0x75           |
| Numpad9        | 0x49           | 0x7D           |
| NumpadAdd      | 0x4E           | 0x79           |
| -              | --             | --             |
| CapsLock       | 0x3A           | 0x58           |
| A              | 0x1E           | 0x1C           |
| S              | 0x1F           | 0x1B           |
| D              | 0x20           | 0x23           |
| F              | 0x21           | 0x2B           |
| G              | 0x22           | 0x34           |
| H              | 0x23           | 0x33           |
| J              | 0x24           | 0x3B           |
| K              | 0x25           | 0x42           |
| L              | 0x26           | 0x4B           |
| Oem1           | 0x27           | 0x4C           |
| Oem3           | 0x28           | 0x52           |
| Return         | 0x1C           | 0x5A           |
| Numpad4        | 0x4B           | 0x6B           |
| Numpad5        | 0x4C           | 0x73           |
| Numpad6        | 0x4D           | 0x74           |
| -              | --             | --             |
| LShift         | 0x2A           | 0x12           |
| Z              | 0x2C           | 0x1A           |
| X              | 0x2D           | 0x22           |
| C              | 0x2E           | 0x21           |
| V              | 0x2F           | 0x2A           |
| B              | 0x30           | 0x32           |
| N              | 0x31           | 0x31           |
| M              | 0x32           | 0x3A           |
| OemComma       | 0x33           | 0x41           |
| OemPeriod      | 0x34           | 0x49           |
| Oem2           | 0x35           | 0x4A           |
| RShift         | 0x36           | 0x59           |
| ArrowUp        | 0xE048         | 0xE075         |
| Numpad1        | 0x4F           | 0x69           |
| Numpad2        | 0x50           | 0x72           |
| Numpad3        | 0x51           | 0x7A           |
| NumpadEnter    | 0xE01C         | 0xE075         |
| -              | --             | --             |
| LControl       | 0x1D           | 0x14           |
| LWin           | 0xE05B         | 0xE01F         |
| LAlt           | 0x38           | 0x11           |
| Spacebar       | 0x39           | 0x29           |
| RAltGr         | 0xE038         | 0xE011         |
| RWin           | 0xE05C         | 0xE027         |
| Apps           | 0xE05C         | 0xE02F         |
| RControl       | 0xE01D         | 0xE014         |
| ArrowLeft      | 0xE04B         | 0xE06B         |
| ArrowDown      | 0xE050         | 0xE072         |
| ArrowRight     | 0xE04D         | 0xE074         |
| Numpad0        | 0x52           | 0x70           |
| NumpadPeriod   | 0x53           | 0x71           |
| -              | --             | --             |
| Oem9           | 0x7B           | 0x67           |
| Oem10          | 0x79           | 0x64           |
| Oem11          | 0x70           | 0x13           |
| Oem12          | 0x73           | 0x51           |
| Oem13          | 0x7D           | 0x6A           |
| -              | --             | --             |
| PrevTrack      | 0xE010         | 0xE015         |
| NextTrack      | 0xE019         | 0xE04D         |
| Mute           | 0xE020         | 0xE023         |
| Calculator     | 0xE021         | 0xE02B         |
| Play           | 0xE022         | 0xE034         |
| Stop           | 0xE024         | 0xE03B         |
| VolumeDown     | 0xE02E         | 0xE021         |
| VolumeUp       | 0xE030         | 0xE032         |
| WWWHome        | 0xE032         | 0xE03A         |
| TooManyKeys    | --             | 0x00           |
| PowerOnTestOk  | --             | 0xAA           |
| RControl2      | 0xE11D         | 0xE114         |
| RAlt2          | 0xE02A         | 0xE012         |

__Note 1:__ `PauseBreak` does not have a scancode because it's something we infer from a
sequence of other keypresses (`NumLock` with `RControl2` held).

__Note 2:__ `SysReq` doesn't have a key on the diagram, because the scancode is
only generated when you do `Alt` + `PrintScreen`.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.61 and up. It might compile with older versions but that may change in any new patch release.

## Changelog

There is a changelog in [CHANGELOG.md](./CHANGELOG.md).

## License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above, without
any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][coc], the maintainer of this crate, the [Rust Embedded Community][team], promises
to intervene to uphold that code of conduct.

[coc]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/orgs/rust-embedded-community/people
