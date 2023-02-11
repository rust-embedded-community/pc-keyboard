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
| [`Us104`](./src/layouts/us104.rs)                    | 101/104  | North American standard English                                          | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#United_States)                     |
| [`Uk105`](./src/layouts/uk105.rs)                    | 102/105  | United Kingdom standard English                                          | [Wikipedia](https://en.wikipedia.org/wiki/QWERTY#United_Kingdom)                    |
| [`Azerty`](./src/layouts/azerty.rs)                  | 102/105  | Typically used in French locales                                         | [Wikipedia](https://en.wikipedia.org/wiki/AZERTY)                                   |
| [`De104`](./src/layouts/de104.rs)                    | 102/105  | German layout                                                            | [Wikipedia](https://en.wikipedia.org/wiki/QWERTZ)                                   |
| [`Jis109`](./src/layouts/jis109.rs)                  | 106/109  | JIS 109-key layout (Latin chars only)                                    | [Wikipedia](https://en.wikipedia.org/wiki/Japanese_input_method#Japanese_keyboards) |
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
Set 2. They represented by the `KeyCode` enum.

This is the mapping of `KeyCode` to a 105-key ISO keyboard:

```text
+----+  +----+----+----+----+  +----+----+----+----+  +----+----+----+----+   +----+----+----+
|Esc |  | F1 | F2 | F3 | F4 |  | F5 | F6 | F7 | F8 |  | F9 |F10 |F11 |F12 |   |PrSc|Scrl|PBrk|
+----+  +----+----+----+----+  +----+----+----+----+  +----+----+----+----+   +----+----+----+

+----+----+----+----+----+----+----+----+----+----+----+----+----+---------+  +----+----+----+  +----+----+----+----+
|Oem8|Key1|Key2|Key3|Key4|Key5|Key6|Key7|Key8|Key9|Key0|Oem-|Oem+|Backspace|  |Inse|Home|PgUp|  |NumL|Num/|Num*|Num-|
+----++---++---++---++---++---++---++---++---++---++---++---++---++--------+  +----+----+----+  +----+----+----+----+
|Tab  | Q  | W  | E  | R  | T  | Y  | U  | I  | O  | P  |Oem4|Oem6| Enter  |  |Dele|End |PgDo|  |Num7|Num8|Num9|    |
+-----++---++---++---++---++---++---++---++---++---++---++---++---++       |  +----+----+----+  +----+----+----+Num+|
|CapsLo| A  | S  | D  | F  | G  | H  | J  | K  | L  |Oem1|Oem3|Oem7|       |                    |Num4|Num5|Num6|    |
+----+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+-------+       +----+       +----+----+----+----+
|LShf|Oem5| Z  | X  | C  | V  | B  | N  | M  |OemC|OemP|Oem2|   RShift     |       | Up |       |Num1|Num2|Num3|    |
+----++---+-+--+--+-+----+----+----+----+----+---++----+----++------+------+  +----+----+----+  +----+----+----+Num |
|LCtrl|LWin | Alt |       Space                  |AltGr|RWin | Menu |RCtrl |  |Left|Down|Righ|  |Num0     |NumP|Ente|
+-----+-----+-----+------------------------------+-----+-----+------+------+  +----+----+----+  +---------+----+----+
```

This is the mapping of `KeyCode` to a 104-key ANSI keyboard: 

```text
+----+  +----+----+----+----+  +----+----+----+----+  +----+----+----+----+   +----+----+----+
|Esc |  | F1 | F2 | F3 | F4 |  | F5 | F6 | F7 | F8 |  | F9 |F10 |F11 |F12 |   |PrSc|Scrl|PBrk|
+----+  +----+----+----+----+  +----+----+----+----+  +----+----+----+----+   +----+----+----+

+----+----+----+----+----+----+----+----+----+----+----+----+----+---------+  +----+----+----+  +----+----+----+----+
|Oem8|Key1|Key2|Key3|Key4|Key5|Key6|Key7|Key8|Key9|Key0|Oem-|Oem+|Backspace|  |Inse|Home|PgUp|  |NumL|Num/|Num*|Num-|
+----++---++---++---++---++---++---++---++---++---++---++---++---++--------+  +----+----+----+  +----+----+----+----+
|Tab  | Q  | W  | E  | R  | T  | Y  | U  | I  | O  | P  |Oem4|Oem6| Oem5   |  |Dele|End |PgDo|  |Num7|Num8|Num9|    |
+-----++---++---++---++---++---++---++---++---++---++---++---++---+--------+  +----+----+----+  +----+----+----+Num+|
|CapsLo| A  | S  | D  | F  | G  | H  | J  | K  | L  |Oem1|Oem3| Enter      |                    |Num4|Num5|Num6|    |
+------+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+--+-+------------+       +----+       +----+----+----+----+
| LShift  | Z  | X  | C  | V  | B  | N  | M  |OemC|OemP|Oem2|   RShift     |       | Up |       |Num1|Num2|Num3|    |
+-----+---+-+--+--+-+----+----+----+----+----+---++----+----++------+------+  +----+----+----+  +----+----+----+Num |
|LCtrl|LWin | Alt |       Space                  |AltGr|RWin | Menu |RCtrl |  |Left|Down|Righ|  |Num0     |NumP|Ente|
+-----+-----+-----+------------------------------+-----+-----+------+------+  +----+----+----+  +---------+----+----+
```

Note that the `Oem7` key is missing on the 104-key ANSI layout.

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
